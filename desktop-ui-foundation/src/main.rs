
use miniquad::*;
use std::fs;

fn main() {
    let conf = conf::Conf::default();

    start(conf, || Box::new(Stage::new()));
}

struct Stage {
    pipeline: Pipeline,
    bindings: Bindings,
    ctx: Box<dyn RenderingBackend>,
}

impl Stage {
    fn new() -> Self {
        let mut ctx = window::new_rendering_backend();

        // Define vertices with position and color (color defaults to white)
        let vertices: [Vertex; 3] = [
            vert([-1.0, 1.0]),
            vert([1.0, 1.0]),
            vert([1.0, -1.0]),
        ];

        // Create vertex buffer with the defined vertices
        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer, 
            BufferUsage::Immutable, 
            BufferSource::slice(&vertices)
        );

        // Define indices, in which order the vertices connect
        let indices: [u16; 3] = [0, 1, 2];

        // Create index buffer with the defined indices
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer, 
            BufferUsage::Immutable, 
            BufferSource::slice(&indices)
        );

        // Create shader by loading vertex and fragment shader files
        // as well as setting meta information
        let shader = ctx.new_shader(
            ShaderSource::Glsl { 
                vertex: &load_shader("src/miniquad/vert.glsl"), 
                fragment: &load_shader("src/miniquad/frag.glsl") 
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
                VertexAttribute::new("in_color", VertexFormat::Float4)
            ],
            shader,
            PipelineParams::default()
        );

        Self { 
            pipeline, 
            bindings, 
            ctx
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

        self.ctx.draw(0, 3, 1);

        self.ctx.end_render_pass();
        self.ctx.commit_frame();
    }
}

#[repr(C)]
struct Vertex {
    pos: [f32; 2],      // x, y
    color: [f32; 4],    // r, g, b, a
}

fn vert(pos: [f32; 2]) -> Vertex {
    Vertex { pos, color: [1.0; 4] }
}

fn load_shader(path: &str) -> String {
    fs::read_to_string(path)
    .expect(&format!("Failed to read shader: {path}"))
}

fn shader_meta() -> ShaderMeta {
    ShaderMeta { 
        uniforms: UniformBlockLayout { 
            uniforms: vec![] 
        }, 
        images: vec![] 
    }
}
