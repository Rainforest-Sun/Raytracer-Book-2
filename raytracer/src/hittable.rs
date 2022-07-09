#![allow(clippy::manual_map)]
pub use crate::material::Material;
pub use crate::ray::Ray;
pub use crate::vec3::Color;
pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;

pub struct Hitrecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: Option<Box<Material>>,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hit {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Hitrecord) -> bool;
}

impl Hitrecord {
    pub fn default_new() -> Hitrecord {
        Hitrecord {
            p: Point3::default_new(),
            normal: Vec3::default_new(),
            t: 0.0,
            front_face: false,
            mat_ptr: None,
        }
    }

    pub fn copy_new(rhs: &Hitrecord) -> Hitrecord {
        Hitrecord {
            p: rhs.p.copy(),
            normal: rhs.normal.copy(),
            t: rhs.t,
            front_face: rhs.front_face,
            mat_ptr: rhs
                .mat_ptr
                .as_ref()
                .map(|in_mat_ptr| Box::new(in_mat_ptr.copy()))
                .map(|in_mat_ptr| Box::new(in_mat_ptr.copy())),
        }
    }

    pub fn copy(&self) -> Hitrecord {
        Hitrecord {
            p: self.p.copy(),
            normal: self.normal.copy(),
            t: self.t,
            front_face: self.front_face,
            mat_ptr: self
                .mat_ptr
                .as_ref()
                .map(|in_mat_ptr| Box::new(in_mat_ptr.copy()))
                .map(|in_mat_ptr| Box::new(in_mat_ptr.copy())),
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&r.direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal.copy()
        } else {
            -outward_normal.copy()
        };
    }
}
