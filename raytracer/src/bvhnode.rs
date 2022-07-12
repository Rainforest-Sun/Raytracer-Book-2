#![allow(warnings, unused)]
pub use crate::aabb::Aabb;
pub use crate::camera::Camera;
pub use crate::dielectric::Dielectric;
pub use crate::hittable::Hit;
pub use crate::hittable::Hitrecord;
pub use crate::hittable_list::Hittablelist;
pub use crate::hittable_list::Object;
pub use crate::lambertian::Lambertian;
pub use crate::material::Material;
pub use crate::material::Scatter;
pub use crate::metal::Metal;
pub use crate::movingsphere::Movingsphere;
pub use crate::rand;
pub use crate::ray::Ray;
pub use crate::sphere::Boundingbox;
pub use crate::sphere::Sphere;
pub use crate::vec3::Color;
pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;
pub use std::cmp::Ordering;

pub struct Bvhnode {
    pub left: Option<Box<Object>>,
    pub right: Option<Box<Object>>,
    pub boxx: Aabb,
}

impl Bvhnode {
    pub fn default_new() -> Bvhnode {
        Bvhnode {
            left: None,
            right: None,
            boxx: Aabb::default_new(),
        }
    }

    pub fn copy(&self) -> Bvhnode {
        Bvhnode {
            boxx: self.boxx.copy(),
            left: self
                .left
                .as_ref()
                .map(|in_left| Box::new(in_left.copy()))
                .map(|in_left| Box::new(in_left.copy())),
            right: self
                .right
                .as_ref()
                .map(|in_right| Box::new(in_right.copy()))
                .map(|in_right| Box::new(in_right.copy())),
        }
    }

    pub fn new_from_list(list: &mut Hittablelist, time0: f64, time1: f64) -> Bvhnode {
        let len = list.objects.len();
        Bvhnode::new_from_vec(&mut list.objects, 0, len, time0, time1)
    }

    pub fn new_from_vec(
        src_objects: &mut Vec<Object>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Bvhnode {
        let mut myleft: Option<Box<Object>> = None;
        let mut myright: Option<Box<Object>> = None;
        let mut objects: &mut Vec<Object> = src_objects;

        let axis = rand::random_int_between(0, 2);
        let mut temp_x = Aabb::default_new();
        let mut temp_y = Aabb::default_new();
        let comparator = |x: &Object, y: &Object| {
            x.boundingbox(time0, time1, &mut temp_x);
            y.boundingbox(time0, time1, &mut temp_y);
            f64::partial_cmp(
                &(temp_x.min()[axis as usize]),
                &(temp_y.min()[axis as usize]),
            )
            .unwrap()
        };
        let object_span = end - start;

        if object_span == 1 {
            myleft = Some(Box::new(objects[start].copy()));
            myright = Some(Box::new(objects[start].copy()));
        } else if object_span == 2 {
            if Bvhnode::box_compare(&objects[start], &objects[start + 1], axis) {
                myleft = Some(Box::new(objects[start].copy()));
                myright = Some(Box::new(objects[start + 1].copy()));
            } else {
                myleft = Some(Box::new(objects[start + 1].copy()));
                myright = Some(Box::new(objects[start].copy()));
            }
        } else {
            objects.sort_unstable_by(comparator);

            let mid = start + object_span / 2;
            myleft = Some(Box::new(Object::Bvhnode(Bvhnode::new_from_vec(
                &mut objects,
                start,
                mid,
                time0,
                time1,
            ))));
            myright = Some(Box::new(Object::Bvhnode(Bvhnode::new_from_vec(
                &mut objects,
                mid,
                end,
                time0,
                time1,
            ))));
        }

        let mut box_left = Aabb::default_new();
        let mut box_right = Aabb::default_new();

        let mut flag_l = false;
        let mut flag_r = false;

        if let Some(left) = &myleft {
            flag_l = left.boundingbox(time0, time1, &mut box_left);
        }
        if let Some(right) = &myright {
            flag_r = right.boundingbox(time0, time1, &mut box_right);
        }
        if !flag_l || !flag_r {
            eprint!("No bounding box in bvh_node constructor.");
        }

        Bvhnode {
            left: myleft,
            right: myright,
            boxx: Aabb::surrounding_box(&box_left, &box_right),
        }
    }

    pub fn box_compare(a: &Object, b: &Object, axis: i32) -> bool {
        let mut box_a = Aabb::default_new();
        let mut box_b = Aabb::default_new();
        /*
        let mut flag_a=false;
        let mut flag_b=false;
        if let in_a = Some(a) {
            flag_a=in_a.boundingbox(0.0,0.0,box_a);
        }
        if let in_b = Some(b) {
            flag_b=in_b.boundingbox(0.0,0.0,box_b);
        }
        */
        let flag_a = a.boundingbox(0.0, 0.0, &mut box_a);
        let flag_b = b.boundingbox(0.0, 0.0, &mut box_b);
        if !flag_a || !flag_b {
            eprint!("No bounding box in bvh_node constructor.");
        }

        box_a.min()[axis as usize] < box_b.min()[axis as usize]
    }

    pub fn box_x_compare(a: &Object, b: &Object) -> bool {
        Bvhnode::box_compare(&a, &b, 0)
    }

    pub fn box_y_compare(a: &Object, b: &Object) -> bool {
        Bvhnode::box_compare(&a, &b, 1)
    }

    pub fn box_z_compare(a: &Object, b: &Object) -> bool {
        Bvhnode::box_compare(&a, &b, 2)
    }
}

impl Hit for Bvhnode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Hitrecord) -> bool {
        let mut temp_rec = Hitrecord::default_new();
        if !self.boxx.hit(&r, t_min, t_max, &mut temp_rec) {
            return false;
        }
        let mut hit_left = false;
        let mut hit_right = false;
        if let Some(left) = &self.left {
            hit_left = left.hit(&r, t_min, t_max, rec);
        }
        if let Some(right) = &self.right {
            hit_right = right.hit(&r, t_min, t_max, rec);
        }
        hit_left || hit_right
    }
}

impl Boundingbox for Bvhnode {
    fn boundingbox(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.boxx.copy();
        true
    }
}
