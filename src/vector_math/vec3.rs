use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

// Vec3 is a vector class
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn zero() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn magnitude(&self) -> f32 {
        f32::sqrt(self.squared_magnitude())
    }

    pub fn squared_magnitude(&self) -> f32 {
        self.dot(*self)
    }

    pub fn normalize(&mut self) -> &Self {
        let m = self.magnitude();
        self.x /= m;
        self.y /= m;
        self.z /= m;
        self
    }

    pub fn normalized(&self) -> Self {
        *self / self.magnitude()
    }

    pub fn dot(&self, rhs: Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: -(self.x * rhs.z - self.z * rhs.x),
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self {
        if rhs == 0.0 {
            panic!("Cannot divide by zero");
        }

        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        if rhs == 0.0 {
            panic!("Cannot divide by zero");
        }

        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

#[cfg(test)]
mod operator_tests {
    use super::*;

    #[test]
    fn vec3_neg() {
        let a = Vec3::new(1.0, 3.0, 5.0);
        let b = Vec3::new(-1.0, -3.0, -5.0);
        assert_eq!(-a, b);
    }

    #[test]
    fn vec3_add_vec3() {
        let a = Vec3::new(1.0, 3.0, 5.0);
        let b = Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(a + b, Vec3::new(3.0, 7.0, 11.0));
    }

    #[test]
    fn vec3_sub_vec3() {
        let a = Vec3::new(1.0, 3.0, 5.0);
        let b = Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(b - a, Vec3::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn vec3_mul_f32() {
        let a = Vec3::new(1.0, 3.0, 5.0);
        assert_eq!(a * 2.0, Vec3::new(2.0, 6.0, 10.0));
    }

    #[test]
    fn vec3_div_f32() {
        let a = Vec3::new(2.0, 6.0, 10.0);
        assert_eq!(a / 2.0, Vec3::new(1.0, 3.0, 5.0));
    }

    #[test]
    fn vec3_add_assign() {
        let mut a = Vec3::new(1.0, 3.0, 5.0);
        a += Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(a, Vec3::new(3.0, 7.0, 11.0));
    }

    #[test]
    fn vec3_sub_assign() {
        let mut a = Vec3::new(2.0, 4.0, 6.0);
        a -= Vec3::new(1.0, 3.0, 5.0);
        assert_eq!(a, Vec3::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn vec3_mul_assign() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        a *= 2.0;
        assert_eq!(a, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn vec3_div_assign() {
        let mut a = Vec3::new(2.0, 4.0, 6.0);
        a /= 2.0;
        assert_eq!(a, Vec3::new(1.0, 2.0, 3.0));
    }
}

#[cfg(test)]
mod method_tests {
    use super::*;

    #[test]
    fn vec3_magnitude() {
        let a = Vec3::new(2.0, 3.0, 6.0);
        assert_eq!(a.magnitude(), 7.0);
    }

    #[test]
    fn vec3_squared_magnitude() {
        let a = Vec3::new(2.0, 3.0, 6.0);
        assert_eq!(a.squared_magnitude(), 49.0);
    }

    #[test]
    fn vec3_normalize() {
        let mut a = Vec3::new(3.0, 4.0, 0.0);
        a.normalize();
        assert_eq!(a, Vec3::new(0.6, 0.8, 0.0));
    }

    #[test]
    fn vec3_normalized() {
        let a = Vec3::new(3.0, 4.0, 0.0);
        assert_eq!(a.normalized(), Vec3::new(0.6, 0.8, 0.0));
    }

    #[test]
    fn vec3_dot() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(a.dot(b), (2.0 + 8.0 + 18.0));
    }

    #[test]
    fn vec3_cross() {
        let x = Vec3::new(1.0, 0.0, 0.0);
        let y = Vec3::new(0.0, 2.0, 0.0);
        assert_eq!(x.cross(y), Vec3::new(0.0, 0.0, 2.0));

        let z = Vec3::new(0.0, 0.0, 1.0);
        assert_eq!(z.cross(y), Vec3::new(-2.0, 0.0, 0.0));
    }
}
