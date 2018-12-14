use vector_math::vec3::Vec3;

pub fn reflect(v: &Vec3, normal: &Vec3) -> Vec3 {
    *v - 2.0 * v.dot(*normal) * (*normal)
}

pub fn refract(v: &Vec3, normal: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.normalized();
    let dt = uv.dot(*normal);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - (*normal) * dt) - f32::sqrt(discriminant) * (*normal))
    } else {
        None
    }
}

pub fn schlick(cosine: f32, refractive_index: f32) -> f32 {
    let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    let r0_squared = r0 * r0;
    r0_squared + (1.0 - r0_squared) * f32::powf(1.0 - cosine, 5.0)
}
