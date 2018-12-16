extern crate rand;

mod image_wrapper;
mod scene;
mod vector_math;

pub use image_wrapper::*;
use rand::prelude::*;
pub use scene::camera::Camera;
pub use scene::dielectric::Dielectric;
pub use scene::hitable::*;
pub use scene::hitable_list::*;
pub use scene::lambertian::Lambertian;
pub use scene::light_ray::LightRay;
pub use scene::material::Material;
pub use scene::metal::Metal;
pub use scene::sphere::Sphere;
pub use vector_math::ray::*;
pub use vector_math::vec3::*;

fn get_color(path: &LightRay, world: &HitableList, depth: u32, max_depth: u32) -> Vec3 {
    let sky_color = || {
        let unit_direction = path.ray.direction.normalized();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    };

    if depth >= max_depth {
        return Vec3::zero();
    }

    match world.scatter(path, 0.001, std::f32::MAX) {
        Some(new_path) => get_color(&new_path, &world, depth + 1, max_depth),
        None => {
            if depth < max_depth {
                path.color * sky_color()
            } else {
                Vec3::zero()
            }
        }
    }
}

fn test_world() -> HitableList {
    let mat1 = Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let mat2 = Box::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.8)));
    let mat3 = Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.2));
    let mat4 = Box::new(Dielectric::new(1.5));

    let sphere1 = Box::new(Sphere::new(Vec3::new(0.0, -100.5, 0.0), 100.0, mat1));
    let sphere2 = Box::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 0.5, mat2));
    let sphere3 = Box::new(Sphere::new(Vec3::new(1.0, 0.0, 0.0), 0.5, mat3));
    let sphere4 = Box::new(Sphere::new(Vec3::new(-1.0, 0.0, 0.0), 0.5, mat4));

    HitableList {
        list: vec![sphere1, sphere2, sphere3, sphere4],
    }
}

fn test_camera(aspect: f32) -> Camera {
    let origin = Vec3::new(3.0, 3.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let fov = 20.0;
    let dist_to_focus = (origin - look_at).magnitude();
    let aperture = 2.0;
    Camera::new(origin, look_at, up, fov, aspect, aperture, dist_to_focus)
}

fn render(width: u32, height: u32, sample_count: u32, max_depth: u32, world: &HitableList, camera: &Camera) -> ImageDataRGB {
    let mut image = ImageDataRGB::new(width, height);

    let image_width = image.width as f32;
    let image_height = image.height as f32;

    let mut rng = thread_rng();

    for j in 0..image.height {
        println!("{}%", (100.0 * j as f32 / sample_count as f32) as u32);

        for i in 0..image.width {
            let mut color = Vec3::zero();

            for _ in 0..sample_count {
                let u = (i as f32 + rng.gen::<f32>()) / image_width;
                let v = (j as f32 + rng.gen::<f32>()) / image_height;

                let r = camera.get_ray(u, v);
                let path = LightRay::new(r, Vec3::new(1.0, 1.0, 1.0));
                color += get_color(&path, &world, 0, max_depth);
            }

            color /= sample_count as f32;
            color = Vec3::new(f32::sqrt(color.x), f32::sqrt(color.y), f32::sqrt(color.z));

            let r = (255.99 * color.x) as u8;
            let g = (255.99 * color.y) as u8;
            let b = (255.99 * color.z) as u8;

            image.set_pixel((i, j), (r, g, b));
        }
    }

    println!("100%");

    image
}

fn main() {
    let width = 200;
    let height = 100;
    let aspect = width as f32 / height as f32;
    let sample_count = 100;
    let max_depth = 50;

    let world = test_world();
    let camera = test_camera(aspect);

    let image = render(width, height, sample_count, max_depth, &world, &camera);
    let image_name = "output/image.png";
    let result = image.save(image_name);
    match result {
        Ok(()) => println!("Image saved to {}", image_name),
        Err(err) => println!("{:?}", err),
    }
}
