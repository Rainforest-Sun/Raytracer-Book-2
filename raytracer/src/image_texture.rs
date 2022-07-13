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

pub struct Imagetexture {
    data: RgbImage,
    width: i32,
    height: i32,
    bytes_per_scanline: i32,
}

impl Imagetexture {
    pub fn new(filename: &str) -> Imagetexture {
        let img = image::open(filename).unwrap().to_rgb8();
        let (width, height) = img.dimensions();
        let width = width as i32;
        let height = height as i32;
        let bytes_per_scanline = BYTES_PER_PIXEL * width;

        Imagetexture {
            data: img,
            width,
            height,
            bytes_per_scanline,
        }
    }

    pub fn copy(&self) -> Imagetexture {
        Imagetexture {
            width: self.width,
            height: self.height,
            data: self.data.clone(),
            bytes_per_scanline: self.bytes_per_scanline,
        }
    }
}

impl Value for Imagetexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let u = color::clamp(u, 0.0, 1.0);
        let v = 1.0 - color::clamp(v, 0.0, 1.0);

        let mut i = (u * (self.width as f64)) as i32;
        let mut j = (v * (self.height as f64)) as i32;

        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }

        let color_scale = 1.0 / 255.0;

        //Color::new((self.data.get_pixel(i as u32,j as u32) as f64)*color_scale,self.data.get_pixel(i as u32,j as u32)*color_scale,self.data.get_pixel(i as u32,j as u32)*color_scale)
        Color::new(
            (self.data.get_pixel(i as u32, j as u32)[0] as f64) * color_scale,
            (self.data.get_pixel(i as u32, j as u32)[1] as f64) * color_scale,
            (self.data.get_pixel(i as u32, j as u32)[2] as f64) * color_scale,
        )
    }
}
