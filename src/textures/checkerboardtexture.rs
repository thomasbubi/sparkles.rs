use crate::materials::Color;
use crate::textures::{Texture, UV};

pub struct CheckerboardTexture {
    checker_size: f64,
    color_a: Color,
    color_b: Color
}

impl CheckerboardTexture {
    pub fn new(checker_size: f64, color_a: Color, color_b: Color) -> CheckerboardTexture {
        CheckerboardTexture {checker_size,color_a,color_b}
    }
}

impl Texture for CheckerboardTexture {
    fn get_value_at(&self, uv: UV) -> Color {
        let mut u = (1.0 / self.checker_size * uv.u) as i32;
        let mut v = (1.0 / self.checker_size * uv.v) as i32;

        if u < 0 && v < 0 {
            u *= -1;
            v *= -1;
        } else if u < 0 {
            u *= -1;
        } else if v < 0 {
            v *= -1;
        }

        return if u % 2 == 1 && v % 2 == 0 {
            self.color_a.clone()
        } else if u % 2 == 0 && v % 2 == 1 {
            self.color_a.clone()
        } else {
            self.color_b.clone()
        }
    }
}