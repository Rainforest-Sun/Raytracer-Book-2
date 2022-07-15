#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::clone_double_ref)]
#![allow(clippy::never_loop)]

//use tobj::Material;

pub use crate::aabb::Aabb;
pub use crate::boxx::Boxx;
pub use crate::bvhnode::Bvhnode;
pub use crate::camera::Camera;
pub use crate::checker_texture::Checkertexture;
pub use crate::constant_medium::ConstantMedium;
pub use crate::dielectric::Dielectric;
pub use crate::diffuse_light::Diffuselight;
pub use crate::hittable::Hit;
pub use crate::hittable::Hitrecord;
pub use crate::hittable_list::Hittablelist;
pub use crate::hittable_list::Object;
pub use crate::image_texture::Imagetexture;
pub use crate::isotropic::Isotropic;
pub use crate::lambertian::Lambertian;
pub use crate::material::Emitted;
pub use crate::material::Material;
pub use crate::material::Scatter;
pub use crate::metal::Metal;
pub use crate::movingsphere::Movingsphere;
pub use crate::noise_texture::Noisetexture;
pub use crate::perlin::Perlin;
pub use crate::ray::Ray;
pub use crate::rect::XYrect;
pub use crate::rect::XZrect;
pub use crate::rect::YZrect;
pub use crate::rotate::RotateY;
pub use crate::solidcolor::Solidcolor;
pub use crate::sphere::Sphere;
pub use crate::texture::Texture;
pub use crate::texture::Value;
use crate::translate::Boundingbox;
pub use crate::translate::Translate;
pub use crate::triangle::Triangle;
pub use crate::vec3::Color;
pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;
pub struct Obj {
    pub obj: Bvhnode,
}

impl Obj {
    pub fn new(file_name: &str, mat: &Option<Box<Material>>, tm: f64, dur: f64) -> Obj {
        let tmp_tri = tobj::load_obj(
            file_name,
            &tobj::LoadOptions {
                single_index: false,
                triangulate: true,
                ..Default::default()
            },
        );
        assert!(tmp_tri.is_ok());
        let (tri, _mtl_mat) = tmp_tri.expect("Failed to load OBJ file.");

        let mut objects = Hittablelist::default_new();
        for (i, obj) in tri.iter().enumerate() {
            let mut cnt = 0;
            let mut pos = [0; 3];
            let mesh = &obj.mesh;
            for p in &mesh.indices {
                pos[cnt] = (*p as usize) * 3;
                cnt += 1;
                if cnt == 3 {
                    objects.add(Object::Triangle(Triangle::new(
                        &[
                            Point3::new(
                                mesh.positions[pos[0]] as f64,
                                mesh.positions[pos[0] + 1] as f64,
                                mesh.positions[pos[0] + 2] as f64,
                            ),
                            Point3::new(
                                mesh.positions[pos[1]] as f64,
                                mesh.positions[pos[1] + 1] as f64,
                                mesh.positions[pos[1] + 2] as f64,
                            ),
                            Point3::new(
                                mesh.positions[pos[2]] as f64,
                                mesh.positions[pos[2] + 1] as f64,
                                mesh.positions[pos[2] + 2] as f64,
                            ),
                        ],
                        mat.clone(),
                    )));
                    cnt = 0;
                }
            }
            break;
        }

        Self {
            obj: Bvhnode::new_from_list(&mut objects, tm, dur),
        }
    }

    pub fn copy(&self) -> Obj {
        Obj {
            obj: self.obj.copy(),
        }
    }
}

impl Hit for Obj {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Hitrecord) -> bool {
        self.obj.hit(&r, t_min, t_max, rec)
    }
}

impl Boundingbox for Obj {
    fn boundingbox(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        self.obj.boundingbox(time0, time1, output_box)
    }
}
