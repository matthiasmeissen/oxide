
use miniquad::*;
use std::{fs, time::Instant, time::Duration};

fn main() {
    let conf = conf::Conf {
        window_title: String::from("Window Title"),
        ..Default::default()
    };

    start(conf, || Box::new(Stage::new()));
}

struct Stage {
    pipeline: Pipeline,
    bindings: Bindings,
    ctx: Box<dyn RenderingBackend>,
    start_time: std::time::Instant,
    fps: FPS,
}

impl Stage {
    fn new() -> Self {
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
                fragment: &load_shader("src/miniquad/shader-01.glsl") 
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

        Self { 
            pipeline, 
            bindings, 
            ctx,
            start_time: Instant::now(),
            fps: FPS::new(),
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

        let time = self.start_time.elapsed().as_secs_f32();
        let uniforms = Uniforms { u_time: time };
        self.ctx.apply_uniforms(UniformsSource::table(&uniforms));

        self.ctx.draw(0, 6, 1);

        self.ctx.end_render_pass();
        self.ctx.commit_frame();

        self.fps.count();
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
}

fn load_shader(path: &str) -> String {
    fs::read_to_string(path)
    .expect(&format!("Failed to read shader: {path}"))
}

fn shader_meta() -> ShaderMeta {
    ShaderMeta { 
        uniforms: UniformBlockLayout { 
            uniforms: vec![
                UniformDesc::new("u_time", UniformType::Float1)
            ] 
        }, 
        images: vec![] 
    }
}

struct FPS {
    last_fps_instant: std::time::Instant,
    frame_count: u32,
    fps: u32,
}

impl FPS {
    fn new() -> Self {
        Self { 
            last_fps_instant: std::time::Instant::now(),
            frame_count: 0,
            fps: 0,
        }
    }

    fn count(&mut self) {
        self.frame_count += 1;

        let now = std::time::Instant::now();

        if now.duration_since(self.last_fps_instant).as_secs_f32() >= 1.0 {
            self.fps = self.frame_count;
            self.frame_count = 0;
            self.last_fps_instant = now;
            println!("FPS: {}", self.fps);
        }
    }
}
