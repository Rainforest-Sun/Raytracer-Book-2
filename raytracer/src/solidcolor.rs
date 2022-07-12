#![allow(warnings, unused)]
pub use crate::texture::Value;
pub use crate::vec3::Color;
pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;

pub struct Solidcolor {
    pub color_value: Color,
}

impl Solidcolor {
    pub fn default_new() -> Solidcolor {
        Solidcolor {
            color_value: Color::default_new(),
        }
    }

    pub fn new_from_color(c: &Color) -> Solidcolor {
        Solidcolor {
            color_value: c.copy(),
        }
    }

    pub fn new_from_rgb(red: f64, green: f64, blue: f64) -> Solidcolor {
        Solidcolor {
            color_value: Color::new(red, green, blue),
        }
    }

    pub fn copy(&self) -> Solidcolor {
        Solidcolor {
            color_value: self.color_value.copy(),
        }
    }
}

impl Value for Solidcolor {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.color_value.copy()
    }
}
