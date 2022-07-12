#![allow(clippy::collapsible_else_if)]
pub use crate::solidcolor::Solidcolor;
pub use crate::texture::Texture;
pub use crate::texture::Value;
pub use crate::vec3::Color;
pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;

pub struct Checkertexture {
    pub odd: Option<Box<Texture>>,
    pub even: Option<Box<Texture>>,
}

impl Checkertexture {
    pub fn default_new() -> Checkertexture {
        Checkertexture {
            odd: None,
            even: None,
        }
    }

    pub fn new_from_ptr(
        _even: &Option<Box<Texture>>,
        _odd: &Option<Box<Texture>>,
    ) -> Checkertexture {
        Checkertexture {
            even: _even
                .as_ref()
                .map(|in_even| Box::new(in_even.copy()))
                .map(|in_even| Box::new(in_even.copy())),
            odd: _odd
                .as_ref()
                .map(|in_odd| Box::new(in_odd.copy()))
                .map(|in_odd| Box::new(in_odd.copy())),
        }
    }

    pub fn new_from_color(c1: &Color, c2: &Color) -> Checkertexture {
        Checkertexture {
            even: Some(Box::new(Texture::Solidcolor(Solidcolor::new_from_color(
                &c1,
            )))),
            odd: Some(Box::new(Texture::Solidcolor(Solidcolor::new_from_color(
                &c2,
            )))),
        }
    }

    pub fn copy(&self) -> Checkertexture {
        Checkertexture {
            even: self
                .even
                .as_ref()
                .map(|in_even| Box::new(in_even.copy()))
                .map(|in_even| Box::new(in_even.copy())),
            odd: self
                .odd
                .as_ref()
                .map(|in_odd| Box::new(in_odd.copy()))
                .map(|in_odd| Box::new(in_odd.copy())),
        }
    }
}

impl Value for Checkertexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = ((p.x() * 10.0).sin()) * ((p.y() * 10.0).sin()) * ((p.z() * 10.0).sin());
        if sines < 0.0 {
            if let Some(in_odd) = &self.odd {
                return in_odd.value(u, v, &p);
            }
        } else {
            if let Some(in_even) = &self.even {
                return in_even.value(u, v, &p);
            }
        }
        Color::new(0.0, 0.0, 0.0)
    }
}
