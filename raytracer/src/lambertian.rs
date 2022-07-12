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

pub struct Lambertian {
    pub albedo: Option<Box<Texture>>,
}

impl Scatter for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &Hitrecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal.copy() + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.copy();
        }

        *scattered = Ray::new(&rec.p.copy(), &scatter_direction.copy(), r_in.time());
        *attenuation = match &self.albedo {
            Some(in_albedo) => in_albedo.value(rec.u, rec.v, &rec.p),
            None => Color::new(0.0, 0.0, 0.0),
        };
        true
    }
}

impl Lambertian {
    pub fn new(a: &Color) -> Lambertian {
        Lambertian {
            albedo: Some(Box::new(Texture::Solidcolor(Solidcolor::new_from_color(
                &a,
            )))),
        }
    }

    pub fn new_from_ptr(a: &Option<Box<Texture>>) -> Lambertian {
        Lambertian {
            albedo: a
                .as_ref()
                .map(|in_a| Box::new(in_a.copy()))
                .map(|in_a| Box::new(in_a.copy())),
        }
    }

    pub fn copy(&self) -> Lambertian {
        Lambertian {
            albedo: self
                .albedo
                .as_ref()
                .map(|in_albedo| Box::new(in_albedo.copy()))
                .map(|in_albedo| Box::new(in_albedo.copy())),
        }
    }
}
