use crate::math::{Vector3, Ray};

pub struct PerspectiveCamera {
    position: Vector3,
    direction: Vector3,
    up: Vector3,
    axis_u: Vector3,
    axis_v: Vector3,
    axis_w: Vector3,
    //aperture: f64,
    focal_length: f64,
    fov: f64,
    resolution_x: usize,
    resolution_y: usize,
    sensor_width: f64,
    sensor_height: f64,
    aspect_ratio: f64
}

impl PerspectiveCamera {
    pub fn new(position: Vector3, direction: Vector3, focal_length_in_mm: f64 ) -> PerspectiveCamera {
        // create orthonormal coordinate system for the camera with axes u, v, w
        // v is pointing upwards, w ist the negative view direction,
        // todo: camera's direction now cannot be (0,0,1)
        // todo: camera cannot be rotated aroud the direction vector
        let tmp_up = Vector3::new(0.0,0.0,1.0);
        let mut w = direction.clone() * -1;
        w = w.normalize();
        let mut u = Vector3::cross(&w, &tmp_up);
        u = u.normalize();
        let mut v = Vector3::cross(&u,&w);
        v = v.normalize();

        PerspectiveCamera {
            position,
            direction,
            up: tmp_up,
            axis_u: u,
            axis_v: v,
            axis_w: w,
            focal_length: focal_length_in_mm / 1000.0,
            fov: 0.0,
            resolution_x: 0,
            resolution_y: 0,
            sensor_width: 0.036,// digital full frame camera
            sensor_height: 0.024,
            aspect_ratio: 0.0
        }
    }

    pub fn create_camera_ray(&self, i: usize, j: usize) -> Ray {
        let r = self.sensor_width / 2.0;
        let l = -r;
        let b = self.sensor_height / 2.0;
        let t = -b;
        let u = l + (r - l) * (i as f64 + 0.5) / self.resolution_x as f64;
        let v = b + (t - b) * (j as f64 + 0.5) / self.resolution_y as f64;

        let mut view_ray_direction : Vector3 = self.axis_w.clone() * - self.focal_length + self.axis_u.clone() * u + self.axis_v.clone() * v;
        view_ray_direction = view_ray_direction.normalize();

        Ray::new(
            self.position.x, self.position.y, self.position.z,
            view_ray_direction.x,view_ray_direction.y,view_ray_direction.z
        )
    }

    pub fn set_resolution(&mut self, resolution_x: u32, resolution_y: u32) {
        self.resolution_x = resolution_x as usize;
        self.resolution_y = resolution_y as usize;
        self.aspect_ratio = resolution_x as f64 / resolution_y as f64;
        println!("aspect-ratio: {}", self.aspect_ratio);
    }
}