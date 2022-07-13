#![allow(unused_variables)]
#![allow(unused_assignments)]
pub use crate::aabb::Aabb;
pub use crate::func;
pub use crate::hittable::Hit;
pub use crate::hittable::Hitrecord;
pub use crate::hittable_list::Hittablelist;
pub use crate::hittable_list::Object;
pub use crate::material::Material;
pub use crate::ray::Ray;
pub use crate::sphere::Boundingbox;
pub use crate::vec3::Color;
pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;
const INF: f64 = 1.79769e+308;

pub struct RotateY {
    ptr: Option<Box<Object>>,
    sin_theta: f64,
    cos_theta: f64,
    hasbox: bool,
    bbox: Aabb,
}

impl RotateY {
    pub fn default_new() -> RotateY {
        RotateY {
            ptr: None,
            sin_theta: 0.0,
            cos_theta: 1.0,
            hasbox: false,
            bbox: Aabb::default_new(),
        }
    }

    pub fn new(p: &Option<Box<Object>>, angle: f64) -> RotateY {
        let radians = func::degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut hasbox = false;
        let mut bbox = Aabb::default_new();
        if let Some(in_ptr) = &p {
            hasbox = in_ptr.boundingbox(0.0, 1.0, &mut bbox);
        }

        let mut min = Point3::new(INF, INF, INF);
        let mut max = Point3::new(-INF, -INF, -INF);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = (i as f64) * bbox.max().x() + ((1 - i) as f64) * bbox.min().x();
                    let y = (j as f64) * bbox.max().y() + ((1 - j) as f64) * bbox.min().y();
                    let z = (k as f64) * bbox.max().z() + ((1 - k) as f64) * bbox.min().z();

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c as usize] = func::fmin(min[c as usize], tester[c as usize]);
                        max[c as usize] = func::fmax(max[c as usize], tester[c as usize]);
                    }
                }
            }
        }
        bbox = Aabb::new(&min, &max);

        RotateY {
            ptr: p
                .as_ref()
                .map(|in_p| Box::new(in_p.copy()))
                .map(|in_p| Box::new(in_p.copy())),
            sin_theta,
            cos_theta,
            hasbox,
            bbox,
        }
    }

    pub fn copy(&self) -> RotateY {
        RotateY {
            ptr: self
                .ptr
                .as_ref()
                .map(|in_p| Box::new(in_p.copy()))
                .map(|in_p| Box::new(in_p.copy())),
            sin_theta: self.sin_theta,
            cos_theta: self.cos_theta,
            hasbox: self.hasbox,
            bbox: self.bbox.copy(),
        }
    }
}

impl Hit for RotateY {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Hitrecord) -> bool {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];

        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];

        let rotated_r = Ray::new(&origin, &direction, r.time());
        /*
                if let None = &self.ptr {
                    return false;
                }
        */
        if let Some(in_ptr) = &self.ptr {
            if !in_ptr.hit(&rotated_r, t_min, t_max, rec) {
                return false;
            }
        }

        let mut p = rec.p.copy();
        let mut normal = rec.normal.copy();

        p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
        p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

        normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
        normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

        rec.p = p;
        rec.set_face_normal(&rotated_r, &normal);

        true
    }
}

impl Boundingbox for RotateY {
    fn boundingbox(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.bbox.copy();
        self.hasbox
    }
}
