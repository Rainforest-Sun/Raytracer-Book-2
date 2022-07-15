#![allow(non_snake_case)]
pub use std::{
    fs::File,
    process::exit,
    sync::{mpsc, Arc},
    thread,
    time::Instant,
};

pub use image::{ImageBuffer, RgbImage};

pub use console::style;
pub use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};

pub mod aabb;
pub mod boxx;
pub mod bvhnode;
pub mod camera;
pub mod checker_texture;
pub mod color;
pub mod constant_medium;
pub mod dielectric;
pub mod diffuse_light;
pub mod func;
pub mod hittable;
pub mod hittable_list;
pub mod image_texture;
pub mod isotropic;
pub mod lambertian;
pub mod material;
pub mod metal;
pub mod movingsphere;
pub mod noise_texture;
pub mod obj;
pub mod obj_texture;
pub mod perlin;
pub mod rand;
pub mod ray;
pub mod rect;
pub mod rotate;
pub mod solidcolor;
pub mod sphere;
pub mod texture;
pub mod translate;
pub mod triangle;
pub mod vec3;

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
pub use crate::obj::Obj;
pub use crate::obj_texture::Objtexture;
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
pub use crate::translate::Translate;
pub use crate::triangle::Triangle;
pub use crate::vec3::Color;
pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;

pub fn ray_color(r: &Ray, background: &Color, world: &Hittablelist, depth: i32) -> Color {
    let mut rec = Hitrecord::default_new();
    let inf: f64 = 1.79769e+308;

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    if !world.hit(&r, 0.001, inf, &mut rec) {
        return background.copy();
    }

    let mut scattered = Ray::default_new();
    let mut attenuation = Color::default_new();
    let emitted;
    if let Some(in_mat_ptr) = &rec.mat_ptr {
        emitted = in_mat_ptr.emitted(rec.u, rec.v, &rec.p.copy());
        if in_mat_ptr.scatter(&r, &rec, &mut attenuation, &mut scattered) {
            return emitted + ray_color(&scattered, &background, &world, depth - 1) * attenuation;
        } else {
            return emitted;
        }
    }

    Color::new(0.0, 0.0, 0.0)
}

pub fn two_spheres() -> Hittablelist {
    let mut objects = Hittablelist::default_new();

    let checker = Some(Box::new(Texture::Checkertexture(
        Checkertexture::new_from_color(&Color::new(0.2, 0.3, 0.1), &Color::new(0.9, 0.9, 0.9)),
    )));

    let ground_material = Some(Box::new(Material::Lambertian(Lambertian::new_from_ptr(
        &checker,
    ))));

    objects.add(Object::Sphere(Sphere::new(
        &Point3::new(0.0, -10.0, 0.0),
        10.0,
        &ground_material,
    )));
    objects.add(Object::Sphere(Sphere::new(
        &Point3::new(0.0, 10.0, 0.0),
        10.0,
        &ground_material,
    )));

    objects
}

pub fn two_perlin_spheres() -> Hittablelist {
    let mut objects = Hittablelist::default_new();

    let texture = Some(Box::new(Texture::Noisetexture(Noisetexture::new(4.0))));

    let material = Some(Box::new(Material::Lambertian(Lambertian::new_from_ptr(
        &texture,
    ))));

    objects.add(Object::Sphere(Sphere::new(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        &material,
    )));
    objects.add(Object::Sphere(Sphere::new(
        &Point3::new(0.0, 2.0, 0.0),
        2.0,
        &material,
    )));

    objects
}

pub fn earth() -> Hittablelist {
    let mut objects = Hittablelist::default_new();

    let texture = Some(Box::new(Texture::Imagetexture(Imagetexture::new(
        &String::from("image/earthmap.jpg"),
    ))));

    let earth_surface = Some(Box::new(Material::Lambertian(Lambertian::new_from_ptr(
        &texture,
    ))));

    objects.add(Object::Sphere(Sphere::new(
        &Point3::new(0.0, 0.0, 0.0),
        2.0,
        &earth_surface,
    )));

    objects
}

pub fn simple_light() -> Hittablelist {
    let mut objects = Hittablelist::default_new();

    let pertext = Some(Box::new(Texture::Noisetexture(Noisetexture::new(4.0))));

    let material1 = Some(Box::new(Material::Lambertian(Lambertian::new_from_ptr(
        &pertext,
    ))));

    objects.add(Object::Sphere(Sphere::new(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        &material1,
    )));
    objects.add(Object::Sphere(Sphere::new(
        &Point3::new(0.0, 2.0, 0.0),
        2.0,
        &material1,
    )));

    let difflight = Some(Box::new(Material::Diffuselight(
        Diffuselight::new_from_color(&Color::new(4.0, 4.0, 4.0)),
    )));

    objects.add(Object::XYrect(XYrect::new(
        &difflight, 3.0, 5.0, 1.0, 3.0, -2.0,
    )));

    objects
}

