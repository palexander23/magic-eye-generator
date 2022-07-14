use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub fn run() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        match event {
            // Perform first time setup for the event loop
            Event::NewEvents(StartCause::Init) => {
                *control_flow = ControlFlow::Wait;
            }

            Event::WindowEvent { ref event, window_id } => {
                
                // Check we're examining the correct window
                if window.id() != window_id {
                    return;
                }

                // Handle window close button pressed
                if event == &WindowEvent::CloseRequested {
                    *control_flow = ControlFlow::Exit
                }
            }
            _ => {}
        }
    });
}

fn main() {
    std::env::set_var("WINIT_UNIX_BACKEND", "x11");

    run();
}
