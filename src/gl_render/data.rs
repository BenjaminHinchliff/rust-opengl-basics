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
}

impl From<(f32, f32, f32)> for vec3 {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        vec3::new(x, y, z)
    }
}
