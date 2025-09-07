use winit::{
    dpi::PhysicalPosition, 
    event::*, 
    event_loop::{ControlFlow, EventLoop}, 
    window::{Fullscreen, Window, WindowBuilder}
};

struct State {
    window: Window,
    is_fullscreen: bool,
    has_decorations: bool,
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
            has_decorations: true,
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
            (Some(VirtualKeyCode::D), ElementState::Pressed) => {
                self.toggle_decorations();
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

    fn toggle_decorations(&mut self) {
        self.has_decorations = !self.has_decorations;

        if self.has_decorations {
            self.window.set_decorations(true);
        } else {
            self.window.set_decorations(false);
        }
    }

    fn set_mouse_position(&mut self, pos: &PhysicalPosition<f64>) {
        self.mouse_position.0 = pos.x;
        self.mouse_position.1 = pos.y;
        println!("x: {} y: {}", self.mouse_position.0, self.mouse_position.1);
    }

    fn set_window_title_to_mouse(&mut self) {
        let title = format!("x: {:.2}, y: {:.2}",self.mouse_position.0, self.mouse_position.1);
        self.window.set_title(&title);
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
                    state.set_window_title_to_mouse();
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    state.input(&input);
                }
                WindowEvent::Resized( size ) => {
                    println!("Size is now {:?}", size);
                }
                _ => ()
            },
            _ => ()
        }
    });
}