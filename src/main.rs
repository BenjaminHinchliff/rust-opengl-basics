use std::path::Path;

use glutin::dpi;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::Api;
use glutin::ContextBuilder;
use glutin::GlProfile;
use glutin::GlRequest;

use nalgebra_glm as glm;

mod gl_render;
use gl_render::buffer;
use gl_render::color_buffer::ColorBuffer;
use gl_render::Transform;
use gl_render::Viewport;
mod resources;
use resources::Resources;
mod square;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

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

    let mut viewport = Viewport::for_window(WIDTH, HEIGHT);
    viewport.set_used(&gl);

    let color_buffer = ColorBuffer::from_color(glm::Vec3::new(0.3, 0.3, 0.5));
    color_buffer.set_used(&gl);

    let square = square::Square::new(&res, &gl).unwrap();

    // create transforms
    square.program().set_used();
    let model_matrix = glm::Mat4::identity();
    let model_matrix = glm::rotate(
        &model_matrix,
        -55f32.to_radians(),
        &glm::vec3(1.0, 0.0, 0.0),
    );
    let model = Transform::new(&square.program(), "model");
    model.set_matrix(&model_matrix);
    let view_matrix = glm::Mat4::identity();
    let view_matrix = glm::translate(&view_matrix, &glm::vec3(0.0, 0.0, -3.0));
    let view = Transform::new(&square.program(), "view");
    view.set_matrix(&view_matrix);
    let proj_matrix =
        glm::perspective(45f32.to_radians(), WIDTH as f32 / HEIGHT as f32, 0.1, 100.0);
    let proj = Transform::new(&square.program(), "projection");
    proj.set_matrix(&proj_matrix);
    square.program().set_unused();

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == gl_window.window().id() => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::Resized(dpi::PhysicalSize { width, height }),
                window_id,
            } if window_id == gl_window.window().id() => {
                viewport.update_size(width as i32, height as i32);
                viewport.set_used(&gl);
            }
            _ => (),
        }

        color_buffer.clear(&gl);
        square.render(&gl);

        gl_window.swap_buffers().unwrap();
    });
}
