use vector_math::ray::Ray;
use vector_math::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct LightRay {
    pub ray: Ray,
    pub color: Vec3,
}

impl LightRay {
    pub fn new(ray: Ray, color: Vec3) -> Self {
        LightRay { ray, color }
    }
}
