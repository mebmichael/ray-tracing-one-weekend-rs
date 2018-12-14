use scene::light_ray::LightRay;
use vector_math::vec3::Vec3;

pub trait Material {
    fn scatter(
        &self,
        incident: &LightRay,
        hit_point: &Vec3,
        surface_normal: &Vec3,
    ) -> Option<LightRay>;
}
