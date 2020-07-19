use gl_render_derive::VertexAttribPointers;
use anyhow::Result;

use crate::gl_render::{self, data, Program};
use crate::buffer::{ArrayBuffer, VertexArray};
use crate::resources::Resources;

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

pub struct Triangle {
    program: gl_render::Program,
    _vbo: ArrayBuffer,
    vao: VertexArray,
}

impl Triangle {
    pub fn new(res: &Resources, gl: &gl::Gl) -> Result<Triangle> {
        let program = Program::from_res(&gl, &res, "shaders/triangle")?;

        // vertex data
        let vertices: Vec<Vertex> = vec![
            Vertex::new((-0.5, -0.5, 0.0), (1.0, 0.0, 0.0, 1.0)), // bottom right
            Vertex::new((0.5, -0.5, 0.0), (0.0, 1.0, 0.0, 1.0)),  // bottom left
            Vertex::new((0.0, 0.5, 0.0), (0.0, 0.0, 1.0, 1.0)),   // top
        ];

        let vbo = ArrayBuffer::new(gl);
        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();

        let vao = VertexArray::new(gl);

        vao.bind();
        vbo.bind();
        Vertex::vertex_attrib_pointers(gl);
        vbo.unbind();
        vao.unbind();

        Ok(Triangle { program, _vbo: vbo, vao })
    }

    pub fn render(&self, gl: &gl::Gl) {
        self.program.set_used();
        self.vao.bind();

        unsafe {
            gl.DrawArrays(
                gl::TRIANGLES,
                0,
                3
            );
        }

        self.vao.unbind();
        self.program.set_unused();
    }
}
