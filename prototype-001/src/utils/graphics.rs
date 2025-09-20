use crate::utils::state::*;

use miniquad::*;
use std::{fs, time::Instant};
use crossbeam_channel::Sender;
use triple_buffer::Output;

pub fn start_graphics_thread(window_sender: Sender<Message>, window_reader: Output<State>) {
    let conf = conf::Conf {
        window_title: String::from("Window Title"),
        high_dpi: true,
        // Resolution has to be set at three points (here, Stage impl, state.rs)
        window_width: 960,
        window_height: 540,
        ..Default::default()
    };

    start(conf, || Box::new(Stage::new(window_sender, window_reader)));
}

struct Stage {
    pipeline: Pipeline,
    bindings: Bindings,
    ctx: Box<dyn RenderingBackend>,
    start_time: std::time::Instant,
    sender: Sender<Message>,
    reader: Output<State>,
    mq_resolution: [f32; 2],
    is_fullscreen: bool,
}

impl Stage {
    fn new(sender: Sender<Message>, reader: Output<State>) -> Self {
        let mut ctx = window::new_rendering_backend();

        // Define vertices with position and uv
        let vertices: [Vertex; 4] = [
            Vertex {pos: [-1.0, -1.0], uv: [0.0, 0.0]},
            Vertex {pos: [1.0, -1.0], uv: [1.0, 0.0]},
            Vertex {pos: [1.0, 1.0], uv: [1.0, 1.0]},
            Vertex {pos: [-1.0, 1.0], uv: [0.0, 1.0]},
        ];

        // Create vertex buffer with the defined vertices
        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer, 
            BufferUsage::Immutable, 
            BufferSource::slice(&vertices)
        );

        // Define indices, in which order the vertices connect
        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];

        // Create index buffer with the defined indices
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer, 
            BufferUsage::Immutable, 
            BufferSource::slice(&indices)
        );

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

        // Create shader by loading vertex and fragment shader files
        // as well as setting meta information
        let shader = ctx.new_shader(
            ShaderSource::Glsl { 
                vertex: VERTEX, 
                fragment: &load_shader("src/shaders/shader-01.glsl") 
            },
            shader_meta()
        ).expect("Something is not working");

        // Create bindings
        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![]
        };

        // Create pipeline
        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float2),
                VertexAttribute::new("in_uv", VertexFormat::Float2)
            ],
            shader,
            PipelineParams::default()
        );


        let (width, height) = window::screen_size();
        sender.try_send(Message::SetResolution(width, height)).ok();

        Self { 
            pipeline, 
            bindings, 
            ctx,
            start_time: Instant::now(),
            sender,
            reader,
            mq_resolution: [960.0, 540.0],
            is_fullscreen: false,
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {
        
    }

    fn draw(&mut self) {
        self.ctx.begin_default_pass(PassAction::Nothing);
        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.bindings);

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

        self.ctx.end_render_pass();
        self.ctx.commit_frame();
    }

    fn resize_event(&mut self, width: f32, height: f32) {
        self.mq_resolution = [width, height];
        self.sender.try_send(Message::SetResolution(width, height)).ok();
        println!("Set resolution to: {width}, {height}");
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        let dpi_factor = miniquad::window::dpi_scale();

        let norm_x = (x / dpi_factor / self.mq_resolution[0]) as f64;
        let norm_y = (y / dpi_factor / self.mq_resolution[1]) as f64;

        if norm_x >= 0.0 && norm_x <= 1.0 && norm_y >= 0.0 && norm_y <= 1.0 {
            self.sender.try_send(Message::SetValue(0, norm_x)).ok();
            self.sender.try_send(Message::SetValue(1, norm_y)).ok();
        }
    }

    fn key_down_event(&mut self, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Key1 => {self.sender.try_send(Message::SetValue(4, 1.0)).ok();},
            KeyCode::Key2 => {self.sender.try_send(Message::SetValue(5, 1.0)).ok();},
            KeyCode::Key3 => {self.sender.try_send(Message::SetValue(6, 1.0)).ok();},
            KeyCode::Key4 => {self.sender.try_send(Message::SetValue(7, 1.0)).ok();},
            KeyCode::F => {
                self.is_fullscreen = !self.is_fullscreen;
                window::set_fullscreen(self.is_fullscreen);
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

// You have to define your uniforms in three places
// In the struct Uniforms
// In the ShaderMeta
// In the draw function of the EventHandler implementation for Stage
#[repr(C)]
struct Uniforms {
    // Important: uniforms are f32 type
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

fn shader_meta() -> ShaderMeta {
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

fn load_shader(path: &str) -> String {
    fs::read_to_string(path)
    .expect(&format!("Failed to read shader: {path}"))
}
