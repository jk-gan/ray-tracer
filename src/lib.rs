pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod color;
pub mod hittable;
pub mod interval;
pub mod material;
pub mod perlin;
pub mod ray;
pub mod rt_image;
pub mod scene;
pub mod texture;

use glam::DVec3;
use rand::Rng;
pub type Point3 = DVec3;

#[inline]
pub fn random_dvec3() -> DVec3 {
    let mut rng = rand::thread_rng();

    DVec3::new(rng.gen(), rng.gen(), rng.gen())
}

#[inline]
pub fn random_dvec3_range(min: f64, max: f64) -> DVec3 {
    let mut rng = rand::thread_rng();

    DVec3::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
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
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}

pub fn random_in_unit_disk() -> DVec3 {
    let mut rng = rand::thread_rng();

    loop {
        let p = DVec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
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
