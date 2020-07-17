use std::ffi::CString;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::Api;
use glutin::ContextBuilder;
use glutin::GlProfile;
use glutin::GlRequest;

mod gl_render;
use gl_render::Program;
use gl_render::Shader;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

fn main() {
    // create event loop
    let el = EventLoop::new();
    // create window builder
    let wb = WindowBuilder::new()
        .with_title("Hello world!")
        .with_inner_size(glutin::dpi::LogicalSize::new(WIDTH, HEIGHT));
    // create window and context
    let gl_window = ContextBuilder::new()
        .with_gl_profile(GlProfile::Core)
        .with_gl(GlRequest::Specific(Api::OpenGl, (4, 5)))
        .build_windowed(wb, &el)
        .unwrap();
    // make context current or panic
    let gl_window = unsafe { gl_window.make_current() }.unwrap();

    // load gl functions
    gl::load_with(|symbol| gl_window.get_proc_address(symbol));

    // load shaders
    let vert_shader =
        Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap()).unwrap();
    let frag_shader =
        Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap()).unwrap();

    // create shader program
    let shader_program = Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    unsafe {
        // set viewport size
        gl::Viewport(0, 0, WIDTH, HEIGHT);
        // set opengl clear color
        gl::ClearColor(1.0, 0.55, 0.0, 1.0);
    }

    // use program
    shader_program.set_used();

    // vertex data
    let vertices: Vec<f32> = vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }

    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null(),
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == gl_window.window().id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        gl_window.swap_buffers().unwrap();
    });
}
