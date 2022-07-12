#![allow(warnings, unused)]
pub use crate::func;
pub use crate::hittable::Hit;
pub use crate::hittable::Hitrecord;
pub use crate::material::Material;
pub use crate::ray::Ray;
pub use crate::vec3::Color;
pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;

pub struct Aabb {
    minimum: Point3,
    maximum: Point3,
}

impl Aabb {
    pub fn default_new() -> Aabb {
        Aabb {
            minimum: Point3::default_new(),
            maximum: Point3::default_new(),
        }
    }

    pub fn new(a: &Point3, b: &Point3) -> Aabb {
        Aabb {
            minimum: a.copy(),
            maximum: b.copy(),
        }
    }

    pub fn copy(&self) -> Aabb {
        Aabb {
            minimum: self.minimum.copy(),
            maximum: self.maximum.copy(),
        }
    }

    pub fn min(&self) -> Point3 {
        self.minimum.copy()
    }

    pub fn max(&self) -> Point3 {
        self.maximum.copy()
    }

    pub fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Aabb {
        let small = Point3::new(
            func::fmin(box0.min().x(), box1.min().x()),
            func::fmin(box0.min().y(), box1.min().y()),
            func::fmin(box0.min().z(), box1.min().z()),
        );
        let big = Point3::new(
            func::fmax(box0.max().x(), box1.max().x()),
            func::fmax(box0.max().y(), box1.max().y()),
            func::fmax(box0.max().z(), box1.max().z()),
        );
        Aabb::new(&small, &big)
    }
}

impl Hit for Aabb {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Hitrecord) -> bool {
        for a in 0..3 {
            let invd = 1.0 / r.direction()[a];
            let mut t0 = (self.min()[a] - r.origin()[a]) * invd;
            let mut t1 = (self.max()[a] - r.origin()[a]) * invd;
            if invd < 0.0 {
                let tmp = t0;
                t0 = t1;
                t1 = tmp;
            }
            let t_min = func::fmax(t0, t_min);
            let t_max = func::fmin(t1, t_max);
            if t_max <= t_min {
                return false;
            }
        }
        return true;
    }
}
