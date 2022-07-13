#![allow(warnings, unused)]
pub use crate::camera::Camera;
pub use crate::hittable::Hit;
pub use crate::hittable::Hitrecord;
pub use crate::hittable_list::Hittablelist;
pub use crate::hittable_list::Object;
pub use crate::material::Scatter;
pub use crate::movingsphere::Movingsphere;
pub use crate::ray::Ray;
pub use crate::solidcolor::Solidcolor;
pub use crate::sphere::Sphere;
pub use crate::texture::Texture;
pub use crate::texture::Value;
pub use crate::vec3::Color;
pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;

pub struct Isotropic {
    albedo: Option<Box<Texture>>,
}

impl Scatter for Isotropic {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &Hitrecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray::new(&rec.p.copy(), &Vec3::random_in_unit_sphere(), r_in.time());
        *attenuation = match &self.albedo {
            Some(in_albedo) => in_albedo.value(rec.u, rec.v, &rec.p),
            None => Color::new(0.0, 0.0, 0.0),
        };
        true
    }
}

impl Isotropic {
    pub fn new_from_color(a: &Color) -> Isotropic {
        Isotropic {
            albedo: Some(Box::new(Texture::Solidcolor(Solidcolor::new_from_color(
                &a,
            )))),
        }
    }

    pub fn new_from_ptr(a: &Option<Box<Texture>>) -> Isotropic {
        Isotropic {
            albedo: a
                .as_ref()
                .map(|in_a| Box::new(in_a.copy()))
                .map(|in_a| Box::new(in_a.copy())),
        }
    }

    pub fn copy(&self) -> Isotropic {
        Isotropic {
            albedo: self
                .albedo
                .as_ref()
                .map(|in_albedo| Box::new(in_albedo.copy()))
                .map(|in_albedo| Box::new(in_albedo.copy())),
        }
    }
}
