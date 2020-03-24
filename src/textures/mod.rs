mod checkerboardtexture;

use crate::materials::Color;
pub use self::checkerboardtexture::*;

pub struct UV {
    pub u: f64,
    pub v: f64
}

impl UV {
    pub fn new(u: f64, v:f64) -> UV {
        UV{u,v}
    }
}

pub trait Texture {
    fn get_value_at(&self, uv: UV) -> Color;
}

pub struct SingleColorTexture {
    color: Color
}

impl SingleColorTexture {
    pub fn new(color: Color) -> SingleColorTexture {
        SingleColorTexture{color}
    }
}

impl Texture for SingleColorTexture {
    fn get_value_at(&self, uv: UV) -> Color {
        self.color.clone()
    }
}