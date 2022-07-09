pub use crate::camera::Camera;
pub use crate::hittable::Hit;
pub use crate::hittable::Hitrecord;
pub use crate::hittable_list::Hittablelist;
pub use crate::hittable_list::Object;
pub use crate::lambertian::Lambertian;
pub use crate::material::Material;
pub use crate::material::Scatter;
pub use crate::metal::Metal;
pub use crate::ray::Ray;
pub use crate::sphere::Sphere;
pub use crate::vec3::vec3::Vec3;
pub use crate::vec3::Color;
pub use crate::vec3::Point3;

pub struct Dielectric {
    pub ir: f64,
}

impl Scatter for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &Hitrecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = Vec3::unit_vector(&r_in.direction());

        //let refracted=Vec3::refract(&unit_direction,&rec.normal.copy(),refraction_ratio);
        //*scattered=Ray::new(&rec.p.copy(),&refracted.copy());
        let num = Vec3::dot(&(-unit_direction.copy()), &rec.normal.copy());
        let cos_theta = if num < 1.0 { num } else { 1.0 };
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract: bool = (refraction_ratio * sin_theta) > 1.0;
        let mut direction = Vec3::default_new();

        if cannot_refract {
            direction = Vec3::reflect(&unit_direction.copy(), &rec.normal.copy());
        } else {
            direction = Vec3::refract(&unit_direction.copy(), &rec.normal.copy(), refraction_ratio);
        }

        *scattered = Ray::new(&rec.p.copy(), &direction);
        true
    }
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            ir: index_of_refraction,
        }
    }

    pub fn copy(&self) -> Dielectric {
        Dielectric { ir: self.ir }
    }

    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * ((1.0 - cosine).powf(5.0))
    }
}
