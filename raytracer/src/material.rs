pub use crate::camera::Camera;
pub use crate::dielectric::Dielectric;
pub use crate::hittable::Hit;
pub use crate::hittable::Hitrecord;
pub use crate::hittable_list::Hittablelist;
pub use crate::hittable_list::Object;
pub use crate::lambertian::Lambertian;
pub use crate::metal::Metal;
pub use crate::ray::Ray;
pub use crate::sphere::Sphere;
pub use crate::vec3::Color;
pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;

pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

pub trait Scatter {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &Hitrecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

impl Scatter for Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &Hitrecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Material::Lambertian(lambertian) => {
                Lambertian::scatter(&lambertian, &r_in, &rec, attenuation, scattered)
            }
            Material::Metal(metal) => Metal::scatter(&metal, &r_in, &rec, attenuation, scattered),
            Material::Dielectric(dielectric) => {
                Dielectric::scatter(&dielectric, &r_in, &rec, attenuation, scattered)
            }
        }
    }
}

impl Material {
    pub fn copy(&self) -> Material {
        match &self {
            Material::Lambertian(lambertian) => Material::Lambertian(lambertian.copy()),
            Material::Metal(metal) => Material::Metal(metal.copy()),
            Material::Dielectric(dielectric) => Material::Dielectric(dielectric.copy()),
        }
    }
}
