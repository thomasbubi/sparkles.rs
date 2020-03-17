use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::fmt::{Display, Formatter, Error, Result};

#[derive(Debug,Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector3 {

    pub fn new(x: f64, y:f64, z:f64) -> Vector3 {
        Vector3 {x, y, z}
    }

    pub fn abs2(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn abs(&self) -> f64 {
        self.abs2().sqrt()
    }

    pub fn cross(lhs: &Vector3, rhs: &Vector3) -> Vector3 {
        Vector3 {
            x: lhs.y * rhs.z - lhs.z * rhs.y,
            y: -( lhs.x * rhs.z ) + lhs.z * rhs.x,
            z: lhs.x * rhs.y - lhs.y * rhs.x
        }
    }

    pub fn dot(lhs: &Vector3, rhs: &Vector3) -> f64 {
        lhs.x * rhs.x + lhs.y * rhs.y +  lhs.z * rhs.z
    }

    pub fn normalize(&self) -> Vector3 {
        let abs = self.abs();
        if abs != 0.0 {
            return Vector3::new(self.x / abs, self.y / abs,self.z / abs)
        }

        Vector3::new(0.0,0.0,0.0)
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3::new( self.x + rhs.x, self.y + rhs.y, self.z + rhs.z )
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3::new( self.x - rhs.x, self.y - rhs.y, self.z - rhs.z )
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, scalar: f64) -> Self::Output {
        Vector3::new( self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, scalar: f64) {
        *self = Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        };
    }
}

impl Mul<i32> for Vector3 {
    type Output = Vector3;

    fn mul(self, scalar: i32) -> Self::Output {
        Vector3::new( self.x * scalar as f64, self.y * scalar as f64, self.z * scalar as f64)
    }
}

impl MulAssign<i32> for Vector3 {
    fn mul_assign(&mut self, scalar: i32) {
        *self = Self {
            x: self.x * scalar as f64,
            y: self.y * scalar as f64,
            z: self.z * scalar as f64,
        };
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({},{},{})",self.x, self.y,self.z)
    }
}