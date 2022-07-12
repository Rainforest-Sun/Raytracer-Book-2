pub use crate::aabb::Aabb;
pub use crate::hittable::Boundingbox;
pub use crate::hittable::Hit;
pub use crate::hittable::Hitrecord;
pub use crate::material::Material;
pub use crate::ray::Ray;
pub use crate::vec3::Color;
pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;

pub struct Movingsphere {
    pub center0: Point3,
    pub center1: Point3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat_ptr: Option<Box<Material>>,
}

impl Movingsphere {
    pub fn default_new() -> Movingsphere {
        Movingsphere {
            center0: Point3::default_new(),
            center1: Point3::default_new(),
            time0: 0.0,
            time1: 0.0,
            radius: 0.0,
            mat_ptr: None,
        }
    }

    pub fn new(
        cen0: &Point3,
        cen1: &Point3,
        _time0: f64,
        _time1: f64,
        r: f64,
        m: &Option<Box<Material>>,
    ) -> Movingsphere {
        Movingsphere {
            center0: cen0.copy(),
            center1: cen1.copy(),
            time0: _time0,
            time1: _time1,
            radius: r,
            mat_ptr: m
                .as_ref()
                .map(|in_mat_ptr| Box::new(in_mat_ptr.copy()))
                .map(|in_mat_ptr| Box::new(in_mat_ptr.copy())),
        }
    }

    pub fn copy(&self) -> Movingsphere {
        Movingsphere {
            center0: self.center0.copy(),
            center1: self.center1.copy(),
            time0: self.time0,
            time1: self.time1,
            radius: self.radius,
            mat_ptr: self
                .mat_ptr
                .as_ref()
                .map(|in_mat_ptr| Box::new(in_mat_ptr.copy()))
                .map(|in_mat_ptr| Box::new(in_mat_ptr.copy())),
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        self.center0.copy()
            + (self.center1.copy() - self.center0.copy())
                * ((time - self.time0) / (self.time1 - self.time0))
    }
}

impl Hit for Movingsphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Hitrecord) -> bool {
        let oc = r.origin() - self.center(r.time());
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b.powf(2.0) - (a * c);
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p.copy() - self.center(r.time())) / self.radius;
        rec.set_face_normal(&r, &outward_normal);
        rec.mat_ptr = self
            .mat_ptr
            .as_ref()
            .map(|in_mat_ptr| Box::new(in_mat_ptr.copy()))
            .map(|in_mat_ptr| Box::new(in_mat_ptr.copy()));
        true
    }
}

impl Boundingbox for Movingsphere {
    fn boundingbox(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        let box0 = Aabb::new(
            &(self.center(_time0) - Vec3::new(self.radius, self.radius, self.radius)),
            &(self.center(_time0) + Vec3::new(self.radius, self.radius, self.radius)),
        );
        let box1 = Aabb::new(
            &(self.center(_time1) - Vec3::new(self.radius, self.radius, self.radius)),
            &(self.center(_time1) + Vec3::new(self.radius, self.radius, self.radius)),
        );
        *output_box = Aabb::surrounding_box(&box0, &box1);
        true
    }
}
