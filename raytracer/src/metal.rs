pub use crate::camera::Camera;
pub use crate::hittable::Hit;
pub use crate::hittable::Hitrecord;
pub use crate::hittable_list::Hittablelist;
pub use crate::hittable_list::Object;
pub use crate::material::Scatter;
pub use crate::movingsphere::Movingsphere;
pub use crate::ray::Ray;
pub use crate::sphere::Sphere;
pub use crate::vec3::Color;
pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Scatter for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &Hitrecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(&Vec3::unit_vector(&r_in.direction()), &rec.normal);
        *scattered = Ray::new(
            &rec.p,
            &(reflected + Vec3::random_in_unit_sphere() * self.fuzz),
            r_in.time(),
        );
        *attenuation = self.albedo.copy();
        Vec3::dot(&scattered.direction(), &rec.normal) > 0.0
    }
}

impl Metal {
    pub fn new(a: &Color, f: f64) -> Metal {
        Metal {
            albedo: a.copy(),
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }

    pub fn copy(&self) -> Metal {
        Metal {
            albedo: self.albedo.copy(),
            fuzz: self.fuzz,
        }
    }
}