pub fn cornell_box() -> Hittablelist {
    let mut objects = Hittablelist::default_new();

    let red = Some(Box::new(Material::Lambertian(Lambertian::new(
        &Color::new(0.65, 0.05, 0.05),
    ))));
    let white = Some(Box::new(Material::Lambertian(Lambertian::new(
        &Color::new(0.73, 0.73, 0.73),
    ))));
    let green = Some(Box::new(Material::Lambertian(Lambertian::new(
        &Color::new(0.12, 0.45, 0.15),
    ))));
    let light = Some(Box::new(Material::Diffuselight(
        Diffuselight::new_from_color(&Color::new(15.0, 15.0, 15.0)),
    )));

    objects.add(Object::YZrect(YZrect::new(
        &green, 0.0, 555.0, 0.0, 555.0, 555.0,
    )));
    objects.add(Object::YZrect(YZrect::new(
        &red, 0.0, 555.0, 0.0, 555.0, 0.0,
    )));
    objects.add(Object::XZrect(XZrect::new(
        &light, 213.0, 343.0, 227.0, 332.0, 554.0,
    )));
    objects.add(Object::XZrect(XZrect::new(
        &white, 0.0, 555.0, 0.0, 555.0, 0.0,
    )));
    objects.add(Object::XZrect(XZrect::new(
        &white, 0.0, 555.0, 0.0, 555.0, 555.0,
    )));
    objects.add(Object::XYrect(XYrect::new(
        &white, 0.0, 555.0, 0.0, 555.0, 555.0,
    )));
    let box1 = Some(Box::new(Object::Boxx(Boxx::new(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 330.0, 165.0),
        &white,
    ))));
    let box1 = Some(Box::new(Object::RotateY(RotateY::new(&box1, 15.0))));
    objects.add(Object::Translate(Translate::new(
        &box1,
        &Vec3::new(265.0, 0.0, 295.0),
    )));

    let box2 = Some(Box::new(Object::Boxx(Boxx::new(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 165.0, 165.0),
        &white,
    ))));
    let box2 = Some(Box::new(Object::RotateY(RotateY::new(&box2, -18.0))));
    objects.add(Object::Translate(Translate::new(
        &box2,
        &Vec3::new(130.0, 0.0, 65.0),
    )));

    objects
}

pub fn cornell_smoke() -> Hittablelist {
    let mut objects = Hittablelist::default_new();

    let red = Some(Box::new(Material::Lambertian(Lambertian::new(
        &Color::new(0.65, 0.05, 0.05),
    ))));
    let white = Some(Box::new(Material::Lambertian(Lambertian::new(
        &Color::new(0.73, 0.73, 0.73),
    ))));
    let green = Some(Box::new(Material::Lambertian(Lambertian::new(
        &Color::new(0.12, 0.45, 0.15),
    ))));
    let light = Some(Box::new(Material::Diffuselight(
        Diffuselight::new_from_color(&Color::new(7.0, 7.0, 7.0)),
    )));

    objects.add(Object::YZrect(YZrect::new(
        &green, 0.0, 555.0, 0.0, 555.0, 555.0,
    )));
    objects.add(Object::YZrect(YZrect::new(
        &red, 0.0, 555.0, 0.0, 555.0, 0.0,
    )));
    objects.add(Object::XZrect(XZrect::new(
        &light, 113.0, 443.0, 127.0, 432.0, 554.0,
    )));
    objects.add(Object::XZrect(XZrect::new(
        &white, 0.0, 555.0, 0.0, 555.0, 0.0,
    )));
    objects.add(Object::XZrect(XZrect::new(
        &white, 0.0, 555.0, 0.0, 555.0, 555.0,
    )));
    objects.add(Object::XYrect(XYrect::new(
        &white, 0.0, 555.0, 0.0, 555.0, 555.0,
    )));

    let box1 = Some(Box::new(Object::Boxx(Boxx::new(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 330.0, 165.0),
        &white,
    ))));
    let box1 = Some(Box::new(Object::RotateY(RotateY::new(&box1, 15.0))));
    let box1 = Some(Box::new(Object::Translate(Translate::new(
        &box1,
        &Vec3::new(265.0, 0.0, 295.0),
    ))));
    objects.add(Object::ConstantMedium(ConstantMedium::new_from_color(
        &box1,
        0.01,
        &Color::new(0.0, 0.0, 0.0),
    )));

    let box2 = Some(Box::new(Object::Boxx(Boxx::new(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 165.0, 165.0),
        &white,
    ))));
    let box2 = Some(Box::new(Object::RotateY(RotateY::new(&box2, -18.0))));
    let box2 = Some(Box::new(Object::Translate(Translate::new(
        &box2,
        &Vec3::new(130.0, 0.0, 65.0),
    ))));
    objects.add(Object::ConstantMedium(ConstantMedium::new_from_color(
        &box2,
        0.01,
        &Color::new(1.0, 1.0, 1.0),
    )));

    objects
}

