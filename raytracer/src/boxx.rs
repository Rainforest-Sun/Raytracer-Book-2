#![allow(unused_variables)]
#![allow(unused_assignments)]
pub use crate::aabb::Aabb;
pub use crate::hittable::Hit;
pub use crate::hittable::Hitrecord;
pub use crate::hittable_list::Hittablelist;
pub use crate::hittable_list::Object;
pub use crate::material::Material;
pub use crate::ray::Ray;
pub use crate::rect::XYrect;
pub use crate::rect::XZrect;
pub use crate::rect::YZrect;
pub use crate::sphere::Boundingbox;
pub use crate::vec3::Color;
pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;

pub struct Boxx {
    box_min: Point3,
    box_max: Point3,
    sides: Hittablelist,
}

impl Boxx {
    pub fn default_new() -> Boxx {
        Boxx {
            box_min: Point3::default_new(),
            box_max: Point3::default_new(),
            sides: Hittablelist::default_new(),
        }
    }

    pub fn new(p0: &Point3, p1: &Point3, ptr: &Option<Box<Material>>) -> Boxx {
        let box_min = p0.copy();
        let box_max = p1.copy();

        let mut sides = Hittablelist::default_new();
        sides.add(Object::XYrect(XYrect::new(
            &ptr,
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
        )));
        sides.add(Object::XYrect(XYrect::new(
            &ptr,
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
        )));
        sides.add(Object::XZrect(XZrect::new(
            &ptr,
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p1.y(),
        )));
        sides.add(Object::XZrect(XZrect::new(
            &ptr,
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p0.y(),
        )));
        sides.add(Object::YZrect(YZrect::new(
            &ptr,
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p1.x(),
        )));
        sides.add(Object::YZrect(YZrect::new(
            &ptr,
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p0.x(),
        )));

        Boxx {
            box_min,
            box_max,
            sides,
        }
    }

    pub fn copy(&self) -> Boxx {
        Boxx {
            box_min: self.box_min.copy(),
            box_max: self.box_max.copy(),
            sides: self.sides.copy(),
        }
    }
}

impl Hit for Boxx {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Hitrecord) -> bool {
        self.sides.hit(&r, t_min, t_max, rec)
    }
}

impl Boundingbox for Boxx {
    fn boundingbox(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(&self.box_min.copy(), &self.box_max.copy());
        true
    }
}
