pub use crate::vec3::vec3::Vec3;
pub use crate::vec3::Color;

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

pub fn write_color(color: Color, samples_per_pixel: i32) -> Color {
    let mut r = color.x();
    let mut g = color.y();
    let mut b = color.z();

    let scale = 1.0 / (samples_per_pixel as f64);
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    let intx: i32 = (256.0 * clamp(r, 0.0, 0.999)) as i32;
    let inty: i32 = (256.0 * clamp(g, 0.0, 0.999)) as i32;
    let intz: i32 = (256.0 * clamp(b, 0.0, 0.999)) as i32;
    //print!("{} {} {}\n", intx, inty, intz);

    Color {
        x: intx as f64,
        y: inty as f64,
        z: intz as f64,
    }
}
