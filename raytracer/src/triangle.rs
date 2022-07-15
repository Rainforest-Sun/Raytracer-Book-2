#![allow(warnings, unused)]
pub use std::f64::{INFINITY, NEG_INFINITY};

pub use crate::aabb::Aabb;
pub use crate::hittable::Boundingbox;
pub use crate::hittable::Hit;
pub use crate::hittable::Hitrecord;
pub use crate::material::Material;
pub use crate::ray::Ray;
pub use crate::vec3::Color;
pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;

pub struct Triangle {
    pub ver: [Point3; 3], // 3 vertices of triangle
    pub normal: Vec3,
    pub mat: Option<Box<Material>>,
    pub cen: Point3, // 三角形中心
    pub area: f64,
    pub v: Vec3,
    pub w: Vec3,
    pub ab: Vec3,
    pub ac: Vec3,
}

impl Triangle {
    pub fn new(ver: &[Point3; 3], mat: &Option<Box<Material>>) -> Triangle {
        let normal = (Vec3::cross(&ver[0], &ver[1])
            + Vec3::cross(&ver[1], &ver[2])
            + Vec3::cross(&ver[2], &ver[0]))
        .unit_vector();
        let cen = (ver[0].copy() + ver[1].copy() + ver[2].copy()) / 3.;

        let l0 = ((ver[0][0] - ver[1][0]).powi(2)
            + (ver[0][1] - ver[1][1]).powi(2)
            + (ver[0][2] - ver[1][2]).powi(2))
        .sqrt();
        let l1 = ((ver[1][0] - ver[2][1]).powi(2)
            + (ver[1][1] - ver[2][1]).powi(2)
            + (ver[1][2] - ver[2][2]).powi(2))
        .sqrt();
        let l2 = ((ver[2][0] - ver[0][0]).powi(2)
            + (ver[2][1] - ver[0][1]).powi(2)
            + (ver[2][2] - ver[0][2]).powi(2))
        .sqrt();
        let p = (l0 + l1 + l2) / 2.;
        let area = (p * (p - l0) * (p - l1) * (p - l2)).sqrt();

        let mut v = Vec3::cross(&normal, &(ver[1] - ver[0]));
        v /= Vec3::dot(&(ver[2] - ver[0]), &v);
        let mut w = Vec3::cross(&normal, &(ver[2] - ver[0]));
        w /= Vec3::dot(&(ver[1] - ver[0]), &w);

        Triangle {
            ver: [ver[0].copy(), ver[1].copy(), ver[2].copy()],
            normal,
            mat: mat
                .as_ref()
                .map(|in_mat_ptr| Box::new(in_mat_ptr.copy()))
                .map(|in_mat_ptr| Box::new(in_mat_ptr.copy())),
            area,
            cen,
            v,
            w,
            ab: ver[1] - ver[0],
            ac: ver[2] - ver[0],
        }
    }

    pub fn copy(&self) -> Triangle {
        let ver: [Point3; 3] = [self.ver[0].copy(), self.ver[1].copy(), self.ver[2].copy()];
        Triangle {
            ver,
            normal: self.normal.copy(),
            mat: self
                .mat
                .as_ref()
                .map(|in_mat| Box::new(in_mat.copy()))
                .map(|in_mat| Box::new(in_mat.copy())),
            cen: self.cen.copy(),
            area: self.area,
            v: self.v.copy(),
            w: self.w.copy(),
            ab: self.ab.copy(),
            ac: self.ac.copy(),
        }
    }
}

impl Hit for Triangle {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Hitrecord) -> bool {
        let orig = r.origin();
        let dir = r.direction();
        let n = self.normal.copy();
        let cen = self.cen.copy();
        let t = ((cen.x() - orig.x()) * n.x()
            + (cen.y() - orig.y()) * n.y()
            + (cen.z() - orig.z()) * n.z())
            / (dir.x() * n.x() + dir.y() * n.y() + dir.z() * n.z());
        if t.is_nan() || t < t_min || t > t_max {
            // 直线与三角形平行
            return false;
        }

        let ap = (orig + dir * t) - self.ver[0];
        let gamma = Vec3::dot(&ap, &self.v);
        if gamma.is_sign_positive() && gamma < 1. {
            let beta = Vec3::dot(&ap, &self.w);
            if beta.is_sign_positive() && beta < 1. {
                let alpha = 1. - gamma - beta;
                if alpha.is_sign_positive() && alpha < 1. {
                    rec.t = t;
                    rec.p = r.at(rec.t);
                    let outward_normal = n.unit_vector();
                    rec.set_face_normal(&r, &outward_normal);
                    rec.mat_ptr = self
                        .mat
                        .as_ref()
                        .map(|in_mat| Box::new(in_mat.copy()))
                        .map(|in_mat| Box::new(in_mat.copy()));
                    rec.u = alpha;
                    rec.v = beta;
                    return true;
                }
            }
        }
        false
    }
}

impl Boundingbox for Triangle {
    fn boundingbox(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY);

        for v in self.ver {
            for i in 0..3 {
                if v[i] < min[i] {
                    min[i] = v[i];
                }
                if v[i] > max[i] {
                    max[i] = v[i];
                }
            }
        }
        *output_box = Aabb::new(&min, &max);
        true
    }
}
