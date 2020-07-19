use thiserror::Error;

use image::imageops as imops;

use crate::resources::{self, Resources};

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to load image {name}")]
    ImageLoad { name: String, message: String },
    #[error("failed to load a resource")]
    Resource(#[from] resources::Error),
}

pub struct Texture {
    id: gl::types::GLuint,
    gl: gl::Gl,
}

impl Texture {
    pub fn new(gl: &gl::Gl, res: &Resources, name: &str) -> Result<Texture, Error> {
        let (width, height, img, format) = match res.load_image(name)? {
            image::DynamicImage::ImageRgb8(mut img) => {
                imops::flip_vertical_in_place(&mut img);
                (img.width(), img.height(), img.into_vec(), gl::RGB)
            }
            image::DynamicImage::ImageRgba8(mut img) => {
                imops::flip_vertical_in_place(&mut img);
                (img.width(), img.height(), img.into_vec(), gl::RGBA)
            }
            _ => {
                return Err(Error::ImageLoad {
                    name: name.to_string(),
                    message: String::from("failed to load image texture - invalid image type"),
                });
            }
        };

        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl.GenTextures(1, &mut id);
        }

        unsafe {
            // bind
            gl.BindTexture(gl::TEXTURE_2D, id);
            // set wrapping
            gl.TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::REPEAT as gl::types::GLint,
            );
            gl.TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::REPEAT as gl::types::GLint,
            );
            // set filtering
            gl.TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR as gl::types::GLint,
            );
            gl.TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR as gl::types::GLint,
            );
            // buffer image
            gl.TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as gl::types::GLint,
                width as gl::types::GLsizei,
                height as gl::types::GLsizei,
                0,
                format,
                gl::UNSIGNED_BYTE,
                img.as_ptr() as *const gl::types::GLvoid,
            );
            gl.GenerateMipmap(gl::TEXTURE_2D);
            // unbind
            gl.BindTexture(gl::TEXTURE_2D, 0);
        }

        Ok(Texture { id, gl: gl.clone() })
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteTextures(1, &self.id);
        }
    }
}
