use std::ffi::{CStr, CString};

use thiserror::Error;

use nalgebra_glm as glm;

use crate::resources::{self, Resources};

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to load resource {name}")]
    ResourceLoad {
        name: String,
        inner: resources::Error,
    },
    #[error("cannot determine shader type for resource {name}")]
    CanNotDetermineShaderTypeForResource { name: String },
    #[error("failed to compile shader {name}: {message}")]
    CompileError { name: String, message: String },
    #[error("failed to link program {name}: {message}")]
    LinkError { name: String, message: String },
}

pub struct Program {
    id: gl::types::GLuint,
    gl: gl::Gl,
}

impl Program {
    pub fn from_res(gl: &gl::Gl, res: &Resources, name: &str) -> Result<Program, Error> {
        const POSSIBLE_EXT: [&str; 2] = [".vert", ".frag"];

        let shaders = POSSIBLE_EXT
            .iter()
            .map(|file_extension| Shader::from_res(gl, res, &format!("{}{}", name, file_extension)))
            .collect::<Result<Vec<Shader>, Error>>()?;

        Program::from_shaders(gl, &shaders[..]).map_err(|message| Error::LinkError {
            name: name.into(),
            message,
        })
    }

    pub fn from_shaders(gl: &gl::Gl, shaders: &[Shader]) -> Result<Program, String> {
        let id = unsafe { gl.CreateProgram() };
        for shader in shaders {
            unsafe {
                gl.AttachShader(id, shader.id());
            }
        }

        unsafe { gl.LinkProgram(id) };

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl.GetProgramiv(id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl.GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_cstring_with_len(len as usize);

            unsafe {
                gl.GetProgramInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe {
                gl.DetachShader(id, shader.id());
            }
        }

        Ok(Program { id, gl: gl.clone() })
    }

    pub fn set_used(&self) {
        unsafe {
            self.gl.UseProgram(self.id);
        }
    }

    pub fn set_unused(&self) {
        unsafe {
            self.gl.UseProgram(0);
        }
    }

    #[allow(dead_code)]
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn get_uniform_location(&self, name: &str) -> gl::types::GLint {
        unsafe {
            self.gl
                .GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr())
        }
    }

    pub fn set_matrix4fv(&self, loc: i32, mat: &glm::Mat4) {
        unsafe {
            self.gl
                .UniformMatrix4fv(loc, 1, gl::FALSE, glm::value_ptr(&mat).as_ptr());
        }
    }

    #[allow(dead_code)]
    pub fn get_and_set_matrix4fv(&self, name: &str, mat: &glm::Mat4) {
        self.set_matrix4fv(self.get_uniform_location(name), mat);
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.id);
        }
    }
}

pub struct Shader {
    id: gl::types::GLuint,
    gl: gl::Gl,
}

impl Shader {
    pub fn from_res(gl: &gl::Gl, res: &Resources, name: &str) -> Result<Shader, Error> {
        const POSSIBLE_EXT: [(&str, gl::types::GLenum); 2] =
            [(".vert", gl::VERTEX_SHADER), (".frag", gl::FRAGMENT_SHADER)];

        let shader_kind = POSSIBLE_EXT
            .iter()
            .find(|&&(file_extension, _)| name.ends_with(file_extension))
            .map(|&(_, kind)| kind)
            .ok_or_else(|| Error::CanNotDetermineShaderTypeForResource { name: name.into() })?;

        let source = res.load_cstring(name).map_err(|e| Error::ResourceLoad {
            name: name.into(),
            inner: e,
        })?;

        Shader::from_source(gl, &source, shader_kind).map_err(|message| Error::CompileError {
            name: name.into(),
            message,
        })
    }

    pub fn _from_vert_source(gl: &gl::Gl, source: &CStr) -> Result<Shader, String> {
        Shader::from_source(gl, source, gl::VERTEX_SHADER)
    }

    pub fn _from_frag_source(gl: &gl::Gl, source: &CStr) -> Result<Shader, String> {
        Shader::from_source(gl, source, gl::FRAGMENT_SHADER)
    }

    pub fn from_source(
        gl: &gl::Gl,
        source: &CStr,
        kind: gl::types::GLenum,
    ) -> Result<Shader, String> {
        let id = unsafe { gl.CreateShader(kind) };

        unsafe {
            gl.ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl.CompileShader(id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error: CString = create_cstring_with_len(len as usize);
            unsafe {
                gl.GetShaderInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        Ok(Shader { id, gl: gl.clone() })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}

fn create_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend(std::iter::repeat(b' ').take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}
