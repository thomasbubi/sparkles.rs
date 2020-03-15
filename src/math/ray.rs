use crate::math::Vector3;

pub struct Ray {
    pub origin : Vector3,
    pub direction : Vector3
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray{ origin, direction }
    }

    pub fn at(t: f64) -> Vector3 {
        Vector3::new(
            origin.x + direction.x * t,
            origin.y + direction.y * t,
            origin.z + direction.z * t
        )
    }
}