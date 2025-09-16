
pub mod shared {
    #[derive(Clone, Copy, Debug)]
    pub struct Uniforms {
        pub u_resolution: [f32; 2],
        pub u_time: f32,
        pub u_params: [f32; 4],
    }
    
    impl Default for Uniforms {
        fn default() -> Self {
            Self { 
                u_resolution: [480.0, 320.0], 
                u_time: 0.0, 
                u_params: [0.5; 4] 
            }
        }
    }
}


mod app_main {
    use miniquad::*;
    use std::{fs, time::{Instant}};
    use std::io::{stdin, stdout, Write};
    use glam::*;
    use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}};
    use fundsp::hacker32::*;
    use crossbeam_channel::{bounded, Receiver, Sender};
    use midir::*;
    use crate::shared::*;

    pub struct Stage {
        pipeline: Pipeline,
        bindings: Bindings,
        ctx: Box<dyn RenderingBackend>,
        start_time: std::time::Instant,
        sender: Sender<Uniforms>,
        mouse_pos: [f32; 2],
        window_size: [f32; 2],
    }

    impl Stage {
        pub fn new(sender: Sender<Uniforms>) -> Self {
            let mut ctx = window::new_rendering_backend();

            let vertices: [Vertex; 4] = [
                Vertex {pos: [-1.0, -1.0], uv: [0.0, 0.0]},
                Vertex {pos: [1.0, -1.0], uv: [1.0, 0.0]},
                Vertex {pos: [1.0, 1.0], uv: [1.0, 1.0]},
                Vertex {pos: [-1.0, 1.0], uv: [0.0, 1.0]},
            ];

            let vertex_buffer = ctx.new_buffer(
                BufferType::VertexBuffer, 
                BufferUsage::Immutable, 
                BufferSource::slice(&vertices)
            );

            let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];

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

            let shader = ctx.new_shader(
                ShaderSource::Glsl { 
                    vertex: VERTEX, 
                    fragment: &load_shader("src/miniquad/shader-03.glsl") 
                },
                shader_meta()
            ).expect("Something is not working");

            let bindings = Bindings {
                vertex_buffers: vec![vertex_buffer],
                index_buffer: index_buffer,
                images: vec![]
            };

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
                mouse_pos: [0.5, 0.5],
                window_size: [480.0, 320.0],
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
            let uniforms = Uniforms { 
                u_resolution: [480.0, 320.0],
                u_time: time,
                u_params: [
                    self.mouse_pos[0],
                    self.mouse_pos[1],
                    0.5, 
                    0.5,
                ],
            };

            self.ctx.apply_uniforms(UniformsSource::table(&uniforms));

            self.ctx.draw(0, 6, 1);

            self.ctx.end_render_pass();
            self.ctx.commit_frame();
        }

        fn mouse_motion_event(&mut self, x: f32, y: f32) {
            let (w, h) = (self.window_size[0], self.window_size[1]);

            if x >= 0.0 && x <= w && y >= 0.0 && y <= h {
                self.mouse_pos[0] = x / w;
                self.mouse_pos[1] = y / h;

                let uniforms = Uniforms {
                    u_resolution: self.window_size,
                    u_time: self.start_time.elapsed().as_secs_f32(),
                    u_params: [self.mouse_pos[0], self.mouse_pos[1], 0.5, 0.5],
                };
                let _ = self.sender.try_send(uniforms);
            }
        }

        fn resize_event(&mut self, width: f32, height: f32) {
            self.window_size = [width, height];
        }
    }

    #[repr(C)]
    struct Vertex {
        pos: [f32; 2],
        uv: [f32; 2],
    }

    fn load_shader(path: &str) -> String {
        fs::read_to_string(path)
        .expect(&format!("Failed to read shader: {path}"))
    }

    fn shader_meta() -> ShaderMeta {
        ShaderMeta { 
            uniforms: UniformBlockLayout { 
                uniforms: vec![
                    UniformDesc::new("u_resolution", UniformType::Float2),
                    UniformDesc::new("u_time", UniformType::Float1),
                    UniformDesc::new("u_params", UniformType::Float4)
                ] 
            }, 
            images: vec![] 
        }
    }

    pub fn run() {

        let (sender, receiver): (Sender<Uniforms>, Receiver<Uniforms>) = bounded(1);

        let conf = conf::Conf {
            window_title: String::from("Window Title"),
            window_width: 480,
            window_height: 320,
            ..Default::default()
        };

        std::thread::spawn(move || {
            let mut midi_in = MidiInput::new("Midi Input 1").unwrap();
            midi_in.ignore(Ignore::None);

            let in_ports = midi_in.ports();
            let port = match in_ports.len() {
                0 => {
                    println!("No midi port available.");
                    return;
                },
                1 => {
                    println!("Connecting to the only available port: {}", midi_in.port_name(&in_ports[0]).unwrap());
                    &in_ports[0]
                },
                _ => {
                    println!("\nAvailable input ports:");
                    for (i, p) in in_ports.iter().enumerate() {
                        println!("{}: {}", i, midi_in.port_name(p).unwrap());
                    }
                    print!("Please select input port: ");
                    stdout().flush().unwrap();
                    let mut input = String::new();
                    stdin().read_line(&mut input).unwrap();
                    in_ports
                        .get(input.trim().parse::<usize>().unwrap())
                        .ok_or("invalid input port selected").unwrap()
                }
            };

            let _connection = midi_in.connect(
                port, "Midi Input", move|timestamp, message, _| {
                    println!("{:?}", message);
                }, ()
            ).unwrap();

            loop {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        });

        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("Could not find output device.");
        println!("Output Device: {:?}", device.name());
        let config = device.default_output_config().unwrap();
        let sample_format = config.sample_format();
        let stream_config: cpal::StreamConfig = config.into();
        println!("Stream config: {:?}", stream_config);
        let sample_rate = stream_config.sample_rate.0 as f64;
        let channels = stream_config.channels as usize;

        let freq1 = shared(0.2);
        let freq2 = shared(0.4);

        let osc1 = var(&freq1) * 8000.0 >> sine();
        let osc2 = var(&freq2) * 2000.0 >> sine();
        let synth = (osc1 + osc2) * 0.5;

        let mut graph = synth * 0.2;
        graph.set_sample_rate(sample_rate);

        let mut next_value = move || graph.get_stereo();

        let audio_callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            
            while let Ok(params) = receiver.try_recv() {
                freq1.set_value(params.u_params[0] as f32);
                freq2.set_value(params.u_params[1] as f32);
            }
            
            for frame in data.chunks_mut(channels) {
                    let (l, r) = next_value();
                    frame[0] = l;
                    if channels > 1 {
                        frame[1] = r;
                    }
                }
        };

        let err_fn = |err| eprintln!("An error occured on the audio stream: {}", err);

        let stream = match sample_format {
            cpal::SampleFormat::F32 => device.build_output_stream(
                &stream_config,
                audio_callback,
                err_fn,
                None
            ),
            _ => Err(cpal::BuildStreamError::StreamConfigNotSupported),
        }.expect("Could not build f32 output stream.");

        stream.play().expect("Could not start audio stream.");

        println!("Audio pipeline is running.");

        start(conf, || Box::new(Stage::new(sender)));
    }

}

fn main() {
    env_logger::init();

    app_main::run();
}

