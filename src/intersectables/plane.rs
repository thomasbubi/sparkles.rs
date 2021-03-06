use crate::math::{Vector3, Ray};
use crate::intersectables::Intersectable;
use std::borrow::Borrow;
use crate::materials::Material;

pub struct Plane {
    position: Vector3,
    normal: Vector3,
    material: Box<Material>
}

impl Plane {
    pub fn new(position: Vector3, normal: Vector3, material: impl Material + 'static) -> Plane{
        Plane{ position, normal, material: Box::new(material)}
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> f64 {
        let dot_n_pos = Vector3::dot(self.normal.borrow(), self.position.borrow());
        let dot_n_ori = Vector3::dot(self.normal.borrow(), ray.origin.borrow());
        let dot_n_dir = Vector3::dot(self.normal.borrow(), ray.direction.borrow());
        (dot_n_pos - dot_n_ori) / dot_n_dir//this value is often called t
    }

    fn get_struct_name(&self) -> String {
        "Plane".to_string()
    }

    fn get_material(&self) -> &Box<Material> {
        &self.material
    }

    fn get_normal_at(&self, intersection_point: Vector3) -> Vector3 {
        self.normal.clone()
    }
}