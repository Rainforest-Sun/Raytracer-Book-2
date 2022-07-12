pub use crate::aabb::Aabb;
pub use crate::bvhnode::Bvhnode;
pub use crate::hittable::Boundingbox;
pub use crate::hittable::Hit;
pub use crate::hittable::Hitrecord;
pub use crate::movingsphere::Movingsphere;
pub use crate::ray::Ray;
pub use crate::sphere::Sphere;
pub use crate::vec3::Color;
pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;

pub enum Object {
    Sphere(Sphere),
    Movingsphere(Movingsphere),
    Aabb(Aabb),
    Bvhnode(Bvhnode),
}

impl Hit for Object {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Hitrecord) -> bool {
        match self {
            Object::Sphere(sphere) => Sphere::hit(&sphere, &r, t_min, t_max, rec),
            Object::Movingsphere(movingsphere) => {
                Movingsphere::hit(&movingsphere, &r, t_min, t_max, rec)
            }
            Object::Aabb(aabb) => Aabb::hit(&aabb, &r, t_min, t_max, rec),
            Object::Bvhnode(bvhnode) => Bvhnode::hit(&bvhnode, &r, t_min, t_max, rec),
        }
    }
}

impl Boundingbox for Object {
    fn boundingbox(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        match self {
            Object::Sphere(sphere) => Sphere::boundingbox(&sphere, _time0, _time1, output_box),
            Object::Movingsphere(movingsphere) => {
                Movingsphere::boundingbox(&movingsphere, _time0, _time1, output_box)
            }
            Object::Bvhnode(bvhnode) => Bvhnode::boundingbox(&bvhnode, _time0, _time1, output_box),
            _ => false,
        }
    }
}

impl Object {
    pub fn copy(&self) -> Object {
        match &self {
            Object::Sphere(sphere) => Object::Sphere(sphere.copy()),
            Object::Movingsphere(movingsphere) => Object::Movingsphere(movingsphere.copy()),
            Object::Aabb(aabb) => Object::Aabb(aabb.copy()),
            Object::Bvhnode(bvhnode) => Object::Bvhnode(bvhnode.copy()),
        }
    }
}

pub struct Hittablelist {
    pub objects: Vec<Object>,
}

impl Hittablelist {
    pub fn default_new() -> Hittablelist {
        Hittablelist { objects: vec![] }
    }

    pub fn new(obj: Object) -> Hittablelist {
        let mut res = Hittablelist::default_new();
        res.add(obj);
        res
    }

    pub fn add(&mut self, obj: Object) {
        self.objects.push(obj);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hit for Hittablelist {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Hitrecord) -> bool {
        let mut temp_rec = Hitrecord::default_new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(&r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.copy();
            }
        }

        hit_anything
    }
}

impl Boundingbox for Hittablelist {
    fn boundingbox(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        if self.objects.is_empty() {
            return false;
        }

        let mut temp_box = Aabb::default_new();
        let mut first_box = true;

        for object in &self.objects {
            if !object.boundingbox(time0, time1, &mut temp_box) {
                return false;
            }
            *output_box = if first_box {
                temp_box.copy()
            } else {
                Aabb::surrounding_box(&output_box, &temp_box)
            };
            first_box = false;
        }

        true
    }
}
