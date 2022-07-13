pub use crate::camera::Camera;
pub use crate::dielectric::Dielectric;
pub use crate::diffuse_light::Diffuselight;
pub use crate::hittable::Hit;
pub use crate::hittable::Hitrecord;
pub use crate::hittable_list::Hittablelist;
pub use crate::hittable_list::Object;
pub use crate::isotropic::Isotropic;
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
    Diffuselight(Diffuselight),
    Isotropic(Isotropic),
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
        match &self {
            Material::Lambertian(lambertian) => {
                Lambertian::scatter(&lambertian, &r_in, &rec, attenuation, scattered)
            }
            Material::Metal(metal) => Metal::scatter(&metal, &r_in, &rec, attenuation, scattered),
            Material::Dielectric(dielectric) => {
                Dielectric::scatter(&dielectric, &r_in, &rec, attenuation, scattered)
            }
            Material::Diffuselight(diffuse_light) => {
                Diffuselight::scatter(&diffuse_light, &r_in, &rec, attenuation, scattered)
            }
            Material::Isotropic(isotropic) => {
                Isotropic::scatter(&isotropic, &r_in, &rec, attenuation, scattered)
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
            Material::Diffuselight(diffuse_light) => Material::Diffuselight(diffuse_light.copy()),
            Material::Isotropic(isotropic) => Material::Isotropic(isotropic.copy()),
        }
    }
}

pub trait Emitted {
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color;
}

impl Emitted for Material {
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        match &self {
            Material::Diffuselight(diffuse_light) => {
                Diffuselight::emitted(&diffuse_light, u, v, &p)
            }
            _ => Color::new(0.0, 0.0, 0.0),
        }
    }
}
