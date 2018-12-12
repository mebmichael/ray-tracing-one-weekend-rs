use scene::hitable::Hitable;
use scene::light_ray::LightRay;
use scene::material::*;
use vector_math::ray::Ray;
use vector_math::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Box<Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn hit_test(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<f32> {
        let oc = ray.origin - self.center; // vector from sphere center to ray origin

        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let temp = (-b - f32::sqrt(b * b - a * c)) / a;
            if temp < t_max && temp > t_min {
                return Some(temp);
            }

            let temp = (-b + f32::sqrt(b * b - a * c)) / a;
            if temp < t_max && temp > t_min {
                return Some(temp);
            }
        }

        None
    }

    fn scatter(&self, incident: &LightRay) -> Option<LightRay> {
        match self.hit_test(&incident.ray, 0.0, std::f32::MAX) {
            Some(t) => {
                let hit_point = incident.ray.point_at(t);
                let incoming_ray = Ray::new(hit_point, incident.ray.direction); // assuming direction is normalized
                let incoming_light = LightRay::new(incoming_ray, incident.color);
                let normal = (hit_point - self.center).normalized();
                self.material.scatter(&incoming_light, &hit_point, &normal)
            }
            None => None,
        }
    }
}
