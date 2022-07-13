#![allow(unused_variables)]
#![allow(unused_assignments)]
pub use crate::aabb::Aabb;
pub use crate::hittable::Hit;
pub use crate::hittable::Hitrecord;
pub use crate::hittable_list::Hittablelist;
pub use crate::hittable_list::Object;
pub use crate::material::Material;
pub use crate::ray::Ray;
pub use crate::sphere::Boundingbox;
pub use crate::vec3::Color;
pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;

pub struct Translate {
    ptr: Option<Box<Object>>,
    offset: Vec3,
}

impl Translate {
    pub fn default_new() -> Translate {
        Translate {
            ptr: None,
            offset: Vec3::default_new(),
        }
    }

    pub fn new(p: &Option<Box<Object>>, displacement: &Vec3) -> Translate {
        Translate {
            ptr: p
                .as_ref()
                .map(|in_p| Box::new(in_p.copy()))
                .map(|in_p| Box::new(in_p.copy())),
            offset: displacement.copy(),
        }
    }

    pub fn copy(&self) -> Translate {
        Translate {
            ptr: self
                .ptr
                .as_ref()
                .map(|in_p| Box::new(in_p.copy()))
                .map(|in_p| Box::new(in_p.copy())),
            offset: self.offset.copy(),
        }
    }
}

impl Hit for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Hitrecord) -> bool {
        let move_r = Ray::new(&(r.origin() - self.offset.copy()), &r.direction(), r.time());
        if let Some(in_ptr) = &self.ptr {
            if !in_ptr.hit(&move_r, t_min, t_max, rec) {
                return false;
            }
            rec.p += self.offset.copy();
            rec.set_face_normal(&move_r, &rec.normal.copy());
        }

        true
    }
}

impl Boundingbox for Translate {
    fn boundingbox(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        if let Some(in_ptr) = &self.ptr {
            if !in_ptr.boundingbox(time0, time1, output_box) {
                return false;
            }

            *output_box = Aabb::new(
                &(output_box.min() + self.offset.copy()),
                &(output_box.max() + self.offset.copy()),
            );
        }
        true
    }
}
