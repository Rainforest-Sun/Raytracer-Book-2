pub use crate::camera::Camera;
pub use crate::hittable::Hit;
pub use crate::hittable::Hitrecord;
pub use crate::hittable_list::Hittablelist;
pub use crate::hittable_list::Object;
pub use crate::material::Scatter;
pub use crate::ray::Ray;
pub use crate::sphere::Sphere;
pub use crate::vec3::Vec3;
pub use crate::vec3::Color;
pub use crate::vec3::Point3;

pub struct Lambertian {
    pub albedo: Color,
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

        *scattered = Ray::new(&rec.p.copy(), &scatter_direction.copy());
        *attenuation = self.albedo.copy();
        true
    }
}

impl Lambertian {
    pub fn new(a: &Color) -> Lambertian {
        Lambertian { albedo: a.copy() }
    }

    pub fn copy(&self) -> Lambertian {
        Lambertian {
            albedo: self.albedo.copy(),
        }
    }
}
