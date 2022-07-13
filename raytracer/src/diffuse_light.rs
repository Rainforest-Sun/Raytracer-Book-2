#![allow(warnings, unused)]
pub use crate::hittable::Hitrecord;
use crate::material::Emitted;
pub use crate::material::Material;
pub use crate::material::Scatter;
pub use crate::ray::Ray;
pub use crate::solidcolor::Solidcolor;
pub use crate::texture::Texture;
pub use crate::texture::Value;
pub use crate::vec3::Color;
pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;

pub struct Diffuselight {
    pub emit: Option<Box<Texture>>,
}

impl Diffuselight {
    pub fn default_new() -> Diffuselight {
        Diffuselight { emit: None }
    }

    pub fn new(a: &Option<Box<Texture>>) -> Diffuselight {
        Diffuselight {
            emit: a
                .as_ref()
                .map(|in_a| Box::new(in_a.copy()))
                .map(|in_a| Box::new(in_a.copy())),
        }
    }

    pub fn new_from_color(c: &Color) -> Diffuselight {
        Diffuselight {
            emit: Some(Box::new(Texture::Solidcolor(Solidcolor::new_from_color(
                &c.copy(),
            )))),
        }
    }

    pub fn copy(&self) -> Diffuselight {
        Diffuselight {
            emit: self
                .emit
                .as_ref()
                .map(|in_a| Box::new(in_a.copy()))
                .map(|in_a| Box::new(in_a.copy())),
        }
    }
}

impl Scatter for Diffuselight {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &Hitrecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        false
    }
}

impl Emitted for Diffuselight {
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        if let Some(in_emitted) = &self.emit {
            return in_emitted.value(u, v, &p);
        }
        return Color::new(0.0, 0.0, 0.0);
    }
}
