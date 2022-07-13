#![allow(unused_variables)]
#![allow(unused_assignments)]
pub use crate::aabb::Aabb;
pub use crate::func;
pub use crate::hittable::Hit;
pub use crate::hittable::Hitrecord;
pub use crate::hittable_list::Hittablelist;
pub use crate::hittable_list::Object;
use crate::isotropic::Isotropic;
pub use crate::material::Material;
pub use crate::rand;
pub use crate::ray::Ray;
pub use crate::sphere::Boundingbox;
pub use crate::texture::Texture;
pub use crate::vec3::Color;
pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;

pub struct ConstantMedium {
    boundary: Option<Box<Object>>,
    phase_function: Option<Box<Material>>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn default_new() -> ConstantMedium {
        ConstantMedium {
            boundary: None,
            phase_function: None,
            neg_inv_density: 0.0,
        }
    }

    pub fn new_from_ptr(
        b: &Option<Box<Object>>,
        d: f64,
        a: &Option<Box<Texture>>,
    ) -> ConstantMedium {
        ConstantMedium {
            boundary: b
                .as_ref()
                .map(|in_b| Box::new(in_b.copy()))
                .map(|in_b| Box::new(in_b.copy())),
            phase_function: Some(Box::new(Material::Isotropic(Isotropic::new_from_ptr(&a)))),
            neg_inv_density: -1.0 / d,
        }
    }

    pub fn new_from_color(b: &Option<Box<Object>>, d: f64, c: &Color) -> ConstantMedium {
        ConstantMedium {
            boundary: b
                .as_ref()
                .map(|in_b| Box::new(in_b.copy()))
                .map(|in_b| Box::new(in_b.copy())),
            phase_function: Some(Box::new(Material::Isotropic(Isotropic::new_from_color(&c)))),
            neg_inv_density: -1.0 / d,
        }
    }

    pub fn copy(&self) -> ConstantMedium {
        ConstantMedium {
            boundary: self
                .boundary
                .as_ref()
                .map(|in_b| Box::new(in_b.copy()))
                .map(|in_b| Box::new(in_b.copy())),
            phase_function: self
                .phase_function
                .as_ref()
                .map(|in_a| Box::new(in_a.copy()))
                .map(|in_a| Box::new(in_a.copy())),
            neg_inv_density: self.neg_inv_density,
        }
    }
}

impl Hit for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Hitrecord) -> bool {
        let enable_debug = false;
        let debugging = enable_debug && rand::random_double() < 0.00001;

        let mut rec1 = Hitrecord::default_new();
        let mut rec2 = Hitrecord::default_new();

        if let Some(in_b) = &self.boundary {
            if !in_b.hit(&r, -std::f64::INFINITY, std::f64::INFINITY, &mut rec1) {
                return false;
            }
            if !in_b.hit(&r, rec1.t + 0.0001, std::f64::INFINITY, &mut rec2) {
                return false;
            }
        }
        if debugging {
            eprint!("\nt_min={}, t_max={}", rec1.t, rec2.t);
        }

        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * (rand::random_double().ln());

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.mat_ptr = self
            .phase_function
            .as_ref()
            .map(|in_p| Box::new(in_p.copy()))
            .map(|in_p| Box::new(in_p.copy()));

        true
    }
}

impl Boundingbox for ConstantMedium {
    fn boundingbox(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        if let Some(in_b) = &self.boundary {
            return in_b.boundingbox(time0, time1, output_box);
        }
        false
    }
}
