use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Fullscreen, Window, WindowBuilder},
};

struct State {
    window: Window,
    is_fullscreen: bool,
}

impl State {
    fn new(event_loop: &EventLoop<()>) -> Self {
        let window = WindowBuilder::new()
            .with_title("Custom Window")
            .build(event_loop)
            .unwrap();

        Self { 
            window,
            is_fullscreen: false 
        }
    }
}

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new();

    let mut state = State::new(&event_loop);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, ..} => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::CursorMoved { position, .. } => {
                    println!("Mouse Position is: {:?}", position);
                }
                // This arm looks for a KeyboardInput Event
                WindowEvent::KeyboardInput { input, .. } => {
                    // Here we check if the keycode and the state match our intention and execute some code based on that
                    if input.virtual_keycode == Some(VirtualKeyCode::F) && input.state == ElementState::Pressed {
                        state.is_fullscreen = !state.is_fullscreen;

                        if state.is_fullscreen {
                            println!("Entering fullscreen.");
                            state.window.set_fullscreen(Some(Fullscreen::Borderless(None)));
                        } else {
                            println!("Leaving Fullscreen.");
                            state.window.set_fullscreen(None);
                        }
                    }
                }
                _ => ()
            },
            _ => ()
        }
    });
}