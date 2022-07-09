use std::{fs::File, process::exit};

use image::{ImageBuffer, RgbImage};

use console::style;
use indicatif::{ProgressBar, ProgressStyle};

pub mod camera;
pub mod color;
pub mod dielectric;
pub mod hittable;
pub mod hittable_list;
pub mod lambertian;
pub mod material;
pub mod metal;
pub mod rand;
pub mod ray;
pub mod sphere;
pub mod vec3;

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
pub use crate::ray::Ray;
pub use crate::sphere::Sphere;
pub use crate::vec3::Color;
pub use crate::vec3::Point3;
pub use crate::vec3::Vec3; //文件位置::mod名::struct名

pub fn ray_color(r: &Ray, world: &Hittablelist, depth: i32) -> Color {
    let mut rec = Hitrecord::default_new();
    let inf: f64 = 1.79769e+308;

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(&r, 0.001, inf, &mut rec) {
        let mut scattered = Ray::default_new();
        let mut attenuation = Color::default_new();
        if let Some(in_mat_ptr) = &rec.mat_ptr {
            if in_mat_ptr.scatter(&r, &rec, &mut attenuation, &mut scattered) {
                return ray_color(&scattered, &world, depth - 1) * attenuation;
            } else {
                return Color::new(0.0, 0.0, 0.0);
            }
        }
    }
    let unit_direction = Vec3::unit_vector(&r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

pub fn random_scene() -> Hittablelist {
    let mut world = Hittablelist::default_new();

    let ground_material = Some(Box::new(Material::Lambertian(Lambertian::new(
        &Color::new(0.5, 0.5, 0.5),
    ))));
    world.add(Box::new(Object::Sphere(Sphere::new(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        &ground_material,
    ))));

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
                    world.add(Box::new(Object::Sphere(Sphere::new(
                        &center.copy(),
                        0.2,
                        &sphere_material,
                    ))));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_between(0.5, 1.0);
                    let fuzz = rand::random_double_between(0.0, 0.5);
                    let sphere_material =
                        Some(Box::new(Material::Metal(Metal::new(&albedo, fuzz))));
                    world.add(Box::new(Object::Sphere(Sphere::new(
                        &center.copy(),
                        0.2,
                        &sphere_material,
                    ))));
                } else {
                    let sphere_material =
                        Some(Box::new(Material::Dielectric(Dielectric::new(1.5))));
                    world.add(Box::new(Object::Sphere(Sphere::new(
                        &center.copy(),
                        0.2,
                        &sphere_material,
                    ))));
                }
            }
        }
    }

    let material1 = Some(Box::new(Material::Dielectric(Dielectric::new(1.5))));
    world.add(Box::new(Object::Sphere(Sphere::new(
        &Point3::new(0.0, 1.0, 0.0),
        1.0,
        &material1,
    ))));

    let material2 = Some(Box::new(Material::Lambertian(Lambertian::new(
        &Color::new(0.4, 0.2, 0.1),
    ))));
    world.add(Box::new(Object::Sphere(Sphere::new(
        &Point3::new(-4.0, 1.0, 0.0),
        1.0,
        &material2,
    ))));

    let material3 = Some(Box::new(Material::Metal(Metal::new(
        &Color::new(0.7, 0.6, 0.5),
        0.0,
    ))));
    world.add(Box::new(Object::Sphere(Sphere::new(
        &Point3::new(4.0, 1.0, 0.0),
        1.0,
        &material3,
    ))));

    world
}

fn main() {
    print!("{}[2J", 27 as char); // Clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Set cursor position as 1,1

    let height = 800;
    let width = 1200;
    let quality = 60; // From 0 to 100
    let path = "output/output.jpg";

    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = ((image_width as f64) / aspect_ratio) as i32;
    let samples_per_pixel = 500; //记得改成500
    let max_depth = 50;

    println!(
        "Image size: {}\nJPEG quality: {}",
        style(width.to_string() + &"x".to_string() + &height.to_string()).yellow(),
        style(quality.to_string()).yellow(),
    );

    // Create image data
    let mut img: RgbImage = ImageBuffer::new(width, height);
    // Progress bar UI powered by library `indicatif`
    // Get environment variable CI, which is true for GitHub Action
    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };
    progress.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] [{pos}/{len}] ({eta})")
        .progress_chars("#>-"));

    let world = random_scene();

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(
        &lookfrom,
        &lookat,
        &vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    //print!("P3\n{} {}\n255\n", image_width, image_height);
    // Generate image
    for y in 0..height {
        for x in 0..width {
            let mut pixel_color1 = Color::new(0.0, 0.0, 0.0);

            for s in 0..samples_per_pixel {
                let u = (((x as f64) + rand::random_double()) / ((image_width - 1) as f64)) as f64;
                let v = (((y as f64) + rand::random_double()) / ((image_height - 1) as f64)) as f64;
                let r = cam.get_ray(u, v);
                pixel_color1 += ray_color(&r, &world, max_depth);
            }
            let pixel_color2 = color::write_color(pixel_color1, samples_per_pixel);

            let pixel_color = [
                pixel_color2.x() as u8,
                pixel_color2.y() as u8,
                pixel_color2.z() as u8,
            ];
            let pixel = img.get_pixel_mut(x, height - y - 1);
            *pixel = image::Rgb(pixel_color);
            progress.inc(1);
        }
    }
    progress.finish();

    // Output image to file
    println!("Ouput image as \"{}\"", style(path).yellow());
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        // Err(_) => panic!("Outputting image fails."),
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    exit(0);
}
