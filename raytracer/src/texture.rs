#![allow(clippy::large_enum_variant)]
pub use crate::checker_texture::Checkertexture;
pub use crate::image_texture::Imagetexture;
pub use crate::noise_texture::Noisetexture;
pub use crate::perlin::Perlin;
pub use crate::solidcolor::Solidcolor;
pub use crate::vec3::Color;
pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;

pub enum Texture {
    Solidcolor(Solidcolor),
    Checkertexture(Checkertexture),
    Noisetexture(Noisetexture),
    Imagetexture(Imagetexture),
}

pub trait Value {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

impl Texture {
    pub fn copy(&self) -> Texture {
        match &self {
            Texture::Solidcolor(solidcolor) => Texture::Solidcolor(solidcolor.copy()),
            Texture::Checkertexture(checkertexture) => {
                Texture::Checkertexture(checkertexture.copy())
            }
            Texture::Noisetexture(noisetexture) => Texture::Noisetexture(noisetexture.copy()),
            Texture::Imagetexture(imagetexture) => Texture::Imagetexture(imagetexture.copy()),
        }
    }
}

impl Value for Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        match &self {
            Texture::Solidcolor(solidcolor) => Solidcolor::value(&solidcolor, u, v, &p),
            Texture::Checkertexture(checkertexture) => {
                Checkertexture::value(&checkertexture, u, v, &p)
            }
            Texture::Noisetexture(noisetexture) => Noisetexture::value(&noisetexture, u, v, &p),
            Texture::Imagetexture(imagetexture) => Imagetexture::value(&imagetexture, u, v, &p),
        }
    }
}