pub fn final_scene() -> Hittablelist {
    let mut boxes1 = Hittablelist::default_new();

    let ground = Some(Box::new(Material::Lambertian(Lambertian::new(
        &Color::new(0.48, 0.83, 0.53),
    ))));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + (i as f64) * w;
            let z0 = -1000.0 + (j as f64) * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rand::random_double_between(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Object::Boxx(Boxx::new(
                &Point3::new(x0, y0, z0),
                &Point3::new(x1, y1, z1),
                &ground,
            )));
        }
    }

    let mut objects = Hittablelist::default_new();

    objects.add(Object::Bvhnode(Bvhnode::new_from_list(
        &mut boxes1,
        0.0,
        1.0,
    )));

    let light = Some(Box::new(Material::Diffuselight(
        Diffuselight::new_from_color(&Color::new(7.0, 7.0, 7.0)),
    )));

    objects.add(Object::XZrect(XZrect::new(
        &light, 123.0, 423.0, 147.0, 412.0, 554.0,
    )));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Some(Box::new(Material::Lambertian(Lambertian::new(
        &Color::new(0.7, 0.3, 0.1),
    ))));
    objects.add(Object::Movingsphere(Movingsphere::new(
        &center1,
        &center2,
        0.0,
        1.0,
        50.0,
        &moving_sphere_material,
    )));

    let die_mat = Some(Box::new(Material::Dielectric(Dielectric::new(1.5))));
    let met_mat = Some(Box::new(Material::Metal(Metal::new(
        &Color::new(0.8, 0.8, 0.9),
        1.0,
    ))));
    objects.add(Object::Sphere(Sphere::new(
        &Point3::new(260.0, 150.0, 45.0),
        50.0,
        &die_mat,
    )));
    objects.add(Object::Sphere(Sphere::new(
        &Point3::new(0.0, 150.0, 145.0),
        50.0,
        &met_mat,
    )));

    let boundary = Some(Box::new(Object::Sphere(Sphere::new(
        &Point3::new(360.0, 150.0, 145.0),
        70.0,
        &die_mat,
    ))));
    objects.add(Object::Sphere(Sphere::new(
        &Point3::new(360.0, 150.0, 145.0),
        70.0,
        &die_mat,
    )));
    objects.add(Object::ConstantMedium(ConstantMedium::new_from_color(
        &boundary,
        0.2,
        &Color::new(0.2, 0.4, 0.9),
    )));
    let boundary = Some(Box::new(Object::Sphere(Sphere::new(
        &Point3::new(0.0, 0.0, 0.0),
        5000.0,
        &die_mat,
    ))));
    objects.add(Object::ConstantMedium(ConstantMedium::new_from_color(
        &boundary,
        0.0001,
        &Color::new(1.0, 1.0, 1.0),
    )));

    let earth_texture = Some(Box::new(Texture::Imagetexture(Imagetexture::new(
        &String::from("image/earthmap.jpg"),
    ))));
    let e_mat = Some(Box::new(Material::Lambertian(Lambertian::new_from_ptr(
        &earth_texture,
    ))));
    objects.add(Object::Sphere(Sphere::new(
        &Point3::new(400.0, 200.0, 400.0),
        100.0,
        &e_mat,
    )));

    let pertext = Some(Box::new(Texture::Noisetexture(Noisetexture::new(0.1))));
    let lam_mat = Some(Box::new(Material::Lambertian(Lambertian::new_from_ptr(
        &pertext,
    ))));
    objects.add(Object::Sphere(Sphere::new(
        &Point3::new(220.0, 280.0, 300.0),
        80.0,
        &lam_mat,
    )));

    let mut boxes2 = Hittablelist::default_new();
    let white = Some(Box::new(Material::Lambertian(Lambertian::new(
        &Color::new(0.73, 0.73, 0.73),
    ))));
    let ns = 1000;

    for _j in 0..ns {
        boxes2.add(Object::Sphere(Sphere::new(
            &Point3::random_between(0.0, 165.0),
            10.0,
            &white,
        )));
    }

    let bvh_obj = Some(Box::new(Object::Bvhnode(Bvhnode::new_from_list(
        &mut boxes2,
        0.0,
        1.0,
    ))));
    let rot_obj = Some(Box::new(Object::RotateY(RotateY::new(&bvh_obj, 15.0))));
    objects.add(Object::Translate(Translate::new(
        &rot_obj,
        &Vec3::new(-100.0, 270.0, 395.0),
    )));

    objects
}

