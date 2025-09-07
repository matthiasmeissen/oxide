use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Window Title")
        .build(&event_loop)
        .unwrap();

    // This creates an api for a direct connection to the gpu itself
    // The Backends::all() variant specifies that we automatically chosse the best one for the system
    let instance = wgpu::Instance::new(wgpu::Backends::all());

    // A surface connects the gpu to a window and gives it an area to draw into
    // The instance.create_surface(&window) method does just that
    // The unsafe function tells that we are sure that the window will be available
    let surface = unsafe { instance.create_surface(&window) };

    // The adapter is the handle to a gpu on the system
    // The instance.request_adapter() is an async function that asks wgpu to find a suitable adapter
    // The pollster::block_on() waits for the request to complete and gives us the adapter

    // The request_adapter function has some options we need to define
    // power preference lets us choose the performance of the gpu we need
    // surface lets us specify that we need the gpu to be able to draw on our surface
    // fallback lets us specify if we want to fallback to something when the is no gpu available
    let adapter = pollster::block_on(instance.request_adapter(
        &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }
    )).unwrap();

    // device is the logical connection to the gpu, here you can create textures, buffers and pipelines
    // queue is where you send instructions to the gpu
    // request_device is an async function the request the device and queue from the adapter
    // You can specify some details on the device, label for debugging, features you want to have and the limits
    let (device, queue) = pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: None,
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
        },
        None,
    )).unwrap();

    // Get window size and store it in variable
    let size = window.inner_size();

    // The configure method tells the surface how to create the underlying textures that will be presented to the window
    surface.configure(&device, &wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT, 
        format: surface.get_preferred_format(&adapter).unwrap(), 
        width: size.width, 
        height: size.height, 
        present_mode: wgpu::PresentMode::Fifo 
    });

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, ..} => {
                println!("The close button was pressed.");
                *control_flow = ControlFlow::Exit;
            },
            Event::WindowEvent { event: WindowEvent::CursorMoved { position: pos, ..}, .. } => {
                println!("Mouse position is: {:?}", pos);
            },
            Event::RedrawRequested(_) => {
                let output = surface.get_current_texture().unwrap();
                let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

                {
                    let _render_pass = encoder.begin_render_pass(
                        &wgpu::RenderPassDescriptor { 
                            label: Some("Render Pass"), 
                            color_attachments: &[wgpu::RenderPassColorAttachment {
                                view: &view,
                                resolve_target: None,
                                ops: wgpu::Operations { 
                                    load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.2, g: 0.8, b: 0.4, a: 1.0 }), 
                                    store: true 
                                },
                            }], 
                            depth_stencil_attachment: None }
                    );
                }
                
                queue.submit(std::iter::once(encoder.finish()));
                output.present();
            }
            _ => {
                //println!("Unhandled Event: {:?}", event);
            }
        }
    });
}