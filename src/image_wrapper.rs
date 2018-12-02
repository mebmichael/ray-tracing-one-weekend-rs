extern crate image;

use self::image::ColorType;
use self::image::png::PNGEncoder;
use std::fs::File;

pub struct ImageDataRGB {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>
}

impl ImageDataRGB {
    pub fn new(width: u32, height: u32) -> Self {
        let pixels_len = (width * height * 3) as usize;
        ImageDataRGB{
            width, height,
            pixels: vec!{0; pixels_len}
        }
    }

    pub fn set_pixel(&mut self, pixel: (u32, u32), rgb: (u8, u8, u8)) {
        if pixel.0 > self.width || pixel.1 > self.height {
            return;
        }

        let y = self.height - pixel.1 - 1;

        let offset = (pixel.0 + y * self.width) as usize * 3;
        self.pixels[offset] = rgb.0;
        self.pixels[offset + 1] = rgb.1;
        self.pixels[offset + 2] = rgb.2;
    }

    pub fn save(&self, filename: &str) -> Result<(), std::io::Error> {
        let file = File::create(filename)?;

        let encoder = PNGEncoder::new(file);
        encoder.encode(&self.pixels, self.width, self.height, ColorType::RGB(8))?;

        Ok(())
    }
}
