use crate::math::Vector3;

pub struct Ray {
    pub origin : Vector3,
    pub direction : Vector3
}

impl Ray {
    pub fn new(o_x: f64, o_y: f64, o_z: f64, d_x: f64, d_y: f64, d_z: f64) -> Ray {
        Ray{
            origin: Vector3::new(o_x,o_y,o_z),
            direction: Vector3::new(d_x,d_y,d_z)
        }
    }

    pub fn new_from_vec3(origin: Vector3, direction: Vector3) -> Ray {
        Ray{ origin, direction }
    }

    pub fn at(&self, t: f64) -> Vector3 {
        Vector3::new(
            self.origin.x + self.direction.x * t,
            self.origin.y + self.direction.y * t,
            self.origin.z + self.direction.z * t
        )
    }
}