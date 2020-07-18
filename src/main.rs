use std::path::Path;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::Api;
use glutin::ContextBuilder;
use glutin::GlProfile;
use glutin::GlRequest;

use render_gl_derive::VertexAttribPointers;

mod gl_render;
use gl_render::data;
use gl_render::Program;
mod resources;
use resources::Resources;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

#[derive(Copy, Clone, Debug, VertexAttribPointers)]
#[repr(C, packed)]
pub struct Vertex {
    #[location = 0]
    pos: data::vec3,
    #[location = 1]
    color: data::U2U10U10U10RevFloat,
}

impl Vertex {
    pub fn new(pos: (f32, f32, f32), color: (f32, f32, f32, f32)) -> Self {
        Vertex {
            pos: pos.into(),
            color: color.into(),
        }
    }
}

fn main() {
    // create resource loader
    let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();
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
    let gl = gl::Gl::load_with(|symbol| gl_window.get_proc_address(symbol));

    // create shader program
    let shader_program = Program::from_res(&gl, &res, "shaders/triangle").unwrap();

    unsafe {
        // set viewport size
        gl.Viewport(0, 0, WIDTH, HEIGHT);
        // set opengl clear color
        gl.ClearColor(1.0, 0.55, 0.0, 1.0);
    }

    // use program
    shader_program.set_used();

    // vertex data
    let vertices: Vec<Vertex> = vec![
        Vertex::new((-0.5, -0.5, 0.0), (1.0, 0.0, 0.0, 1.0)), // bottom right
        Vertex::new((0.5, -0.5, 0.0), (0.0, 1.0, 0.0, 1.0)),  // bottom left
        Vertex::new((0.0, 0.5, 0.0), (0.0, 0.0, 1.0, 1.0)),   // top
    ];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl.GenBuffers(1, &mut vbo);
    }

    unsafe {
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl.GenVertexArrays(1, &mut vao);
    }

    unsafe {
        gl.BindVertexArray(vao);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        Vertex::vertex_attrib_pointers(&gl);
        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        gl.BindVertexArray(0);
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
            gl.Clear(gl::COLOR_BUFFER_BIT);
            gl.BindVertexArray(vao);
            gl.DrawArrays(gl::TRIANGLES, 0, 3);
        }

        gl_window.swap_buffers().unwrap();
    });
}
