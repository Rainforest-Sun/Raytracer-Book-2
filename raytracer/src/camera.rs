#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]
pub use crate::rand;
pub use crate::ray::Ray;
pub use crate::vec3::Color;
pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
    pub time0: f64,
    pub time1: f64,
}

impl Camera {
    /*
    pub fn default_new() -> Camera {
        let aspect_ratio:f64=16.0/9.0;
        let viewport_height=2.0;
        let viewport_width=aspect_ratio*viewport_height;
        let focal_length=1.0;
        let orig=Point3::new(0.0,0.0,0.0);
        let hori=Vec3::new(viewport_width,0.0,0.0);
        let vert=Vec3::new(0.0,viewport_height,0.0);
        let lower_left_corner_=orig.copy()-hori.copy()/2.0-vert.copy()/2.0-Vec3::new(0.0,0.0,focal_length);
        Camera {
            origin:orig,
            lower_left_corner:lower_left_corner_,
            horizontal:hori,
            vertical:vert,
        }
    }
    */
    pub fn new(
        lookfrom: &Point3,
        lootat: &Point3,
        vup: &Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        _time0: f64,
        _time1: f64,
    ) -> Camera {
        let theta = vfov * (std::f64::consts::PI) / 180.0;
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3::unit_vector(&(lookfrom.copy() - lootat.copy()));
        let u = Vec3::unit_vector(&Vec3::cross(&vup.copy(), &w.copy()));
        let v = Vec3::cross(&w.copy(), &u.copy());

        let orig = lookfrom.copy();
        let hori = u.copy() * viewport_width * focus_dist;
        let vert = v.copy() * viewport_height * focus_dist;
        let lower_left_corner_ =
            orig.copy() - hori.copy() / 2.0 - vert.copy() / 2.0 - w.copy() * focus_dist;

        Camera {
            origin: orig,
            lower_left_corner: lower_left_corner_,
            horizontal: hori,
            vertical: vert,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
            time0: _time0,
            time1: _time1,
        }
    }

    pub fn copy(&self) -> Camera {
        Camera {
            origin: self.origin.copy(),
            lower_left_corner: self.lower_left_corner.copy(),
            horizontal: self.horizontal.copy(),
            vertical: self.vertical.copy(),
            u: self.u.copy(),
            v: self.v.copy(),
            w: self.w.copy(),
            lens_radius: self.lens_radius,
            time0: self.time0,
            time1: self.time1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u.copy() * rd.x() + self.v.copy() * rd.y();

        Ray::new(
            &(self.origin.copy() + offset.copy()),
            &(self.lower_left_corner.copy()
                + self.horizontal.copy() * s
                + self.vertical.copy() * t
                - self.origin.copy()
                - offset.copy()),
            rand::random_double_between(self.time0, self.time1),
        )
    }
}
