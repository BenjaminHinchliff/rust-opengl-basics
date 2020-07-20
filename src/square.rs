use anyhow::Result;

use gl_render_derive::VertexAttribPointers;

use crate::buffer::{ArrayBuffer, ElementArrayBuffer, VertexArray};
use crate::gl_render::{self, data, texture, Program};
use crate::resources::Resources;

#[derive(Copy, Clone, Debug, VertexAttribPointers)]
#[repr(C, packed)]
pub struct Vertex {
    #[location = 0]
    pos: data::vec3,
    #[location = 1]
    color: data::U2U10U10U10RevFloat,
    #[location = 2]
    texcoord: data::vec2,
}

impl Vertex {
    pub fn new(pos: (f32, f32, f32), color: (f32, f32, f32, f32), texcoord: (f32, f32)) -> Self {
        Vertex {
            pos: pos.into(),
            color: color.into(),
            texcoord: texcoord.into(),
        }
    }
}

pub struct Square {
    program: gl_render::Program,
    _vbo: ArrayBuffer,
    ebo: ElementArrayBuffer,
    vao: VertexArray,
    container_tex: texture::Texture,
    face_tex: texture::Texture,
}

impl Square {
    pub fn new(res: &Resources, gl: &gl::Gl) -> Result<Square> {
        let program = Program::from_res(gl, &res, "shaders/square")?;

        // vertex data
        let vertices: Vec<Vertex> = vec![
            Vertex::new((-0.5, -0.5, 0.0), (1.0, 0.0, 0.0, 1.0), (0.0, 0.0)), // bottom left
            Vertex::new((0.5, -0.5, 0.0), (0.0, 1.0, 0.0, 1.0), (1.0, 0.0)),  // bottom right
            Vertex::new((-0.5, 0.5, 0.0), (1.0, 1.0, 1.0, 1.0), (0.0, 1.0)),  // top left
            Vertex::new((0.5, 0.5, 0.0), (0.0, 0.0, 1.0, 1.0), (1.0, 1.0)),   // top right
        ];

        // index buffer data
        let indices: Vec<gl::types::GLuint> = vec![0, 1, 2, 2, 1, 3];

        // load texture
        program.set_used();
        let container_tex = texture::Texture::new(gl, res, "textures/container.jpg", &program, "container")?;
        let face_tex = texture::Texture::new(gl, res, "textures/awesomeface.png", &program, "face")?;
        program.set_unused();

        let vbo = ArrayBuffer::new(gl);
        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();

        let ebo = ElementArrayBuffer::new(gl);
        ebo.bind();
        ebo.static_draw_data(&indices);
        ebo.unbind();

        let vao = VertexArray::new(gl);

        vao.bind();
        vbo.bind();
        Vertex::vertex_attrib_pointers(gl);
        vbo.unbind();
        vao.unbind();

        Ok(Square {
            program,
            _vbo: vbo,
            ebo,
            vao,
            container_tex,
            face_tex,
        })
    }

    pub fn render(&self, gl: &gl::Gl) {
        self.program.set_used();
        self.vao.bind();
        self.ebo.bind();
        self.container_tex.bind();
        self.face_tex.bind();

        unsafe {
            gl.DrawElements(
                gl::TRIANGLES,
                6,
                gl::UNSIGNED_INT,
                0 as *const gl::types::GLvoid,
            );
        }

        self.face_tex.unbind();
        self.container_tex.unbind();
        self.ebo.unbind();
        self.vao.unbind();
        self.program.set_unused();
    }

    pub fn program(&self) -> &Program {
        &self.program
    }
}
