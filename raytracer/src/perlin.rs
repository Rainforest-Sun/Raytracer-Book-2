#![allow(unused_mut)]
#![allow(clippy::many_single_char_names)]
#![allow(unused_variables)]
pub use crate::rand;
pub use crate::texture::Value;
pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;
const POINT_COUNT: i32 = 256;

pub struct Perlin {
    //pub ranfloat: [f64; POINT_COUNT as usize],
    pub ranvec: [Vec3; POINT_COUNT as usize],
    pub perm_x: [i32; POINT_COUNT as usize],
    pub perm_y: [i32; POINT_COUNT as usize],
    pub perm_z: [i32; POINT_COUNT as usize],
}

impl Perlin {
    pub fn permute(p: &mut [i32; POINT_COUNT as usize], n: i32) {
        for i in (1..n).rev() {
            let target = rand::random_int_between(0, i);
            p.swap(i as usize, target as usize);
        }
    }

    pub fn perlin_generate_perm() -> [i32; POINT_COUNT as usize] {
        let mut p: [i32; POINT_COUNT as usize] = [0; POINT_COUNT as usize];
        for i in 0..POINT_COUNT {
            p[i as usize] = i;
        }
        Perlin::permute(&mut p, POINT_COUNT);
        p
    }

    pub fn turb(&self, p: &Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p.copy();
        let mut weight = 1.0;

        for i in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }

    pub fn default_new() -> Perlin {
        let mut ranvec: [Vec3; POINT_COUNT as usize] = [Vec3::default_new(); POINT_COUNT as usize];
        for i in 0..POINT_COUNT {
            ranvec[i as usize] = Vec3::unit_vector(&Vec3::random_between(-1.0, 1.0));
        }

        let perm_x = Perlin::perlin_generate_perm();
        let perm_y = Perlin::perlin_generate_perm();
        let perm_z = Perlin::perlin_generate_perm();

        Perlin {
            ranvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn copy(&self) -> Perlin {
        let mut ranvec: [Vec3; POINT_COUNT as usize] = [Vec3::default_new(); POINT_COUNT as usize];
        for i in 0..POINT_COUNT {
            ranvec[i as usize] = self.ranvec[i as usize].copy();
        }
        Perlin {
            ranvec,
            perm_x: self.perm_x,
            perm_y: self.perm_y,
            perm_z: self.perm_z,
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i: i32 = p.x().floor() as i32;
        let j: i32 = p.y().floor() as i32;
        let k: i32 = p.z().floor() as i32;
        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::default_new(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di as usize][dj as usize][dk as usize] = self.ranvec[(self.perm_x
                        [((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize])
                        as usize]
                        .copy();
                }
            }
        }

        Perlin::perlin_interp(&c, u, v, w)
    }

    pub fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += ((i as f64) * uu + (1.0 - (i as f64)) * (1.0 - uu))
                        * ((j as f64) * vv + (1.0 - (j as f64)) * (1.0 - vv))
                        * ((k as f64) * ww + (1.0 - (k as f64)) * (1.0 - ww))
                        * Vec3::dot(&c[i as usize][j as usize][k as usize], &weight_v);
                }
            }
        }
        accum
    }
}
