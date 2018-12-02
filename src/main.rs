mod image_wrapper;
mod vector_math;

pub use image_wrapper::ImageDataRGB;
pub use vector_math::vec3::Vec3;
pub use vector_math::ray::Ray;

fn color(r: &Ray) -> Vec3 {
    let unit_direction = r.direction.normalized();
    let t = 0.5 * (unit_direction.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let mut image = ImageDataRGB::new(200, 100);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let image_width = image.width as f32;
    let image_height = image.height as f32;

    for j in 0..image.height {
        for i in 0..image.width {

            let u = i as f32 / image_width;
            let v = j as f32 / image_height;

            let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let c = color(&r);

            let r = (255.99 * c.x) as u8;
            let g = (255.99 * c.y) as u8;
            let b = (255.99 * c.z) as u8;

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
