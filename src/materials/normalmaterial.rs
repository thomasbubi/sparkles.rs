use crate::math::Vector3;
use crate::materials::{Material, ShaderInput, Color};

pub struct NormalMaterial {}

impl Material for NormalMaterial {
    fn shade(&self, input: ShaderInput) -> Color {
        let r = 0.5 + input.normal.x / 2.0;
        let g = 0.5 + input.normal.y / 2.0;
        let b = 0.5 + input.normal.z / 2.0;
        Color::new(r,g,b,1.0)
    }
}
