#[macro_use]
extern crate glium;

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex { position: [0.0, 0.0] };
    let vertex2 = Vertex { position: [0.0, 1.0] };
    let vertex3 = Vertex { position: [1.0, 1.0] };
    let vertex4 = Vertex { position: [1.0, 0.0] };
    let shape = vec![vertex1, vertex2, vertex3, vertex1, vertex3, vertex4];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    const TARGET_FPS: u64 = 60;

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        uniform mat4 matrix;
        out vec2 my_attr;      

        void main() {
            my_attr = position; 
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec2 my_attr;
        out vec4 color;

        void main() {
            color = vec4(my_attr, 0.0, 1.0);   
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut t: f32 = 0.0;
    let mut delta: f32 = 0.02;
    event_loop.run(move |event, _, control_flow| {
        let start_time = std::time::Instant::now();

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                glutin::event::WindowEvent::KeyboardInput { input, .. } => if input.state == glutin::event::ElementState::Pressed {
                    if let Some(key) = input.virtual_keycode {
                        match key {
                            glutin::event::VirtualKeyCode::C => delta = -delta,
                            glutin::event::VirtualKeyCode::R => t = 0.0,
                            _ => {}
                        }
                    }
                }, 
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let elapsed_time = std::time::Instant::now().duration_since(start_time).as_millis() as u64;

        let wait_millis = match 1000 / TARGET_FPS >= elapsed_time {
            true => 1000 / TARGET_FPS - elapsed_time,
            false => 0
        };
        let new_inst = start_time + std::time::Duration::from_millis(wait_millis);

        *control_flow =  glutin::event_loop::ControlFlow::WaitUntil(new_inst);

        t += delta;

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let uniforms = uniform! {
            matrix: [
                [ t.cos(), t.sin(), 0.0, 0.0],
                [-t.sin(), t.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ]
        };

        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();
    });
}