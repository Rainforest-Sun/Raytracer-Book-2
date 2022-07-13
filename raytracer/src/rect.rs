#![allow(unused_variables)]
#![allow(unused_assignments)]
pub use crate::aabb::Aabb;
pub use crate::hittable::Hit;
pub use crate::hittable::Hitrecord;
pub use crate::hittable_list::Hittablelist;
pub use crate::material::Material;
pub use crate::ray::Ray;
pub use crate::sphere::Boundingbox;
pub use crate::vec3::Color;
pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;

pub struct XYrect {
    pub mp: Option<Box<Material>>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}

pub struct XZrect {
    pub mp: Option<Box<Material>>,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

pub struct YZrect {
    pub mp: Option<Box<Material>>,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl XYrect {
    pub fn default_new() -> XYrect {
        XYrect {
            mp: None,
            x0: 0.0,
            x1: 0.0,
            y0: 0.0,
            y1: 0.0,
            k: 0.0,
        }
    }

    pub fn new(
        mat: &Option<Box<Material>>,
        _x0: f64,
        _x1: f64,
        _y0: f64,
        _y1: f64,
        _k: f64,
    ) -> XYrect {
        XYrect {
            mp: mat
                .as_ref()
                .map(|in_mp| Box::new(in_mp.copy()))
                .map(|in_mp| Box::new(in_mp.copy())),
            x0: _x0,
            x1: _x1,
            y0: _y0,
            y1: _y1,
            k: _k,
        }
    }

    pub fn copy(&self) -> XYrect {
        XYrect {
            mp: self
                .mp
                .as_ref()
                .map(|in_mp| Box::new(in_mp.copy()))
                .map(|in_mp| Box::new(in_mp.copy())),
            x0: self.x0,
            x1: self.x1,
            y0: self.y0,
            y1: self.y1,
            k: self.k,
        }
    }
}

impl XZrect {
    pub fn default_new() -> XZrect {
        XZrect {
            mp: None,
            x0: 0.0,
            x1: 0.0,
            z0: 0.0,
            z1: 0.0,
            k: 0.0,
        }
    }

    pub fn new(
        mat: &Option<Box<Material>>,
        _x0: f64,
        _x1: f64,
        _z0: f64,
        _z1: f64,
        _k: f64,
    ) -> XZrect {
        XZrect {
            mp: mat
                .as_ref()
                .map(|in_mp| Box::new(in_mp.copy()))
                .map(|in_mp| Box::new(in_mp.copy())),
            x0: _x0,
            x1: _x1,
            z0: _z0,
            z1: _z1,
            k: _k,
        }
    }

    pub fn copy(&self) -> XZrect {
        XZrect {
            mp: self
                .mp
                .as_ref()
                .map(|in_mp| Box::new(in_mp.copy()))
                .map(|in_mp| Box::new(in_mp.copy())),
            x0: self.x0,
            x1: self.x1,
            z0: self.z0,
            z1: self.z1,
            k: self.k,
        }
    }
}

impl YZrect {
    pub fn default_new() -> YZrect {
        YZrect {
            mp: None,
            y0: 0.0,
            y1: 0.0,
            z0: 0.0,
            z1: 0.0,
            k: 0.0,
        }
    }

    pub fn new(
        mat: &Option<Box<Material>>,
        _y0: f64,
        _y1: f64,
        _z0: f64,
        _z1: f64,
        _k: f64,
    ) -> YZrect {
        YZrect {
            mp: mat
                .as_ref()
                .map(|in_mp| Box::new(in_mp.copy()))
                .map(|in_mp| Box::new(in_mp.copy())),
            y0: _y0,
            y1: _y1,
            z0: _z0,
            z1: _z1,
            k: _k,
        }
    }

    pub fn copy(&self) -> YZrect {
        YZrect {
            mp: self
                .mp
                .as_ref()
                .map(|in_mp| Box::new(in_mp.copy()))
                .map(|in_mp| Box::new(in_mp.copy())),
            y0: self.y0,
            y1: self.y1,
            z0: self.z0,
            z1: self.z1,
            k: self.k,
        }
    }
}

impl Hit for XYrect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Hitrecord) -> bool {
        let t = (self.k - r.origin().z()) / r.direction().z();
        if t < t_min || t > t_max {
            return false;
        }
        let x = r.origin().x() + t * r.direction().x();
        let y = r.origin().y() + t * r.direction().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(&r, &outward_normal);
        rec.mat_ptr = self
            .mp
            .as_ref()
            .map(|in_mat_ptr| Box::new(in_mat_ptr.copy()))
            .map(|in_mat_ptr| Box::new(in_mat_ptr.copy()));
        rec.p = r.at(t);
        true
    }
}

impl Boundingbox for XYrect {
    fn boundingbox(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(
            &Point3::new(self.x0, self.y0, self.k - 0.0001),
            &Point3::new(self.x1, self.y1, self.k + 0.0001),
        );
        true
    }
}

impl Hit for XZrect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Hitrecord) -> bool {
        let t = (self.k - r.origin().y()) / r.direction().y();
        if t < t_min || t > t_max {
            return false;
        }
        let x = r.origin().x() + t * r.direction().x();
        let z = r.origin().z() + t * r.direction().z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        rec.set_face_normal(&r, &outward_normal);
        rec.mat_ptr = self
            .mp
            .as_ref()
            .map(|in_mat_ptr| Box::new(in_mat_ptr.copy()))
            .map(|in_mat_ptr| Box::new(in_mat_ptr.copy()));
        rec.p = r.at(t);
        true
    }
}

impl Boundingbox for XZrect {
    fn boundingbox(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(
            &Point3::new(self.x0, self.k - 0.0001, self.z0),
            &Point3::new(self.x1, self.k + 0.0001, self.z1),
        );
        true
    }
}

impl Hit for YZrect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Hitrecord) -> bool {
        let t = (self.k - r.origin().x()) / r.direction().x();
        if t < t_min || t > t_max {
            return false;
        }
        let y = r.origin().y() + t * r.direction().y();
        let z = r.origin().z() + t * r.direction().z();
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return false;
        }
        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        rec.set_face_normal(&r, &outward_normal);
        rec.mat_ptr = self
            .mp
            .as_ref()
            .map(|in_mat_ptr| Box::new(in_mat_ptr.copy()))
            .map(|in_mat_ptr| Box::new(in_mat_ptr.copy()));
        rec.p = r.at(t);
        true
    }
}

impl Boundingbox for YZrect {
    fn boundingbox(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(
            &Point3::new(self.k - 0.0001, self.y0, self.z0),
            &Point3::new(self.k + 0.0001, self.y1, self.z1),
        );
        true
    }
}
