use crate::utils::state::*;

use miniquad::*;
use std::{fs, time::Instant};
use crossbeam_channel::Sender;
use triple_buffer::Output;

pub fn start_graphics_thread(window_sender: Sender<Message>, window_reader: Output<State>) {
    let conf = conf::Conf {
        window_title: String::from("Window Title"),
        high_dpi: true,
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

        Self { 
            pipeline, 
            bindings, 
            ctx,
            start_time: Instant::now(),
            sender,
            reader
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
            u_param1: state.values[0] as f32,
            u_param2: state.values[1] as f32,
            u_param3: state.values[2] as f32,
            u_param4: state.values[3] as f32,
        };
        self.ctx.apply_uniforms(UniformsSource::table(&uniforms));

        self.ctx.draw(0, 6, 1);

        self.ctx.end_render_pass();
        self.ctx.commit_frame();
    }

    fn resize_event(&mut self, width: f32, height: f32) {
        self.sender.try_send(Message::SetResolution(width, height)).unwrap();
        println!("Set resolution to: {width}, {height}");
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
    u_param1: f32,
    u_param2: f32,
    u_param3: f32,
    u_param4: f32,
}

fn shader_meta() -> ShaderMeta {
    ShaderMeta { 
        uniforms: UniformBlockLayout { 
            uniforms: vec![
                UniformDesc::new("u_time", UniformType::Float1),
                UniformDesc::new("u_resolution", UniformType::Float2),
                UniformDesc::new("u_param1", UniformType::Float1),
                UniformDesc::new("u_param2", UniformType::Float1),
                UniformDesc::new("u_param3", UniformType::Float1),
                UniformDesc::new("u_param4", UniformType::Float1),
            ] 
        }, 
        images: vec![] 
    }
}

fn load_shader(path: &str) -> String {
    fs::read_to_string(path)
    .expect(&format!("Failed to read shader: {path}"))
}
