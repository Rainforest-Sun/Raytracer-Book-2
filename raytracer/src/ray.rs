pub use crate::vec3::Color;
pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
    pub tm: f64,
}

impl Ray {
    pub fn default_new() -> Ray {
        Ray {
            orig: Point3::default_new(),
            dir: Vec3::default_new(),
            tm: 0.0,
        }
    }

    pub fn new(origin: &Vec3, direction: &Vec3, time: f64) -> Ray {
        Ray {
            orig: origin.copy(),
            dir: direction.copy(),
            tm: time,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig.copy()
    }

    pub fn direction(&self) -> Vec3 {
        self.dir.copy()
    }

    pub fn time(&self) -> f64 {
        self.tm
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig.copy() + self.dir.copy() * t
    }
}
