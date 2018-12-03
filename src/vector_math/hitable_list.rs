use vector_math::hitable::{HitRecord, Hitable};
use vector_math::ray::Ray;

pub struct HitableList {
    pub list: Vec<Box<Hitable>>,
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::zero();

        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for item in &self.list {
            if item.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *record = temp_rec;
            }
        }

        hit_anything
    }
}
