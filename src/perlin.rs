use crate::{random_dvec3_range, random_usize_range, Point3};
use glam::DVec3;

pub struct Perlin {
    random_vec: Vec<DVec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let mut random_vec = Vec::with_capacity(Self::POINT_COUNT);

        for _ in 0..Self::POINT_COUNT {
            random_vec.push(random_dvec3_range(-1.0, 1.0).normalize());
            // random_vec.push(random_in_unit_sphere());
        }

        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();

        Self {
            random_vec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, point: &Point3) -> f64 {
        // return 10.0;

        let u = point.x - point.x.floor();
        let v = point.y - point.y.floor();
        let w = point.z - point.z.floor();

        let i = point.x.floor() as i32;
        let j = point.y.floor() as i32;
        let k = point.z.floor() as i32;
        let mut c = [[[DVec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.random_vec[self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]];
                }
            }
        }

        // self.random_float[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
        Self::perlin_interp(c, u, v, w)
    }

    pub fn turb(&self, point: &Point3, depth: usize) -> f64 {
        // depth default to 7
        let mut accum = 0.0;
        let mut temp_point = *point;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_point);
            weight *= 0.5;
            temp_point *= 2.0;
        }

        accum.abs()
    }

    fn perlin_generate_perm() -> Vec<usize> {
        let mut perm: Vec<usize> = Vec::with_capacity(Self::POINT_COUNT);

        for i in 0..Self::POINT_COUNT {
            perm.push(i);
        }

        Self::permute(&mut perm, Self::POINT_COUNT);
        perm
    }

    fn permute(perm: &mut Vec<usize>, n: usize) {
        for i in (1..n).rev() {
            let target = random_usize_range(0, i);
            perm.swap(i, target);
        }
    }

    fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f64 * u + (1.0 - i as f64) * (1.0 - u))
                        * (j as f64 * v + (1.0 - j as f64) * (1.0 - v))
                        * (k as f64 * w + (1.0 - k as f64) * (1.0 - w))
                        * c[i][j][k];
                }
            }
        }

        accum
    }

    fn perlin_interp(c: [[[DVec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = DVec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                        * c[i][j][k].dot(weight_v);
                }
            }
        }

        accum
    }
}
