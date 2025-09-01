use winit::{
    dpi::PhysicalPosition, 
    event::*, 
    event_loop::{ControlFlow, EventLoop}, 
    window::{Fullscreen, Window, WindowBuilder}
};

struct State {
    window: Window,
    is_fullscreen: bool,
    mouse_position: (f64, f64),
}

impl State {
    fn new(event_loop: &EventLoop<()>) -> Self {
        let window = WindowBuilder::new()
            .with_title("Custom Window")
            .build(event_loop)
            .unwrap();

        Self { 
            window,
            is_fullscreen: false,
            mouse_position: (0.0, 0.0),
        }
    }

    fn input(&mut self, event: &KeyboardInput) -> bool {
        match (event.virtual_keycode, event.state) {
            (Some(VirtualKeyCode::F), ElementState::Pressed) => {
                self.toggle_fullscreen();
                true
            },
            (Some(VirtualKeyCode::T), ElementState::Pressed) => {
                self.window.set_title("Test Title");
                true
            },
            _ => false,
        }
    }

    fn toggle_fullscreen(&mut self) {
        self.is_fullscreen = !self.is_fullscreen;

        if self.is_fullscreen {
            println!("Entering fullscreen.");
            self.window.set_fullscreen(Some(Fullscreen::Borderless(None)));
        } else {
            println!("Leaving Fullscreen.");
            self.window.set_fullscreen(None);
        }
    }

    fn set_mouse_position(&mut self, pos: &PhysicalPosition<f64>) {
        self.mouse_position.0 = pos.x;
        self.mouse_position.1 = pos.y;
        println!("x: {} y: {}", self.mouse_position.0, self.mouse_position.1);
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
                    state.set_mouse_position(&position);
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    state.input(&input);
                }
                _ => ()
            },
            _ => ()
        }
    });
}