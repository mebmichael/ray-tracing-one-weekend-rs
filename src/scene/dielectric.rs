use scene::material::Material;
use vector_math::vec3::Vec3;
use scene::light_ray::LightRay;
use scene::surface_interface::{reflect, refract, schlick};
use vector_math::ray::Ray;
use rand::prelude::*;

pub struct Dielectric {
    pub refractive_index: f32,
}

impl Dielectric {
    pub fn new(refractive_index: f32) -> Self {
        Dielectric { refractive_index }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        incident: &LightRay,
        hit_point: &Vec3,
        surface_normal: &Vec3,
    ) -> Option<LightRay> {

        let color = Vec3::new(1.0, 1.0, 1.0);
        let cos = incident.ray.direction.dot(*surface_normal) / incident.ray.direction.magnitude();
        
        let outward_normal: Vec3;
        let ni_over_nt: f32;
        let cosine: f32;

        if incident.ray.direction.dot(*surface_normal) > 0.0 {
            outward_normal = -(*surface_normal);
            ni_over_nt = self.refractive_index;
            cosine = self.refractive_index * cos;
        } else {
            outward_normal = *surface_normal;
            ni_over_nt = 1.0 / self.refractive_index;
            cosine = -cos;
        }

        let refracted = refract(&incident.ray.direction, &outward_normal, ni_over_nt);

        match refracted {
            Some(refracted) => {
                let mut rng = thread_rng(); // todo: consider the cost of this call, perhaps we need a shared ref.
                if rng.gen::<f32>() < schlick(cosine, self.refractive_index) {
                    let ray = Ray::new(*hit_point, reflect(&incident.ray.direction, surface_normal));
                    Some(LightRay::new(ray, incident.color * color))
                } else {
                    let ray = Ray::new(*hit_point, refracted);
                    Some(LightRay::new(ray, incident.color * color))
                }
            },
            None => {
                let ray = Ray::new(*hit_point, reflect(&incident.ray.direction, surface_normal));
                Some(LightRay::new(ray, incident.color * color))
            },
        }
    }
}
