use vector_math::random_methods::random_in_unit_sphere;
use vector_math::ray::Ray;
use vector_math::vec3::Vec3;
use vector_math::light_ray::LightRay;

pub trait Material {
    fn scatter(&self, incident: &LightRay, hit_point: &Vec3, surface_normal: &Vec3) -> Option<LightRay>;
}

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
    fn scatter(&self, incident: &LightRay, hit_point: &Vec3,  surface_normal: &Vec3) -> Option<LightRay> {
        let exitance_direction = *surface_normal + random_in_unit_sphere();
        let exitance_ray = Ray::new(*hit_point, exitance_direction.normalized());
        let color = incident.color * self.albedo;
        Some(LightRay::new(exitance_ray, color))
    }
}

fn reflect(v: &Vec3, normal: &Vec3) -> Vec3 {
    *v - 2.0 * v.dot(*normal) * (*normal)
}

pub struct Metal {
    pub albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, incident: &LightRay, hit_point: &Vec3,  surface_normal: &Vec3) -> Option<LightRay> {
        let reflected = reflect(&incident.ray.direction, surface_normal); // direction is assumed to be normalized

        if reflected.dot(*surface_normal) > 0.0 {
            let outgoing_ray = Ray::new(*hit_point, reflected);
            let outgoing = LightRay::new(outgoing_ray, self.albedo);
            Some(outgoing)
        } else {
            None
        }
    }
}
