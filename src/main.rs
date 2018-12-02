mod image_wrapper;

use image_wrapper::ImageDataRGB;

fn main() {

    let mut image = ImageDataRGB::new(200, 100);

    for j in 0..image.height {
        for i in 0..image.width {
            
            let r = (i as f32) / (image.width as f32);
            let g = ((image.height - 1 - j) as f32) / (image.height as f32);
            let b: f32 = 0.2;

            let ir = (255.99 * r) as u8;
            let ig = (255.99 * g) as u8;
            let ib = (255.99 * b) as u8;

            image.set_pixel((i, j), (ir, ig, ib));
        }
    }

    let image_name = "output/image.png";
    let result = image.save(image_name);
    match result {
        Ok(()) => println!("Image saved to {}", image_name),
        Err(err) => println!("{:?}", err)
    }
}