pub fn obj() -> Hittablelist {
    let mut world = Hittablelist::default_new();
    /*
        let texture = Some(Box::new(Texture::Imagetexture(Imagetexture::new(
            &String::from("image/Char_Patrick.png"),
        ))));

        let mat = Some(Box::new(Material::Lambertian(Lambertian::new_from_ptr(
            &texture,
        ))));
    */
    let mat = Some(Box::new(Material::Lambertian(Lambertian::new(
        &Color::new(0.65, 0.05, 0.05),
    ))));
    //let patrick=Obj::new("image/patrick.obj",&met_mat,0.0,1.0);
    world.add(Object::Obj(Obj::new("image/patrick.obj", &mat, 0.0, 1.0)));
    world
}

pub fn myworld() -> Hittablelist {
    let mut world = Hittablelist::default_new();
    //地面
    let checker = Some(Box::new(Texture::Checkertexture(
        Checkertexture::new_from_color(&Color::new(0.0, 0.0, 0.0), &Color::new(0.2, 0.2, 0.2)),
    )));
    let ground_material = Some(Box::new(Material::Lambertian(Lambertian::new_from_ptr(
        &checker,
    ))));
    world.add(Object::Sphere(Sphere::new(
        &Point3::new(0.0, -100000.0, 0.0),
        100000.0,
        &ground_material,
    )));

    //光源
    let light = Some(Box::new(Material::Diffuselight(
        Diffuselight::new_from_color(&Color::new(7.0, 7.0, 7.0)),
    )));
    let orange_light = Some(Box::new(Material::Diffuselight(
        Diffuselight::new_from_color(&Color::new(7.0, 0.725, 0.059)),
    )));
    world.add(Object::Sphere(Sphere::new(
        &Point3::new(0.0, 2.0, 0.0),
        2.0,
        &light,
    )));
    let box1 = Some(Box::new(Object::YZrect(YZrect::new(
        &orange_light,
        0.0,
        5.0,
        5.0,
        18.0,
        18.0,
    ))));
    let box1 = Some(Box::new(Object::RotateY(RotateY::new(&box1, -20.0))));
    world.add(Object::Translate(Translate::new(
        &box1,
        &Vec3::new(0.0, 0.0, 0.0),
    )));
    //上方有个亮球
    let strong_light = Some(Box::new(Material::Diffuselight(
        Diffuselight::new_from_color(&Color::new(15.0, 15.0, 15.0)),
    )));
    world.add(Object::Sphere(Sphere::new(
        &Point3::new(0.0, 300.0, 0.0),
        50.0,
        &strong_light,
    )));
    //吊起来的球
    let orange_light = Some(Box::new(Material::Diffuselight(
        Diffuselight::new_from_color(&Color::new(1.0, 0.5, 0.0)),
    )));
    world.add(Object::YZrect(YZrect::new(
        &orange_light,
        8.0,
        50.0,
        10.0,
        10.1,
        -15.0,
    )));
    world.add(Object::Sphere(Sphere::new(
        &Point3::new(-15.0, 6.0, 10.0),
        2.0,
        &orange_light,
    )));

    let pink_light = Some(Box::new(Material::Diffuselight(
        Diffuselight::new_from_color(&Color::new(1.0, 0.08, 0.65)),
    )));
    world.add(Object::YZrect(YZrect::new(
        &pink_light,
        10.0,
        50.0,
        0.0,
        0.1,
        -12.0,
    )));
    world.add(Object::Sphere(Sphere::new(
        &Point3::new(-12.0, 8.0, 0.0),
        2.0,
        &pink_light,
    )));
    //玻璃球
    let die_mat = Some(Box::new(Material::Dielectric(Dielectric::new(1.5))));
    world.add(Object::Sphere(Sphere::new(
        &Point3::new(-2.0, 2.0, 25.0),
        2.0,
        &die_mat,
    )));
    //金属球
    let met_mat = Some(Box::new(Material::Metal(Metal::new(
        &Color::new(0.7, 0.6, 0.5),
        0.4,
    ))));
    world.add(Object::Sphere(Sphere::new(
        &Point3::new(-8.0, 2.0, 22.0),
        2.0,
        &met_mat,
    )));
    //大理石纹理
    let pertext = Some(Box::new(Texture::Noisetexture(Noisetexture::new(4.0))));

    let material1 = Some(Box::new(Material::Lambertian(Lambertian::new_from_ptr(
        &pertext,
    ))));
    world.add(Object::Sphere(Sphere::new(
        &Point3::new(8.0, 2.0, 18.0),
        2.0,
        &material1,
    )));
    //方格
    let blue = Some(Box::new(Material::Lambertian(Lambertian::new(
        &Color::new(0.0, 0.0, 0.8029),
    ))));
    let box1 = Some(Box::new(Object::Boxx(Boxx::new(
        &Point3::new(5.0, 0.0, -20.0),
        &Point3::new(10.0, 5.0, -15.0),
        &blue,
    ))));
    let box1 = Some(Box::new(Object::RotateY(RotateY::new(&box1, -45.0))));
    world.add(Object::Translate(Translate::new(
        &box1,
        &Vec3::new(-5.0, 0.0, -5.0),
    )));
    //镜子
    let met_mat = Some(Box::new(Material::Metal(Metal::new(
        &Color::new(0.7, 0.6, 0.5),
        0.0,
    ))));
    let met_box = Some(Box::new(Object::XYrect(XYrect::new(
        &met_mat, -50.0, 20.0, -10.0, 20.0, -15.0,
    ))));
    let met_box = Some(Box::new(Object::RotateY(RotateY::new(&met_box, 25.0))));
    world.add(Object::Translate(Translate::new(
        &met_box,
        &Vec3::new(0.0, 0.0, 0.0),
    )));
    //星星
    let star = Some(Box::new(Material::Diffuselight(
        Diffuselight::new_from_color(&Color::new(1.0, 0.843, 0.0)),
    )));
    world.add(Object::Triangle(Triangle::new(
        &[
            Point3::new(20.0, 20.0, -40.0),
            Point3::new(20.5, 19.0, -40.0),
            Point3::new(19.5, 19.0, -40.0),
        ],
        &star,
    )));
    world.add(Object::Triangle(Triangle::new(
        &[
            Point3::new(20.0, 18.0, -40.0),
            Point3::new(20.5, 19.0, -40.0),
            Point3::new(19.5, 19.0, -40.0),
        ],
        &star,
    )));
    world.add(Object::Triangle(Triangle::new(
        &[
            Point3::new(19.0, 19.0, -40.0),
            Point3::new(20.0, 19.5, -40.0),
            Point3::new(20.0, 18.5, -40.0),
        ],
        &star,
    )));
    world.add(Object::Triangle(Triangle::new(
        &[
            Point3::new(21.0, 19.0, -40.0),
            Point3::new(20.0, 19.5, -40.0),
            Point3::new(20.0, 18.5, -40.0),
        ],
        &star,
    )));
    world.add(Object::Triangle(Triangle::new(
        &[
            Point3::new(23.0, 13.6, -40.0),
            Point3::new(23.3, 13.0, -40.0),
            Point3::new(22.7, 13.0, -40.0),
        ],
        &star,
    )));
    world.add(Object::Triangle(Triangle::new(
        &[
            Point3::new(23.0, 12.4, -40.0),
            Point3::new(23.3, 13.0, -40.0),
            Point3::new(22.7, 13.0, -40.0),
        ],
        &star,
    )));
    world.add(Object::Triangle(Triangle::new(
        &[
            Point3::new(23.6, 13.0, -40.0),
            Point3::new(23.0, 13.3, -40.0),
            Point3::new(23.0, 12.7, -40.0),
        ],
        &star,
    )));
    world.add(Object::Triangle(Triangle::new(
        &[
            Point3::new(22.4, 13.0, -40.0),
            Point3::new(23.0, 13.3, -40.0),
            Point3::new(23.0, 12.7, -40.0),
        ],
        &star,
    )));

    world.add(Object::Triangle(Triangle::new(
        &[
            Point3::new(30.0, 17.6, -40.0),
            Point3::new(30.3, 17.0, -40.0),
            Point3::new(29.7, 17.0, -40.0),
        ],
        &star,
    )));
    world.add(Object::Triangle(Triangle::new(
        &[
            Point3::new(30.0, 16.4, -40.0),
            Point3::new(30.3, 17.0, -40.0),
            Point3::new(29.7, 17.0, -40.0),
        ],
        &star,
    )));
    world.add(Object::Triangle(Triangle::new(
        &[
            Point3::new(30.6, 17.0, -40.0),
            Point3::new(30.0, 17.3, -40.0),
            Point3::new(30.0, 16.7, -40.0),
        ],
        &star,
    )));
    world.add(Object::Triangle(Triangle::new(
        &[
            Point3::new(29.4, 17.0, -40.0),
            Point3::new(30.0, 17.3, -40.0),
            Point3::new(30.0, 16.7, -40.0),
        ],
        &star,
    )));

    world.add(Object::Triangle(Triangle::new(
        &[
            Point3::new(35.0, 10.0, -40.0),
            Point3::new(35.5, 9.0, -40.0),
            Point3::new(34.5, 9.0, -40.0),
        ],
        &star,
    )));
    world.add(Object::Triangle(Triangle::new(
        &[
            Point3::new(35.0, 8.0, -40.0),
            Point3::new(35.5, 9.0, -40.0),
            Point3::new(34.5, 9.0, -40.0),
        ],
        &star,
    )));
    world.add(Object::Triangle(Triangle::new(
        &[
            Point3::new(34.0, 9.0, -40.0),
            Point3::new(35.0, 9.5, -40.0),
            Point3::new(35.0, 8.5, -40.0),
        ],
        &star,
    )));
    world.add(Object::Triangle(Triangle::new(
        &[
            Point3::new(36.0, 9.0, -40.0),
            Point3::new(35.0, 9.5, -40.0),
            Point3::new(35.0, 8.5, -40.0),
        ],
        &star,
    )));

    world.add(Object::Triangle(Triangle::new(
        &[
            Point3::new(23.0, 17.6, 30.0),
            Point3::new(23.3, 17.0, 30.0),
            Point3::new(22.7, 17.0, 30.0),
        ],
        &star,
    )));
    world.add(Object::Triangle(Triangle::new(
        &[
            Point3::new(23.0, 16.4, 30.0),
            Point3::new(23.3, 17.0, 30.0),
            Point3::new(22.7, 17.0, 30.0),
        ],
        &star,
    )));
    world.add(Object::Triangle(Triangle::new(
        &[
            Point3::new(23.6, 17.0, 30.0),
            Point3::new(23.0, 17.3, 30.0),
            Point3::new(23.0, 16.7, 30.0),
        ],
        &star,
    )));
    world.add(Object::Triangle(Triangle::new(
        &[
            Point3::new(22.4, 17.0, 30.0),
            Point3::new(23.0, 17.3, 30.0),
            Point3::new(23.0, 16.7, 30.0),
        ],
        &star,
    )));

    world.add(Object::Triangle(Triangle::new(
        &[
            Point3::new(30.0, 15.0, 30.0),
            Point3::new(30.5, 14.0, 30.0),
            Point3::new(29.5, 14.0, 30.0),
        ],
        &star,
    )));
    world.add(Object::Triangle(Triangle::new(
        &[
            Point3::new(30.0, 13.0, 30.0),
            Point3::new(30.5, 14.0, 30.0),
            Point3::new(29.5, 14.0, 30.0),
        ],
        &star,
    )));
    world.add(Object::Triangle(Triangle::new(
        &[
            Point3::new(29.0, 14.0, 30.0),
            Point3::new(30.0, 14.5, 30.0),
            Point3::new(30.0, 13.5, 30.0),
        ],
        &star,
    )));
    world.add(Object::Triangle(Triangle::new(
        &[
            Point3::new(31.0, 14.0, 30.0),
            Point3::new(30.0, 14.5, 30.0),
            Point3::new(30.0, 13.5, 30.0),
        ],
        &star,
    )));

    world.add(Object::Triangle(Triangle::new(
        &[
            Point3::new(23.0, 13.6, 30.0),
            Point3::new(23.3, 13.0, 30.0),
            Point3::new(22.7, 13.0, 30.0),
        ],
        &star,
    )));
    world.add(Object::Triangle(Triangle::new(
        &[
            Point3::new(23.0, 12.4, 30.0),
            Point3::new(23.3, 13.0, 30.0),
            Point3::new(22.7, 13.0, 30.0),
        ],
        &star,
    )));
    world.add(Object::Triangle(Triangle::new(
        &[
            Point3::new(23.6, 13.0, 30.0),
            Point3::new(23.0, 13.3, 30.0),
            Point3::new(23.0, 12.7, 30.0),
        ],
        &star,
    )));
    world.add(Object::Triangle(Triangle::new(
        &[
            Point3::new(22.4, 13.0, 30.0),
            Point3::new(23.0, 13.3, 30.0),
            Point3::new(23.0, 12.7, 30.0),
        ],
        &star,
    )));
    world
}

