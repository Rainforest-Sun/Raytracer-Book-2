#![allow(warnings, unused)]
pub use crate::aabb::Aabb;
pub use crate::hittable::Boundingbox;
pub use crate::hittable::Hit;
pub use crate::hittable::Hitrecord;
pub use crate::material::Material;
pub use crate::ray::Ray;
pub use crate::vec3::Color;
pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Option<Box<Material>>,
}

impl Sphere {
    pub fn default_new() -> Sphere {
        Sphere {
            center: Point3::default_new(),
            radius: 0.0,
            mat_ptr: None,
        }
    }

    pub fn new(cen: &Point3, r: f64, m: &Option<Box<Material>>) -> Sphere {
        Sphere {
            center: cen.copy(),
            radius: r,
            mat_ptr: m
                .as_ref()
                .map(|in_mat_ptr| Box::new(in_mat_ptr.copy()))
                .map(|in_mat_ptr| Box::new(in_mat_ptr.copy())),
        }
    }

    pub fn copy(&self) -> Sphere {
        Sphere {
            center: self.center.copy(),
            radius: self.radius,
            mat_ptr: self
                .mat_ptr
                .as_ref()
                .map(|in_mat_ptr| Box::new(in_mat_ptr.copy()))
                .map(|in_mat_ptr| Box::new(in_mat_ptr.copy())),
        }
    }

    pub fn get_sphere_uv(p: &Point3, u: &mut f64, v: &mut f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + std::f64::consts::PI;

        *u = phi / (2.0 * std::f64::consts::PI);
        *v = theta / std::f64::consts::PI;
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Hitrecord) -> bool {
        let oc = r.origin() - self.center.copy();
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b.powf(2.0) - (a * c);
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p.copy() - self.center.copy()) / self.radius;
        rec.set_face_normal(&r, &outward_normal);
        Sphere::get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
        rec.mat_ptr = self
            .mat_ptr
            .as_ref()
            .map(|in_mat_ptr| Box::new(in_mat_ptr.copy()))
            .map(|in_mat_ptr| Box::new(in_mat_ptr.copy()));
        true
    }
}

impl Boundingbox for Sphere {
    fn boundingbox(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(
            &(self.center.copy() - Vec3::new(self.radius, self.radius, self.radius)),
            &(self.center.copy() + Vec3::new(self.radius, self.radius, self.radius)),
        );
        true
    }
}
