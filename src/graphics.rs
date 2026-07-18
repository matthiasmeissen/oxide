use crate::state::*;
use crate::screens::draw;
use crate::shaders::ShaderLibrary;

use miniquad::*;
use std::sync::Arc;
use std::{fs, time::Instant};
use crossbeam_channel::Sender;
use triple_buffer::Output;
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
};

const VERTEX: &str = r#"
    #version 100
    precision mediump float;

    attribute vec2 in_pos;
    attribute vec2 in_uv;

    varying mediump vec2 v_uv;

    void main() {
        v_uv = in_uv;
        gl_Position = vec4(in_pos, 0, 1);
    }
"#;

const OLED_FRAGMENT_SHADER: &str = r#"
    #version 100
    precision mediump float;

    varying vec2 v_uv;
    uniform sampler2D u_oled_texture;

    void main() {
        // Sample the color from the texture. Flip the y-coordinate 
        // to correctly orient the display.
        gl_FragColor = texture2D(u_oled_texture, vec2(v_uv.x, 1.0 - v_uv.y));
    }
"#;

pub fn start_graphics_thread(
    window_sender: Sender<Message>,
    window_reader: Output<State>,
    display_reader: Output<DisplayState>,
    shaders: Arc<ShaderLibrary>,
) {
    let conf = conf::Conf {
        window_title: String::from("Window Title"),
        high_dpi: true,
        window_width: 960,
        window_height: 540,
        fullscreen: true,
        ..Default::default()
    };

    start(conf, move || Box::new(Stage::new(window_sender, window_reader, display_reader, shaders)));
}

struct DisplayBuffer {
    buffer: [u8; 128 * 64 / 8],
    rgba_cache: Vec<u8>,
    width: usize,
    height: usize,
}

impl DisplayBuffer {
    fn new() -> Self {
        Self {
            buffer: [0u8; 128 * 64 / 8],
            rgba_cache: vec![0u8; 128 * 64 * 4],
            width: 128,
            height: 64,
        }
    }

    fn clear(&mut self) {
        self.buffer.fill(0);
    }

    fn to_rgba(&mut self) -> &[u8] {
        const ON_R: u8 = 255;
        const ON_G: u8 = 0;
        const ON_B: u8 = 0;
        const OFF_R: u8 = 10;
        const OFF_G: u8 = 10;
        const OFF_B: u8 = 10;
        
        for y in 0..self.height {
            let row_base = y * self.width;
            for x in 0..self.width {
                let byte_idx = (y / 8) * self.width + x;
                let bit_idx = y % 8;
                let pixel_on = (self.buffer[byte_idx] >> bit_idx) & 1 == 1;
                
                let rgba_idx = (row_base + x) * 4;
                
                let (r, g, b) = if pixel_on { 
                    (ON_R, ON_G, ON_B) 
                } else { 
                    (OFF_R, OFF_G, OFF_B) 
                };
                
                self.rgba_cache[rgba_idx] = r;
                self.rgba_cache[rgba_idx + 1] = g;
                self.rgba_cache[rgba_idx + 2] = b;
                self.rgba_cache[rgba_idx + 3] = 255;
            }
        }
        
        &self.rgba_cache
    }
}

impl DrawTarget for DisplayBuffer {
    type Color = BinaryColor;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            let x = coord.x as usize;
            let y = coord.y as usize;
            
            if x < self.width && y < self.height {
                let byte_idx = (y / 8) * self.width + x;
                let bit_idx = y % 8;
                
                if color.is_on() {
                    self.buffer[byte_idx] |= 1 << bit_idx;
                } else {
                    self.buffer[byte_idx] &= !(1 << bit_idx);
                }
            }
        }
        Ok(())
    }
}

impl OriginDimensions for DisplayBuffer {
    fn size(&self) -> Size {
        Size::new(self.width as u32, self.height as u32)
    }
}

struct Stage {
    main_pipeline: Pipeline,
    main_bindings: Bindings,

    oled_pipeline: Pipeline,
    oled_bindings: Bindings,

    ctx: Box<dyn RenderingBackend>,
    start_time: std::time::Instant,
    sender: Sender<Message>,
    reader: Output<State>,

    display_reader: Output<DisplayState>,
    display_buffer: DisplayBuffer,
    display_texture: TextureId,

    mq_resolution: [f32; 2],
    is_fullscreen: bool,
    shaders: Arc<ShaderLibrary>,
    current_shader_index: usize,
    last_fps_update: Instant,
    frames_since_update: u32,
    display_update_counter: u32,
    show_oled_preview: bool,
}

