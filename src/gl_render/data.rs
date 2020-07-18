

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct U2U10U10U10RevFloat {
    pub inner: vec_2_10_10_10::Vector,
}

impl U2U10U10U10RevFloat {
    pub unsafe fn vertex_attrib_pointer(gl: &gl::Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            4,
            gl::UNSIGNED_INT_2_10_10_10_REV,
            gl::TRUE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(f32, f32, f32, f32)> for U2U10U10U10RevFloat {
    fn from((x, y, z, w): (f32, f32, f32, f32)) -> Self {
        U2U10U10U10RevFloat { inner: vec_2_10_10_10::Vector::new(x, y, z, w) }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> vec3 {
        vec3 { x, y, z }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(f32, f32, f32)> for vec3 {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        vec3::new(x, y, z)
    }
}
