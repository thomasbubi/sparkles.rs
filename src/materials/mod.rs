mod color;
mod shadelessmaterial;
mod normalmaterial;
mod diffusematerial;

pub use self::shadelessmaterial::*;
pub use self::normalmaterial::*;
pub use self::diffusematerial::*;
pub use self::color::*;
use crate::scene::Scene;
use crate::math::{Ray, Vector3};

pub trait Material {
    fn shade(&self, input: ShaderInput) -> Color;
}

pub struct ShaderInput<'a> {
    pub scene: &'a Scene,
    pub ray: &'a Ray,
    pub intersection_point: &'a Vector3,
    pub normal: &'a Vector3,
    pub current_recursion_depth: u32,
    pub max_recursion_depth: u32
}