impl Stage {
    fn new(sender: Sender<Message>, reader: Output<State>, mut display_reader: Output<DisplayState>, shaders: Arc<ShaderLibrary>) -> Self {
        let mut ctx = window::new_rendering_backend();

        // =======================================================================
        // SETUP MAIN QUAD
        // =======================================================================
        let vertices: [Vertex; 4] = [
            Vertex {pos: [-1.0, -1.0], uv: [0.0, 0.0]},
            Vertex {pos: [1.0, -1.0], uv: [1.0, 0.0]},
            Vertex {pos: [1.0, 1.0], uv: [1.0, 1.0]},
            Vertex {pos: [-1.0, 1.0], uv: [0.0, 1.0]},
        ];
        let main_vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer, BufferUsage::Immutable, BufferSource::slice(&vertices)
        );
        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let main_index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer, BufferUsage::Immutable, BufferSource::slice(&indices)
        );

        // =======================================================================
        // 2. SETUP OLED TEXTURE
        // =======================================================================
        let mut display_buffer = DisplayBuffer::new();
        let display_state = display_reader.read();
        display_buffer.clear();

        draw(&mut display_buffer, &display_state, &shaders);
        let initial_rgba = display_buffer.to_rgba();
        let display_texture = ctx.new_texture_from_rgba8(128, 64, initial_rgba);
        ctx.texture_set_filter(display_texture, FilterMode::Nearest, MipmapFilterMode::None);

        // =======================================================================
        // 3. SETUP MAIN PIPELINE
        // =======================================================================
        let current_shader_index = 0;
        let initial_shader_path = shaders
            .path(current_shader_index)
            .expect("Shader library is empty")
            .to_path_buf();
        let initial_shader_source = std::fs::read_to_string(&initial_shader_path)
            .expect("Error reading the initial shader");

        let main_shader = ctx.new_shader(
            ShaderSource::Glsl { vertex: VERTEX, fragment: &initial_shader_source },
            main_shader_meta()
        ).expect("Main shader compilation failed");

        // Main bindings no longer include the texture!
        let main_bindings = Bindings {
            vertex_buffers: vec![main_vertex_buffer],
            index_buffer: main_index_buffer,
            images: vec![] // <-- Texture removed
        };

        let main_pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[VertexAttribute::new("in_pos", VertexFormat::Float2), VertexAttribute::new("in_uv", VertexFormat::Float2)],
            main_shader, PipelineParams::default()
        );

        // =======================================================================
        // 4. SETUP OLED PIPELINE
        // =======================================================================
        let oled_vertices: [Vertex; 4] = [
            Vertex { pos: [0.35, -0.95],      uv: [0.0, 0.0] }, // Bottom-left
            Vertex { pos: [0.95, -0.95],      uv: [1.0, 0.0] }, // Bottom-right
            Vertex { pos: [0.95, -0.4166667], uv: [1.0, 1.0] }, // Top-right
            Vertex { pos: [0.35, -0.4166667], uv: [0.0, 1.0] }, // Top-left
        ];
        let oled_vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer, BufferUsage::Immutable, BufferSource::slice(&oled_vertices)
        );
        let oled_index_buffer = ctx.new_buffer( // Can reuse the same index data
            BufferType::IndexBuffer, BufferUsage::Immutable, BufferSource::slice(&indices)
        );
        
        // Bind the oled geometry and the SHARED display_texture
        let oled_bindings = Bindings {
            vertex_buffers: vec![oled_vertex_buffer],
            index_buffer: oled_index_buffer,
            images: vec![display_texture],
        };

        let oled_shader = ctx.new_shader(
            ShaderSource::Glsl { vertex: VERTEX, fragment: OLED_FRAGMENT_SHADER },
            oled_shader_meta()
        ).expect("OLED shader compilation failed");

        let oled_pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[VertexAttribute::new("in_pos", VertexFormat::Float2), VertexAttribute::new("in_uv", VertexFormat::Float2)],
            oled_shader,
            PipelineParams::default()
        );

        // Set window resolution
        let (width, height) = window::screen_size();
        sender.try_send(Message::SetResolution(width, height)).ok();

        Self { 
            main_pipeline,
            main_bindings,

            oled_pipeline,
            oled_bindings,

            ctx,
            start_time: Instant::now(),
            sender,
            reader,
            display_reader,
            display_buffer,
            display_texture,
            mq_resolution: [width, height],
            is_fullscreen: true,
            shaders,
            current_shader_index,
            last_fps_update: Instant::now(),
            frames_since_update: 0,
            display_update_counter: 0,
            show_oled_preview: true,
        }
    }

    #[inline]
    fn update_display_texture(&mut self) {
        let display_state = self.display_reader.read();
        
        self.display_buffer.clear();
        draw(&mut self.display_buffer, &display_state, &self.shaders);

        let rgba_data = self.display_buffer.to_rgba();
        self.ctx.texture_update(self.display_texture, rgba_data);
    }

    fn set_shader(&mut self, new_index: usize) {
        let num_shaders = self.shaders.len();
        if num_shaders == 0 {
            return;
        }

        let wrapped_index = new_index % num_shaders;
        if wrapped_index == self.current_shader_index {
            return;
        }

        // Acknowledge the request up front so a failed compile does not make
        // `update()` retry the same broken shader every frame. On failure the
        // previous pipeline is kept, so the last working visual stays on screen.
        self.current_shader_index = wrapped_index;

        let new_shader_path = match self.shaders.path(wrapped_index) {
            Some(path) => path.to_path_buf(),
            None => return,
        };

        match fs::read_to_string(&new_shader_path) {
            Ok(fragment_source) => {
                match self.ctx.new_shader(
                    ShaderSource::Glsl { vertex: VERTEX, fragment: &fragment_source },
                    main_shader_meta()
                ) {
                    Ok(new_shader) => {
                        let new_pipeline = self.ctx.new_pipeline(
                            &[BufferLayout::default()],
                            &[
                                VertexAttribute::new("in_pos", VertexFormat::Float2),
                                VertexAttribute::new("in_uv", VertexFormat::Float2)
                            ],
                            new_shader,
                            PipelineParams::default()
                        );

                        self.main_pipeline = new_pipeline;
                        println!("Successfully swapped to shader: {}", new_shader_path.display());
                    }
                    Err(err) => {
                        eprintln!("Failed to compile shader '{}': {:?}", new_shader_path.display(), err);
                    }
                }
            }
            Err(err) => {
                eprintln!("Failed to load shader file '{}': {}", new_shader_path.display(), err);
            }
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {
        self.frames_since_update += 1;
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_fps_update).as_secs_f64();

        if elapsed >= 1.0 {
            let fps = self.frames_since_update as f32 / elapsed as f32;
            self.sender.try_send(Message::SetFps(fps)).ok();
            self.frames_since_update = 0;
            self.last_fps_update = now;
        }

        let new_shader_index = self.reader.read().shader_index;
        
        if new_shader_index != self.current_shader_index {
            self.set_shader(new_shader_index);
        }

        // OPTIMIZED: Update display texture every 2 frames (30fps for 60fps main)
        // Adjust divisor based on your needs: 1=60fps, 2=30fps, 3=20fps, 4=15fps
        if self.show_oled_preview {
            self.display_update_counter += 1;
            if self.display_update_counter >= 4 {
                self.update_display_texture();
                self.display_update_counter = 0;
            }
        }
    }

    #[inline]
    fn draw(&mut self) {
        self.ctx.begin_default_pass(PassAction::Clear {
            color: Some((0.0, 0.0, 0.0, 1.0)),
            depth: None,
            stencil: None,
        });
        self.ctx.apply_pipeline(&self.main_pipeline);
        self.ctx.apply_bindings(&self.main_bindings);

        let time = self.start_time.elapsed().as_secs_f64();
        self.sender.try_send(Message::SetTime(time)).ok();

        let state = self.reader.read();

        let uniforms = Uniforms { 
            u_time: state.time as f32,
            u_resolution: state.resolution,
            u_cv1: state.values[0] as f32,
            u_cv2: state.values[1] as f32,
            u_cv3: state.values[2] as f32,
            u_cv4: state.values[3] as f32,
            u_gate1: state.values[4] as f32,
            u_gate2: state.values[5] as f32,
            u_gate3: state.values[6] as f32,
            u_gate4: state.values[7] as f32,
        };
        self.ctx.apply_uniforms(UniformsSource::table(&uniforms));

        self.ctx.draw(0, 6, 1);

        if self.show_oled_preview {
            self.ctx.apply_pipeline(&self.oled_pipeline);
            self.ctx.apply_bindings(&self.oled_bindings);
            self.ctx.draw(0, 6, 1);
        }

        self.ctx.end_render_pass();
        self.ctx.commit_frame();
    }

    fn resize_event(&mut self, width: f32, height: f32) {
        self.mq_resolution = [width, height];
        self.sender.try_send(Message::SetResolution(width, height)).ok();
        println!("Set resolution to: {width}, {height}");
    }

    #[inline]
    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        let (width, height) = miniquad::window::screen_size();
        
        let norm_x = (x / width).clamp(0.0, 1.0);
        let norm_y = (y / height).clamp(0.0, 1.0);

        if norm_x >= 0.0 && norm_x <= 1.0 && norm_y >= 0.0 && norm_y <= 1.0 {
            let u1 = norm_x;
            let u2 = norm_y;
            let u3 = 1.0 - norm_x;
            let u4 = 1.0 - norm_y;

            self.sender.try_send(Message::SetValue(0, u1)).ok();
            self.sender.try_send(Message::SetValue(1, u2)).ok();
            self.sender.try_send(Message::SetValue(2, u3)).ok();
            self.sender.try_send(Message::SetValue(3, u4)).ok();
        }
    }

    fn key_down_event(&mut self, keycode: KeyCode, _keymods: KeyMods, repeat: bool) {
        if repeat {
            return
        }

        match keycode {
            KeyCode::Key1 => {self.sender.try_send(Message::SetValue(4, 1.0)).ok();},
            KeyCode::Key2 => {self.sender.try_send(Message::SetValue(5, 1.0)).ok();},
            KeyCode::Key3 => {self.sender.try_send(Message::SetValue(6, 1.0)).ok();},
            KeyCode::Key4 => {self.sender.try_send(Message::SetValue(7, 1.0)).ok();},
            KeyCode::A => {self.sender.try_send(Message::SetDspType(DspType::SimpleSine)).ok();},
            KeyCode::S => {self.sender.try_send(Message::SetDspType(DspType::BasicFm)).ok();},
            KeyCode::D => {self.sender.try_send(Message::SetDspType(DspType::DrumEngine)).ok();},
            KeyCode::F => {
                self.is_fullscreen = !self.is_fullscreen;
                window::set_fullscreen(self.is_fullscreen);
            },
            KeyCode::O => {
                self.show_oled_preview = !self.show_oled_preview;
                println!("OLED Preview: {}", if self.show_oled_preview { "ON" } else { "OFF" });
            },
            KeyCode::Up => {
                let num_shaders = self.shaders.len();
                if num_shaders > 0 {
                    let next_index = (self.current_shader_index + 1) % num_shaders;
                    self.sender.try_send(Message::SetShaderIndex(next_index)).ok();
                }
            },
            KeyCode::Right => {
                self.sender.try_send(Message::UiInput(InputEvent::Next)).ok();
            }
            KeyCode::Left => {
                self.sender.try_send(Message::UiInput(InputEvent::Prev)).ok();
            }
            KeyCode::Space => {
                self.sender.try_send(Message::UiInput(InputEvent::Enter)).ok();
            }
            KeyCode::Escape => {
                std::process::exit(0);
            }
            _ => (),
        }
    }

    fn key_up_event(&mut self, keycode: KeyCode, _keymods: KeyMods) {
        match keycode {
            KeyCode::Key1 => {self.sender.try_send(Message::SetValue(4, 0.0)).ok();},
            KeyCode::Key2 => {self.sender.try_send(Message::SetValue(5, 0.0)).ok();},
            KeyCode::Key3 => {self.sender.try_send(Message::SetValue(6, 0.0)).ok();},
            KeyCode::Key4 => {self.sender.try_send(Message::SetValue(7, 0.0)).ok();},
            _ => (),
        }
    }
}

