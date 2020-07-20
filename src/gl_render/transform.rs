use nalgebra_glm as glm;

use super::Program;

pub struct Transform<'a> {
    program: &'a Program,
    loc: gl::types::GLint,
}

impl<'a> Transform<'a> {
    pub fn new(program: &'a Program, field: &str) -> Transform<'a> {
        Transform {
            program,
            loc: program.get_uniform_location(&field),
        }
    }

    pub fn set_matrix(&self, matrix: &glm::Mat4) {
        self.program.set_matrix4fv(self.loc, matrix);
    }
}
