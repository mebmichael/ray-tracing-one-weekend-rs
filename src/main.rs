extern crate rand;

mod camera;
mod image_wrapper;
mod vector_math;

pub use camera::*;
pub use image_wrapper::*;
pub use vector_math::hitable::*;
pub use vector_math::hitable_list::*;
pub use vector_math::ray::*;
pub use vector_math::sphere::*;
pub use vector_math::vec3::*;

use rand::prelude::*;

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = thread_rng();
    loop {
        let p = Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) * 2.0 - Vec3::new(1.0, 1.0, 1.0);

        if p.squared_magnitude() >= 1.0 {
            return p;
        }
    }
}

fn get_color(r: &Ray, world: &HitableList) -> Vec3 {
    let mut rec = HitRecord::zero();

    if world.hit(r, 0.001, std::f32::MAX, &mut rec) {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        let random_ray = Ray::new(rec.p, target - rec.p);
        get_color(&random_ray, world) * 0.5
    } else {
        let unit_direction = r.direction.normalized();
        let t = 0.5 * (unit_direction.y + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    let mut image = ImageDataRGB::new(200, 100);
    let sample_count: u32 = 100;

    let image_width = image.width as f32;
    let image_height = image.height as f32;

    let sphere1 = Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    let sphere2 = Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    let world = HitableList {
        list: vec![sphere1, sphere2],
    };

    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(-2.0, -1.0, -1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
    );

    let mut rng = thread_rng();

    for j in 0..image.height {
        for i in 0..image.width {
            let mut color = Vec3::zero();

            for _ in 0..sample_count {
                let u = (i as f32 + rng.gen::<f32>()) / image_width;
                let v = (j as f32 + rng.gen::<f32>()) / image_height;

                let r = camera.get_ray(u, v);
                color += get_color(&r, &world);
            }

            color /= sample_count as f32;
            color = Vec3::new(f32::sqrt(color.x), f32::sqrt(color.y), f32::sqrt(color.z));

            let r = (255.99 * color.x) as u8;
            let g = (255.99 * color.y) as u8;
            let b = (255.99 * color.z) as u8;

            image.set_pixel((i, j), (r, g, b));
        }
    }

    let image_name = "output/image.png";
    let result = image.save(image_name);
    match result {
        Ok(()) => println!("Image saved to {}", image_name),
        Err(err) => println!("{:?}", err),
    }
}
