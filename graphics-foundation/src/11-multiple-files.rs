use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::*,
    event_loop::{self, ControlFlow, EventLoop},
    window::{Fullscreen, Window, WindowBuilder},
};

use wgpu::util::DeviceExt;
use wgpu::{Adapter, Device, Instance, Queue, RenderPipeline, ShaderModule, Surface};

use std::time::Instant;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
struct Uniforms {
    time: f32,
    key1: f32,
    mouse: [f32; 2],
}

struct Fps {
    val: u32,
    frame_count: u32,
    last_update: Instant,
}

impl Fps {
    fn new() -> Self {
        Self { val: 0, frame_count: 0, last_update: Instant::now() }
    }

    fn update(&mut self) -> bool {
        self.frame_count += 1;
        if self.last_update.elapsed().as_secs_f32() >= 1.0 {
            self.val = self.frame_count;
            self.frame_count = 0;
            self.last_update = Instant::now();
            return true;
        }
        false
    }
}

struct State {
    // winit
    window: Window,
    // wgpu
    instance: Instance,
    surface: Surface,
    adapter: Adapter,
    device: Device,
    queue: Queue,
    // shaders
    render_pipelines: Vec<RenderPipeline>,
    current_pipeline_index: usize,
    // uniforms
    uniforms: Uniforms,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    // custom
    start_time: Instant,
    fps: Fps,
    is_fullscreen: bool,
    size: winit::dpi::PhysicalSize<u32>,
    mouse_position: [f32; 2],
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

        let uniforms = Uniforms {
            time: 0.0,
            key1: 0.0,
            mouse: [0.0, 0.0],
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group_layout =
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
                label: Some("uniform_bind_group_layout"),
            });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("uniform_bind_group"),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        let shader_sources = [
            ("Shader 1", include_str!("shader-01.wgsl")),
            ("Shader 2", include_str!("shader-02.wgsl")),
            ("Shader 3", include_str!("shader-03.wgsl")),
        ];

        let mut render_pipelines = Vec::new();

        for (label, source) in shader_sources.iter() {
            let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: Some(label),
                source: wgpu::ShaderSource::Wgsl((*source).into()),
            });

            let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(label),
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

            render_pipelines.push(render_pipeline);
        }



        let size = window.inner_size();

        Self {
            window,
            instance,
            surface,
            adapter,
            device,
            queue,
            render_pipelines,
            current_pipeline_index: 0,
            start_time: Instant::now(),
            uniforms,
            uniform_buffer,
            uniform_bind_group,
            fps: Fps::new(),
            is_fullscreen: false,
            size,
            mouse_position: [0.0, 0.0],
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
        let time = self.start_time.elapsed().as_secs_f32();
        
        if (self.fps.update()) {
            println!("Fps: {}", self.fps.val);
        };
        
        self.uniforms.time = self.start_time.elapsed().as_secs_f32();
        self.uniforms.mouse = self.mouse_position;
        
        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[self.uniforms]));

        self.set_stats_in_window_title();

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

            render_pass.set_pipeline(&self.render_pipelines[self.current_pipeline_index]);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.draw(0..4, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present()
    }

    fn get_mouse_position(&mut self, pos: PhysicalPosition<f64>) {
        self.mouse_position[0] = pos.x as f32;
        self.mouse_position[0] = pos.y as f32;
        println!("Mouse position is: {:?}", pos);
    }

    fn input(&mut self, event: &KeyboardInput) -> bool {
        match (event.virtual_keycode, event.state) {
            (Some(VirtualKeyCode::F), ElementState::Pressed) => {
                self.window.set_fullscreen(Some(Fullscreen::Borderless(None)));
                true
            },
            (Some(VirtualKeyCode::T), ElementState::Pressed) => {
                self.window.set_title("Test Title");
                true
            },
            (Some(VirtualKeyCode::D), ElementState::Pressed) => {
                self.window.set_decorations(false);
                true
            },
            (Some(VirtualKeyCode::Key1), ElementState::Pressed) => {
                self.uniforms.key1 = 1.0;
                true
            },
            (Some(VirtualKeyCode::Key1), ElementState::Released) => {
                self.uniforms.key1 = 0.0;
                true
            },
            (Some(VirtualKeyCode::Key2), ElementState::Pressed) => {
                self.current_pipeline_index = (self.current_pipeline_index + 1) % self.render_pipelines.len();
                true
            },
            _ => false,
        }
    }

    fn set_stats_in_window_title(&self) {
        let title = format!(
            "Window Title (Fps: {}, Time: {:.2}, Mouse: {:.2}, {:.2})", 
            self.fps.val, self.start_time.elapsed().as_secs_f32(), self.mouse_position[0], self.mouse_position[1]
        );
        self.window.set_title(&title);
    }
}

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
                    state.mouse_position[0] = position.x as f32 / state.size.width as f32;
                    state.mouse_position[1] = position.y as f32 / state.size.height as f32;
                    //state.get_mouse_position(position);
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    state.input(&input);
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
