use super::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn point_at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_point_at() {
        let o = Vec3::new(2.0, 2.0, 2.0);
        let d = Vec3::new(1.0, 0.0, 0.0);
        let r = Ray::new(o, d);

        assert_eq!(r.point_at(2.0), Vec3::new(4.0, 2.0, 2.0));
        assert_eq!(r.point_at(-3.0), Vec3::new(-1.0, 2.0, 2.0));
    }
}
