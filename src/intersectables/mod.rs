mod plane;
mod sphere;

pub use self::plane::*;
pub use self::sphere::*;
use crate::math::{Ray, Vector3};
use crate::materials::Material;

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> f64;
    fn get_struct_name(&self) -> String;
    fn get_material(&self) -> &Box<Material>;
    fn get_normal_at(&self, intersection_point: Vector3) -> Vector3;
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

    fn get_normal_at(&self, intersection_point: Vector3) -> Vector3 {
        Vector3::new(0.0,0.0,0.0)
    }
}