use scene::material::Material;
use vector_math::vec3::Vec3;
use scene::light_ray::LightRay;
use vector_math::random_methods::random_in_unit_sphere;
use scene::surface_interface::reflect;
use vector_math::ray::Ray;

pub struct Metal {
    pub albedo: Vec3,
    pub roughness: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, roughness: f32) -> Self {
        Metal { albedo, roughness }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        incident: &LightRay,
        hit_point: &Vec3,
        surface_normal: &Vec3,
    ) -> Option<LightRay> {
        let reflected = reflect(&incident.ray.direction, surface_normal); // direction is assumed to be normalized

        if reflected.dot(*surface_normal) > 0.0 {
            let outgoing_ray = Ray::new(
                *hit_point,
                reflected + self.roughness * random_in_unit_sphere(),
            );
            let outgoing = LightRay::new(outgoing_ray, incident.color * self.albedo);
            Some(outgoing)
        } else {
            None
        }
    }
}
