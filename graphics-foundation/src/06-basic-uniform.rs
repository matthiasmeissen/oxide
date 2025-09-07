use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::*,
    event_loop::{self, ControlFlow, EventLoop},
    window::{Fullscreen, Window, WindowBuilder},
};

use wgpu::util::DeviceExt;
use wgpu::{Adapter, Device, Instance, Queue, RenderPipeline, ShaderModule, Surface};

struct State {
    // winit
    window: Window,
    // wgpu
    instance: Instance,
    surface: Surface,
    adapter: Adapter,
    device: Device,
    queue: Queue,
    shader: ShaderModule,
    render_pipeline: RenderPipeline,
    time: f32,
    time_buffer: wgpu::Buffer,
    time_bind_group: wgpu::BindGroup,
    // custom
    is_fullscreen: bool,
    size: winit::dpi::PhysicalSize<u32>,
    mouse_position: (f64, f64),
}

impl State {
    fn new(event_loop: &EventLoop<()>) -> Self {
        let window = WindowBuilder::new()
            .with_title("Custom Window")
            .build(event_loop)
            .unwrap();

        let instance = wgpu::Instance::new(wgpu::Backends::all());

        let surface = unsafe { instance.create_surface(&window) };

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None,
        ))
        .unwrap();

        let time_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Time Buffer"),
            contents: &0.0f32.to_ne_bytes()[..],
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let time_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("time_bind_group_layout"),
            });

        let time_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &time_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: time_buffer.as_entire_binding(),
            }],
            label: Some("time_bind_group"),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&time_bind_group_layout],
            push_constant_ranges: &[],
        });

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(SHADER_CODE.into()),
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: surface.get_preferred_format(&adapter).unwrap(),
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            multiview: None,
        });

        let size = window.inner_size();

        Self {
            window,
            instance,
            surface,
            adapter,
            device,
            queue,
            shader,
            render_pipeline,
            time: 0.0,
            time_buffer,
            time_bind_group,
            is_fullscreen: false,
            size,
            mouse_position: (0.0, 0.0),
        }
    }

    fn set_size(&mut self) {
        self.size = self.window.inner_size();
    }

    fn configure_surface(&mut self) {
        println!("Configure size: {:?}", self.size);
        self.set_size();
        self.surface.configure(
            &self.device,
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: self.surface.get_preferred_format(&self.adapter).unwrap(),
                width: self.size.width,
                height: self.size.height,
                present_mode: wgpu::PresentMode::Fifo,
            },
        );
    }

    fn draw_frame(&mut self) {
        self.time += 0.01;
        self.queue.write_buffer(&self.time_buffer, 0, &self.time.to_ne_bytes());

        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.2,
                            g: 0.8,
                            b: 0.4,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.time_bind_group, &[]);
            render_pass.draw(0..4, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present()
    }

    fn get_mouse_position(&self, pos: PhysicalPosition<f64>) {
        println!("Mouse position is: {:?}", pos);
    }
}

const SHADER_CODE: &str = r#"
    struct Uniforms {
        time: f32;
    };

    [[group(0), binding(0)]]
    var<uniform> uniforms: Uniforms;

    struct VertexOutput {
        [[builtin(position)]] clip_position: vec4<f32>;
        [[location(0)]] uv: vec2<f32>;
    };

    [[stage(vertex)]]
    fn vs_main([[builtin(vertex_index)]] in_vertex_index: u32) -> VertexOutput {
        var out: VertexOutput;

        let x = f32(i32(in_vertex_index) % 2) * 2.0 - 1.0;
        let y = f32(i32(in_vertex_index) / 2) * 2.0 - 1.0;
        
        out.clip_position = vec4<f32>(x, -y, 0.0, 1.0);
        out.uv = vec2<f32>((x + 1.0) / 2.0, (y + 1.0) / 2.0);

        return out;
    }

    [[stage(fragment)]]
    fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
        let r = sin(uniforms.time + in.uv.x);
        let g = cos(uniforms.time + in.uv.y);
        return vec4<f32>(r, g, 0.8, 1.0);
    }
"#;

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();

    let mut state = State::new(&event_loop);
    state.configure_surface();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    println!("The close button was pressed.");
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::CursorMoved { position, .. } => {
                    state.get_mouse_position(position);
                }
                WindowEvent::Resized(new_size) => {
                    println!("New size is: {:?}", new_size);
                    state.configure_surface();
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                state.draw_frame();
            }
            Event::MainEventsCleared => {
                state.window.request_redraw();
            }
            _ => {
                //println!("Unhandled Event: {:?}", event);
            }
        }
    });
}
