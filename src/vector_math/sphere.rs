use vector_math::hitable::{HitRecord, Hitable};
use vector_math::ray::Ray;
use vector_math::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center; // vector from sphere center to ray origin

        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let updated_record_fn = |t: f32| -> HitRecord {
                let p = ray.point_at(t);
                let normal = ((p - self.center) / self.radius).normalized(); // note: "normalized" here is added
                HitRecord::new(t, p, normal)
            };

            let temp = (-b - f32::sqrt(b * b - a * c)) / a;
            if temp < t_max && temp > t_min {
                *record = updated_record_fn(temp);
                return true;
            }

            let temp = (-b + f32::sqrt(b * b - a * c)) / a;
            if temp < t_max && temp > t_min {
                *record = updated_record_fn(temp);
                return true;
            }
        }
        return false;
    }
}
