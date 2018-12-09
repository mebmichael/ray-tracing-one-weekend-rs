use vector_math::light_ray::LightRay;
use vector_math::ray::Ray;

pub trait Hitable {
    fn hit_test(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<f32>;
    fn scatter(&self, incident: &LightRay) -> Option<LightRay>;
}
