use super::vec3::Vec3;
use rand::prelude::*;

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = thread_rng();
    loop {
        let p = 2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) - Vec3::new(1.0, 1.0, 1.0);

        if p.squared_magnitude() < 1.0 {
            return p;
        }
    }
}
