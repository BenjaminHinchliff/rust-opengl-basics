pub struct ArrayBuffer {
    gl: gl::Gl,
    vbo: gl::types::GLuint,
}

impl ArrayBuffer {
    pub fn new(gl: &gl::Gl) -> ArrayBuffer {
        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl.GenBuffers(1, &mut vbo);
        }
        ArrayBuffer {
            gl: gl.clone(),
            vbo,
        }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn static_draw_data<T>(&self, data: &[T]) {
        unsafe {
            self.gl.BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }
    }
}

impl Drop for ArrayBuffer {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteBuffers(1, &mut self.vbo);
        }
    }
}