pub fn random_scene() -> Hittablelist {
    let mut world = Hittablelist::default_new();

    let checker = Some(Box::new(Texture::Checkertexture(
        Checkertexture::new_from_color(&Color::new(0.2, 0.3, 0.1), &Color::new(0.9, 0.9, 0.9)),
    )));

    let ground_material = Some(Box::new(Material::Lambertian(Lambertian::new_from_ptr(
        &checker,
    ))));
    world.add(Object::Sphere(Sphere::new(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        &ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random_double();
            let center = Point3::new(
                (a as f64) + 0.9 * rand::random_double(),
                0.2,
                (b as f64) + 0.9 * rand::random_double(),
            );

            if (center.copy() - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Some(Box::new(Material::Lambertian(Lambertian::new(
                        &albedo.copy(),
                    ))));
                    let center2 =
                        center.copy() + Vec3::new(0.0, rand::random_double_between(0.0, 0.5), 0.0);
                    world.add(Object::Movingsphere(Movingsphere::new(
                        &center.copy(),
                        &center2.copy(),
                        0.0,
                        1.0,
                        0.2,
                        &sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_between(0.5, 1.0);
                    let fuzz = rand::random_double_between(0.0, 0.5);
                    let sphere_material =
                        Some(Box::new(Material::Metal(Metal::new(&albedo, fuzz))));
                    world.add(Object::Sphere(Sphere::new(
                        &center.copy(),
                        0.2,
                        &sphere_material,
                    )));
                } else {
                    let sphere_material =
                        Some(Box::new(Material::Dielectric(Dielectric::new(1.5))));
                    world.add(Object::Sphere(Sphere::new(
                        &center.copy(),
                        0.2,
                        &sphere_material,
                    )));
                }
            }
        }
    }

    let material1 = Some(Box::new(Material::Dielectric(Dielectric::new(1.5))));
    world.add(Object::Sphere(Sphere::new(
        &Point3::new(0.0, 1.0, 0.0),
        1.0,
        &material1,
    )));

    let material2 = Some(Box::new(Material::Lambertian(Lambertian::new(
        &Color::new(0.4, 0.2, 0.1),
    ))));
    world.add(Object::Sphere(Sphere::new(
        &Point3::new(-4.0, 1.0, 0.0),
        1.0,
        &material2,
    )));

    let material3 = Some(Box::new(Material::Metal(Metal::new(
        &Color::new(0.7, 0.6, 0.5),
        0.0,
    ))));
    world.add(Object::Sphere(Sphere::new(
        &Point3::new(4.0, 1.0, 0.0),
        1.0,
        &material3,
    )));

    world
}

fn main() {
    print!("{}[2J", 27 as char); // Clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Set cursor position as 1,1
    println!(
        "\n         {}    {}\n",
        style("rainforest's Ray Tracer").cyan(),
        style(format!("v{}", env!("CARGO_PKG_VERSION"))).yellow(),
    );
    println!(
        "{} 💿 {}",
        style("[1/5]").bold().dim(),
        style("Initlizing...").green()
    );
    let begin_time = Instant::now();

    const THREAD_NUMBER: usize = 7;
    let quality = 100; // From 0 to 100
    let image_width = 800;
    let aspect_ratio = 16.0 / 9.0;
    let image_height = ((image_width as f64) / aspect_ratio) as u32;
    let samples_per_pixel = 100; //记得改成500
    let path = "output/output.jpg";
    let max_depth = 50;

    println!(
        "Image size: {}\nJPEG quality: {}",
        style(image_width.to_string() + &"x".to_string() + &image_height.to_string()).yellow(),
        style(quality.to_string()).yellow(),
    );

    // Create image data
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    // Progress bar UI powered by library `indicatif`
    // Get environment variable CI, which is true for GitHub Action

    let lookfrom;
    let lookat;
    let vfov;
    let mut aperture = 0.0;
    let mut background = Color::new(0.0, 0.0, 0.0);
    match 0 {
        1 => {
            //world = random_scene();
            background = Color::new(0.7, 0.8, 1.0);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.1;
        }
        2 => {
            //world = two_spheres();
            background = Color::new(0.7, 0.8, 1.0);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        3 => {
            //world = two_perlin_spheres();
            background = Color::new(0.7, 0.8, 1.0);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        4 => {
            //world = earth();
            background = Color::new(0.7, 0.8, 1.0);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        5 => {
            //world = simple_light();
            lookfrom = Point3::new(26.0, 3.0, 6.0);
            lookat = Point3::new(0.0, 2.0, 0.0);
            vfov = 20.0;
        }
        6 => {
            //world = cornell_box();
            lookfrom = Point3::new(278.0, 278.0, -800.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        7 => {
            //world = cornell_smoke();
            lookfrom = Point3::new(278.0, 278.0, -800.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        8 => {
            //world = final_scene();
            lookfrom = Point3::new(478.0, 278.0, -600.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        9 => {
            //world = obj();
            background = Color::new(0.7, 0.8, 1.0);
            lookfrom = Point3::new(3.0, 0.0, 10.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        _ => {
            //world = myworld();
            //background = Color::new(0.098, 0.098, 0.439);
            lookfrom = Point3::new(0.0, 5.0, 40.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 40.0;
        }
    }
    let world = myworld();
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let cam = Camera::new(
        &lookfrom,
        &lookat,
        &vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    // Generate image
    println!(
        "{} 🚀 {} {} {}",
        style("[2/5]").bold().dim(),
        style("Rendering with").green(),
        style((THREAD_NUMBER + 2).to_string()).yellow(),
        style("Threads...").green(),
    );
    let SECTION_LINE_NUM: usize = (image_height as usize) / (THREAD_NUMBER * 4);
    let mut output_pixel_color = Vec::<Color>::new();
    let mut thread_pool = Vec::<_>::new();
    let multiprogress = Arc::new(MultiProgress::new());
    multiprogress.set_move_cursor(true); // turn on this to reduce flickering

    for thread_id in 0..(THREAD_NUMBER + 2) {
        let line_beg = match thread_id {
            0 => 0,
            1 => SECTION_LINE_NUM * 6,
            2 => SECTION_LINE_NUM * 8,
            3 => SECTION_LINE_NUM * 10,
            4 => SECTION_LINE_NUM * 12,
            5 => SECTION_LINE_NUM * 13,
            6 => SECTION_LINE_NUM * 14,
            7 => SECTION_LINE_NUM * 15,
            _ => SECTION_LINE_NUM * 16,
        };
        let line_end = match thread_id {
            0 => SECTION_LINE_NUM * 6,
            1 => SECTION_LINE_NUM * 8,
            2 => SECTION_LINE_NUM * 10,
            3 => SECTION_LINE_NUM * 12,
            4 => SECTION_LINE_NUM * 13,
            5 => SECTION_LINE_NUM * 14,
            6 => SECTION_LINE_NUM * 15,
            7 => SECTION_LINE_NUM * 16,
            _ => image_height as usize,
        };
        /*
        let line_beg = SECTION_LINE_NUM * thread_id;
        let mut line_end = line_beg + SECTION_LINE_NUM;
        if line_end > (image_height as usize)
            || (thread_id == THREAD_NUMBER - 1 && line_end < (image_height as usize))
        {
            line_end = image_height as usize;
        }
        */

        // Secene

        let world = world.copy();
        let cam = cam.copy();
        let mp = multiprogress.clone();
        let progress_bar = mp.add(ProgressBar::new(
            ((line_end - line_beg) * (image_width as usize)) as u64,
        ));
        progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] [{pos}/{len}] ({eta})")
        .progress_chars("#>-"));

        let (tx, rx) = mpsc::channel();

        thread_pool.push((
            thread::spawn(move || {
                let mut progress = 0;
                progress_bar.set_position(0);

                let channel_send = tx;

                let mut section_pixel_color = Vec::<Color>::new();

                //let mut rnd = rand::thread_rng();

                for y in line_beg..line_end {
                    for x in 0..image_width {
                        let mut pixel_color = Color::default_new();
                        for _i in 0..samples_per_pixel {
                            let u = (x as f64 + rand::random_double()) / (image_width - 1) as f64;
                            let v = (y as f64 + rand::random_double()) / (image_height - 1) as f64;
                            let ray = cam.get_ray(u, v);
                            pixel_color += ray_color(&ray, &background, &world, max_depth);
                        }
                        section_pixel_color.push(pixel_color);

                        progress += 1;
                        progress_bar.set_position(progress);
                    }
                }
                channel_send.send(section_pixel_color).unwrap();
                progress_bar.finish_with_message("Finished.");
            }),
            rx,
        ));
    }
    // 等待所有线程结束
    multiprogress.join().unwrap();

    //========================================================

    println!(
        "{} 🚛 {}",
        style("[3/5]").bold().dim(),
        style("Collecting Threads Results...").green(),
    );

    //let mut thread_finish_successfully = true;
    let collecting_progress_bar = ProgressBar::new((THREAD_NUMBER + 2) as u64);
    // join 和 recv 均会阻塞主线程
    for thread_id in 0..(THREAD_NUMBER + 2) {
        let thread = thread_pool.remove(0);
        match thread.0.join() {
            Ok(_) => {
                let mut received = thread.1.recv().unwrap();
                output_pixel_color.append(&mut received);
                collecting_progress_bar.inc(1);
            }
            Err(_) => {
                //thread_finish_successfully = false;
                println!(
                    "      ⚠️ {}{}{}",
                    style("Joining the ").red(),
                    style(thread_id.to_string()).yellow(),
                    style("th thread failed!").red(),
                );
            }
        }
    }

    collecting_progress_bar.finish_and_clear();

    println!(
        "{} 🏭 {}",
        style("[4/5]").bold().dim(),
        style("Generating Image...").green()
    );

    let mut pixel_id = 0;

    for y in 0..image_height as u32 {
        for x in 0..image_width as u32 {
            let pixel_color = output_pixel_color[pixel_id].calc_color(samples_per_pixel);

            let pixel = img.get_pixel_mut(x, image_height - y - 1);
            *pixel = image::Rgb(pixel_color.to_u8_array());

            pixel_id += 1;
        }
    }

    // Output image to file
    println!(
        "{} 🥽 {}",
        style("[5/5]").bold().dim(),
        style("Outping Image...").green()
    );
    println!(
        "         Image format:              {}",
        style("JPEG").yellow()
    );
    println!(
        "         JPEG image quality:        {}",
        style(quality.to_string()).yellow()
    );
    println!("Ouput image as \"{}\"", style(path).yellow());
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        // Err(_) => panic!("Outputting image fails."),
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    println!(
        "\n      🎉 {}\n      🕒 Elapsed Time: {}",
        style("All Work Done.").bold().green(),
        style(HumanDuration(begin_time.elapsed())).yellow(),
    );
    println!("\n");

    exit(0);
}
