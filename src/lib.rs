pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod color;
pub mod constant_medium;
pub mod hittable;
pub mod interval;
pub mod material;
pub mod perlin;
pub mod ray;
pub mod rt_image;
pub mod scene;
pub mod texture;

use glam::DVec3;
use nanorand::Rng;
pub type Point3 = DVec3;

#[inline]
pub fn random_f64() -> f64 {
    let mut rng = nanorand::tls_rng();

    rng.generate_range(0..=1000) as f64 / 1001.0
}

#[inline]
pub fn random_f64_range(min: f64, max: f64) -> f64 {
    // Returns a random real in [min,max).
    min + (max - min) * random_f64()
}

#[inline]
pub fn random_isize_range(min: isize, max: isize) -> isize {
    // Returns a random integer in [min,max].
    random_f64_range(min as f64, (max + 1) as f64) as isize
}

#[inline]
pub fn random_usize_range(min: usize, max: usize) -> usize {
    // Returns a random integer in [min,max].
    random_f64_range(min as f64, (max + 1) as f64) as usize
}

#[inline]
pub fn random_dvec3() -> DVec3 {
    DVec3::new(random_f64(), random_f64(), random_f64())
}

#[inline]
pub fn random_dvec3_range(min: f64, max: f64) -> DVec3 {
    DVec3::new(
        random_f64_range(min, max),
        random_f64_range(min, max),
        random_f64_range(min, max),
    )
}

pub fn random_in_unit_sphere() -> DVec3 {
    loop {
        let p = random_dvec3_range(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }

        return p;
    }
}

pub fn random_unit_vertor() -> DVec3 {
    random_in_unit_sphere().normalize()
}

pub fn random_in_hemisphere(normal: &DVec3) -> DVec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(*normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn random_in_unit_disk() -> DVec3 {
    loop {
        let p = DVec3::new(
            random_f64_range(-1.0, 1.0),
            random_f64_range(-1.0, 1.0),
            0.0,
        );
        if p.length_squared() >= 1.0 {
            continue;
        }

        return p;
    }
}

#[inline]
pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        return min;
    }
    if value > max {
        return max;
    }

    value
}
