pub use crate::hittable::Hit;
pub use crate::hittable::Hitrecord;
pub use crate::ray::Ray;
pub use crate::vec3::Vec3;
pub use crate::vec3::Color;
pub use crate::vec3::Point3;
pub use crate::Sphere;

pub enum Object {
    Sphere(Sphere),
}

impl Hit for Object {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Hitrecord) -> bool {
        match self {
            Object::Sphere(sphere) => Sphere::hit(&sphere, &r, t_min, t_max, rec),
            _ => false,
        }
    }
}

impl Object {
    pub fn copy(&self) -> Object {
        match &self {
            Object::Sphere(sphere) => Object::Sphere(sphere.copy()),
        }
    }
}

pub struct Hittablelist {
    pub objects: Vec<Box<Object>>,
}

impl Hittablelist {
    pub fn default_new() -> Hittablelist {
        Hittablelist { objects: vec![] }
    }

    pub fn new(obj: Box<Object>) -> Hittablelist {
        let mut res = Hittablelist::default_new();
        res.add(obj);
        res
    }

    pub fn add(&mut self, obj: Box<Object>) {
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
