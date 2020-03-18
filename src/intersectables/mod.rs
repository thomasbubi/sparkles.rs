mod plane;
mod sphere;

pub use self::plane::*;
pub use self::sphere::*;
use crate::math::Ray;
use crate::materials::Material;

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> f64;
    fn get_struct_name(&self) -> String;
    fn get_material(&self) -> &Box<Material>;
}

pub struct Background {
    pub material: Box<Material>
}

impl Intersectable for Background {
    fn intersect(&self, ray: &Ray) -> f64 {
        -1_000_000.0
    }

    fn get_struct_name(&self) -> String {
        "Background".to_string()
    }

    fn get_material(&self) -> &Box<Material> {
        &self.material
    }
}