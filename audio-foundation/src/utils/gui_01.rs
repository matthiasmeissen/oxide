use crate::AudioState; // We need access to the shared AudioState struct
use std::sync::{Arc, Mutex};
use winit::{
    event::{Event, WindowEvent, ElementState},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
    keyboard::PhysicalKey, // Be specific about what we import
};

// We've essentially renamed your `GraphicState` to be more descriptive.
// This struct will now own all the winit-related components.
pub struct UserInterface {
    event_loop: EventLoop<()>,
    window: Window,
    audio_state: Arc<Mutex<AudioState>>,
}

impl UserInterface {
    // The `new` function will set up everything winit needs.
    // It takes the shared audio_state as an argument.
    pub fn new(audio_state: Arc<Mutex<AudioState>>) -> Self {
        let event_loop = EventLoop::new().expect("Failed to create event loop");
        let window = WindowBuilder::new()
            .with_title("Mouse-Controlled Synthesizer")
            .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
            .build(&event_loop)
            .expect("Failed to create window");

        Self {
            event_loop,
            window,
            audio_state,
        }
    }

    // The `run` method will take ownership of the UserInterface and start the event loop.
    // This function will not return, as it runs until the user closes the window.
    pub fn run(self) {
        // These variables are purely for the UI's display logic, so they belong here.
        let mut current_frequency = 440.0;
        let mut current_amplitude = 0.2;

        // We move the entire event_loop.run call from main.rs into here.
        self.event_loop.run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Poll);

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        println!("Window closed. Goodbye!");
                        elwt.exit();
                    },
                    WindowEvent::CursorMoved { position, .. } => {
                        let window_size = self.window.inner_size();
                        let x_ratio = position.x as f32 / window_size.width as f32;
                        let frequency = 100.0 + (x_ratio * 1900.0);

                        let y_ratio = 1.0 - (position.y as f32 / window_size.height as f32);
                        let amplitude = y_ratio * 0.5;

                        current_frequency = frequency;
                        current_amplitude = amplitude;

                        if let Ok(mut state) = self.audio_state.lock() {
                            state.frequency = frequency;
                            state.amplitude = amplitude;
                        }

                        // Update window title logic is unchanged, just uses `self.`
                        self.window.set_title(&format!(
                            "Synthesizer - Freq: {:.1}Hz, Amp: {:.2}, Wave: {:?}",
                            frequency, amplitude,
                            self.audio_state.lock().map(|s| s.waveform).unwrap_or_default()
                        ));
                    },
                    WindowEvent::KeyboardInput { event, .. } => {
                        if event.state == ElementState::Pressed {
                            let new_waveform = match event.physical_key {
                                PhysicalKey::Code(code) => match code {
                                    winit::keyboard::KeyCode::Digit1 => Some(crate::Waveform::Sine),
                                    winit::keyboard::KeyCode::Digit2 => Some(crate::Waveform::Saw),
                                    winit::keyboard::KeyCode::Digit3 => Some(crate::Waveform::Triangle),
                                    winit::keyboard::KeyCode::Digit4 => Some(crate::Waveform::Square),
                                    _ => None,
                                },
                                _ => None,
                            };
                            
                            if let Some(waveform) = new_waveform {
                                if let Ok(mut state) = self.audio_state.lock() {
                                    state.waveform = waveform;
                                }
                                println!("Switched to {:?} wave", waveform);
                                
                                self.window.set_title(&format!(
                                    "Synthesizer - Freq: {:.1}Hz, Amp: {:.2}, Wave: {:?}",
                                    current_frequency, current_amplitude, waveform
                                ));
                            }
                        }
                    },
                    _ => {}
                },
                Event::AboutToWait => {
                    self.window.request_redraw();
                },
                _ => {}
            }
        }).expect("Event loop failed");
    }
}