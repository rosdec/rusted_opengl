use winit::{
    event::{Event, WindowEvent},
    event_loop::{ EventLoop},
    window::WindowBuilder,
};
extern crate gl;

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested, window_id
            } => {
                control_flow.set_exit();
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            } 
            _ => (),
        }
    });
}