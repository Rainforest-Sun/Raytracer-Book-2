#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::clone_double_ref)]
extern crate image;

pub use crate::color;
pub use crate::vec3::Color;
pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;
pub use crate::Value;
pub use image::{imageops, DynamicImage, GenericImage, GenericImageView, ImageBuffer, RgbImage};
const BYTES_PER_PIXEL: i32 = 3;

pub struct Objtexture {
    data: RgbImage,
}

impl Objtexture {
    pub fn new(filename: &str) -> Objtexture {
        let img = image::open(filename).unwrap().to_rgb8();
        Objtexture { data: img }
    }

    pub fn copy(&self) -> Objtexture {
        Objtexture {
            data: self.data.clone(),
        }
    }
}

impl Value for Objtexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let color_scale = 1.0 / 255.0;

        Color::new(
            (self.data.get_pixel(u as u32, v as u32)[0] as f64) * color_scale,
            (self.data.get_pixel(u as u32, v as u32)[1] as f64) * color_scale,
            (self.data.get_pixel(u as u32, v as u32)[2] as f64) * color_scale,
        )
    }
}
