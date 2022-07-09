pub use crate::hittable::Hit;
pub use crate::hittable::Hitrecord;
pub use crate::material::Material;
pub use crate::ray::Ray;
pub use crate::vec3::Vec3;
pub use crate::vec3::Color;
pub use crate::vec3::Point3;

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
            mat_ptr: match m {
                Some(in_mat_ptr) => Some(Box::new(in_mat_ptr.copy())),
                None => None,
            },
        }
    }

    pub fn copy(&self) -> Sphere {
        Sphere {
            center: self.center.copy(),
            radius: self.radius,
            mat_ptr: match &self.mat_ptr {
                Some(in_mat_ptr) => Some(Box::new(in_mat_ptr.copy())),
                None => None,
            },
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Hitrecord) -> bool {
        let oc = r.origin() - self.center.copy();
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = (half_b * half_b) - (a * c);
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
        rec.mat_ptr = match &self.mat_ptr {
            Some(in_mat_ptr) => Some(Box::new(in_mat_ptr.copy())),
            None => None,
        };
        true
    }
}
