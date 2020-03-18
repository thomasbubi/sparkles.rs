use crate::math::{Vector3, Ray};
use crate::materials::Material;
use crate::intersectables::Intersectable;

pub struct Sphere {
    position: Vector3,
    radius: f64,
    material: Box<Material>
}

impl Sphere {
    pub fn new(position: Vector3, radius: f64, material: impl Material + 'static) -> Sphere {
        Sphere { position, radius, material: Box::new(material) }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> f64 {
        //solving the intersection between ray and sphere uses
        //calculating the discriminant

        let origin_minus_position = ray.origin.clone() - self.position.clone();
        let a = Vector3::dot(&ray.direction, &ray.direction);
        let b = 2.0 * Vector3::dot(&ray.direction,&origin_minus_position);
        let c = Vector3::dot(&origin_minus_position, &origin_minus_position)
            - self.radius * self.radius;

        let discriminant : f64 = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return -1_000_000.0;
        }

        ( -b - discriminant.sqrt() ) / 2.0 * a
    }

    fn get_struct_name(&self) -> String {
        "Sphere".to_string()
    }

    fn get_material(&self) -> &Box<Material> {
        &self.material
    }
}