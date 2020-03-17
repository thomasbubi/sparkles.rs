use crate::materials::{Color, Material};
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
    fn shade(&self) -> Color {
        self.color.clone()
    }
}