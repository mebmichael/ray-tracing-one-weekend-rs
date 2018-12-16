use scene::light_ray::LightRay;
use scene::material::Material;
use scene::surface_interface::reflect;
use vector_math::random_methods::random_in_unit_sphere;
use vector_math::ray::Ray;
use vector_math::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Metal {
    pub albedo: Vec3,
    pub roughness: f32,
}

impl Metal {
    pub fn new(r: f32, g: f32, b: f32, roughness: f32) -> Self {
        Metal {
            albedo: Vec3::new(r, g, b),
            roughness,
        }
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
