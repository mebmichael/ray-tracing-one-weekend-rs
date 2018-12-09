use vector_math::hitable::Hitable;
use vector_math::light_ray::LightRay;

pub struct HitableList {
    pub list: Vec<Box<Hitable>>,
}

impl HitableList {
    pub fn scatter(&self, path: &LightRay, t_min: f32, t_max: f32) -> Option<LightRay> {
        let mut closest_so_far = t_max;
        let mut hit_result: Option<&Box<Hitable>> = None;

        for item in &self.list {
            match item.hit_test(&path.ray, t_min, closest_so_far) {
                Some(t) => {
                    closest_so_far = t;
                    hit_result = Some(item);
                }
                None => {}
            }
        }

        match hit_result {
            Some(object) => object.scatter(path),
            None => None,
        }
    }
}
