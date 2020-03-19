use crate::materials::{Color, Material, ShaderInput};
use std::borrow::Borrow;

pub struct ShadelessMaterial {
    color: Color
}

impl ShadelessMaterial {
    pub fn new(color: Color) -> ShadelessMaterial {
        ShadelessMaterial {color}
    }
}

impl Material for ShadelessMaterial {
    fn shade(&self, input: ShaderInput) -> Color {
        self.color.clone()
    }
}