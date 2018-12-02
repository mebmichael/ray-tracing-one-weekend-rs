mod image_wrapper;
mod vector_math;

pub use image_wrapper::ImageDataRGB;
pub use vector_math::vec3::Vec3;
pub use vector_math::ray;

fn main() {
    let mut image = ImageDataRGB::new(200, 100);

    let image_width = image.width as f32;
    let image_height = image.height as f32;

    for j in 0..image.height {
        for i in 0..image.width {
            let color = Vec3 {
                x: (i as f32) / image_width,
                y: (image_height - j as f32 - 1.0) / image_height,
                z: 0.2,
            };

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
