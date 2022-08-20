use glam::DVec3;
use rand::Rng;

fn perlin_generate() -> Vec<DVec3> {
    let mut rng = rand::thread_rng();
    let mut p = Vec::with_capacity(256);
    for _ in 0..256 {
        p.push(DVec3::new(
            -1.0 + 2.0 * rng.gen::<f64>(),
            -1.0 + 2.0 * rng.gen::<f64>(),
            -1.0 + 2.0 * rng.gen::<f64>()).normalize());
    }
    p
}

fn permute(p: &mut [usize], n: usize) {
    let mut rng = rand::thread_rng();
    for i in (0..n as usize).rev() {
        let target = rng.gen_range(0..=i);
        p.swap(i, target);
    }
}

fn perlin_generate_perm() -> Vec<usize> {
    let mut p = Vec::with_capacity(256);
    for i in 0..256 {
        p.push(i);
    }
    permute(&mut p, 256);
    p
}

fn perlin_interp(c: &[[[DVec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut accum = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight = DVec3::new(u - i as f64, v - j as f64, w - k as f64);
                accum += (i as f64 * uu + (1 - i) as f64 * (1.0 - uu)) *
                    (j as f64 * vv + (1 - j) as f64 * (1.0 - vv)) *
                    (k as f64 * ww + (1 - k) as f64 * (1.0 - ww)) *
                    c[i][j][k].dot(weight);
            }
        }
    }
    accum
}

#[derive(Clone)]
pub struct Perlin {
    ran_vec: Vec<DVec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>
}

impl Perlin {
    pub fn new() -> Self {
        Perlin {
            ran_vec: perlin_generate(),
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm()
        }
    }

    fn noise(&self, p: &DVec3) -> f64 {
        let u = p.x - f64::floor(p.x);
        let v = p.y - f64::floor(p.y);
        let w = p.z - f64::floor(p.z);
        let i = f64::floor(p.x) as usize;
        let j = f64::floor(p.y) as usize;
        let k = f64::floor(p.z) as usize;
        let mut c = [[[DVec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] =
                        self.ran_vec[self.perm_x[(i + di) & 255] ^ self.perm_y[(j + dj) & 255] ^ self.perm_z[(k + dk) & 255]]
                }
            }
        };
        perlin_interp(&c, u, v, w)
    }

    pub fn turb(&self, p: &DVec3, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;
        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        f64::abs(accum)
    }
}