use scene::light_ray::LightRay;
use scene::material::Material;
use vector_math::random_methods::random_in_unit_sphere;
use vector_math::ray::Ray;
use vector_math::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        incident: &LightRay,
        hit_point: &Vec3,
        surface_normal: &Vec3,
    ) -> Option<LightRay> {
        let exitance_direction = *surface_normal + random_in_unit_sphere();
        let exitance_ray = Ray::new(*hit_point, exitance_direction.normalized());
        let color = incident.color * self.albedo;
        Some(LightRay::new(exitance_ray, color))
    }
}
