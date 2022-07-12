#![allow(unused_variables)]
pub use crate::perlin::Perlin;
pub use crate::texture::Value;
pub use crate::vec3::Color;
pub use crate::vec3::Point3;

pub struct Noisetexture {
    noise: Perlin,
    scale: f64,
}

impl Noisetexture {
    pub fn default_new() -> Noisetexture {
        Noisetexture {
            noise: Perlin::default_new(),
            scale: 0.0,
        }
    }

    pub fn new(sc: f64) -> Noisetexture {
        Noisetexture {
            noise: Perlin::default_new(),
            scale: sc,
        }
    }

    pub fn copy(&self) -> Noisetexture {
        Noisetexture {
            noise: self.noise.copy(),
            scale: self.scale,
        }
    }
}

impl Value for Noisetexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        //Color::new(1.0, 1.0, 1.0) * self.noise.turb(&(p.copy() * self.scale), 7)
        Color::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (10.0 * (self.noise.turb(&p.copy(), 7)) + self.scale * p.z()).sin())
    }
}
