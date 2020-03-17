mod plane;

pub use self::plane::*;
use crate::math::Ray;

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> f64;
    fn get_struct_name(&self) -> String;
}