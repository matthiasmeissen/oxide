// Import things from the winit libraray
// This library handles window creation
use winit::{
    // This is a global import and * means to import everything from the event module
    event::*,
    // This imports two specific items from the event_loop module
    event_loop::{ControlFlow, EventLoop},
    // This imports the WindowBuilder for the window module
    window::WindowBuilder,
};

fn main() {
    // Set up a logging system to get error messages from wgpu more meaningful
    env_logger::init();

    // Create a new central event handler
    // It recives receives all signals (events) from the operating system and makes it available to the program
    // It is created once and lives for the entire duration of the program
    let event_loop = EventLoop::new();

    // This creates a new window builder with default settings
    // The build() method takes the configuartion from the window builder and tells the operating system to create a window
    // The window needs to be linked to an event loop to serve as a line of communication
    // You can add methods to window builder to configure it (like .with_title() to set a title)

    // This build() method returns a Result enum with details about the action (success, error or something else)
    // The unwrap() method is a shortcut that says if it was successfull give me that window
    // This should not be used for production, since we should handle all possible outputs
    let window = WindowBuilder::new()
        .with_title("Window Title")
        .build(&event_loop)
        .unwrap();

    // The .run() method starts the event loop (it takes control over the current thread and never returns)
    // The program will live inside this loop until you close it

    // (move |event, _, control_flow| {}) is a closure 
    // It is an anonymouse function given to the run() method
    // It will be called for every event it reveives from the os

    // move - means the function takes ovenership over every variable used from the outside scope
    // It is not necessary right now, but will be needed as soon as we interact with the window

    // event - will be populated each call with the event that occured
    // For example Event::WindowEvent { event: WindowEvent::CloseRequested, ..} is the close button clicked event

    // _ is the event loop target, and in this case the _ means that we are intentionally not using it

    // control_flow - is how to tell the event loop what to do next
    // This can be set inside the function

    event_loop.run(move |event, _, control_flow| {
        // The * is a dereferene and is pointing to the place of control_flow
        // This lets you adjust the value stored at this pointer, not the pointer itself
        *control_flow = ControlFlow::Wait;

        // The match control flow defines what should happen at which event
        match event {
            // This match arm looks at the Event enum for the WindowEvent variant
            // And inside that if the event field is the WindowEvent::CloseRequested variant
            // And the .. means that we do not care about other fields
            Event::WindowEvent { event: WindowEvent::CloseRequested, ..} => {
                println!("The close button was pressed.");
                *control_flow = ControlFlow::Exit;
            },
            // For all other Event variants we print the to the console
            _ => {
                println!("Unhandled Event: {:?}", event);
            }
        }
    });
}