#[repr(C)]
struct Vertex {
    pos: [f32; 2],
    uv: [f32; 2],
}

#[repr(C)]
struct Uniforms {
    u_time: f32,
    u_resolution: [f32; 2],
    u_cv1: f32,
    u_cv2: f32,
    u_cv3: f32,
    u_cv4: f32,
    u_gate1: f32,
    u_gate2: f32,
    u_gate3: f32,
    u_gate4: f32,
}

fn main_shader_meta() -> ShaderMeta {
    ShaderMeta { 
        uniforms: UniformBlockLayout { 
            uniforms: vec![
                UniformDesc::new("u_time", UniformType::Float1),
                UniformDesc::new("u_resolution", UniformType::Float2),
                UniformDesc::new("u_cv1", UniformType::Float1),
                UniformDesc::new("u_cv2", UniformType::Float1),
                UniformDesc::new("u_cv3", UniformType::Float1),
                UniformDesc::new("u_cv4", UniformType::Float1),
                UniformDesc::new("u_gate1", UniformType::Float1),
                UniformDesc::new("u_gate2", UniformType::Float1),
                UniformDesc::new("u_gate3", UniformType::Float1),
                UniformDesc::new("u_gate4", UniformType::Float1),
            ] 
        }, 
        images: vec![]
    }
}

fn oled_shader_meta() -> ShaderMeta {
    ShaderMeta {
        uniforms: UniformBlockLayout { uniforms: vec![] },
        images: vec!["u_oled_texture".to_string()],
    }
}