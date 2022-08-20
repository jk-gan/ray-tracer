use glam::DVec3;
use rand::Rng;

use crate::{
    color::Color,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    texture::{SolidColor, Texture},
};
use std::sync::Arc;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<Material>,
}

impl ConstantMedium {
    pub fn from_texture(
        boundary: Arc<dyn Hittable>,
        density: f64,
        texture: Arc<dyn Texture>,
    ) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Material::Isotropic { albedo: texture }),
        }
    }

    pub fn from_color(boundary: Arc<dyn Hittable>, density: f64, color: Color) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Material::Isotropic {
                albedo: Arc::new(SolidColor::new(color)),
            }),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        ray_t: &crate::interval::Interval,
    ) -> Option<crate::hittable::HitRecord> {
        // Print occasional samples when debugging. To enable, set enable_debug true
        let mut rng = rand::thread_rng();
        let enable_debug = false;
        let debugging = enable_debug && rng.gen::<f64>() < 0.00001;

        let mut hit_record_1 = match self.boundary.hit(ray, &Interval::UNIVERSE) {
            Some(hitted_record) => hitted_record,
            None => {
                return None;
            }
        };

        let mut hit_record_2 = match self
            .boundary
            .hit(ray, &Interval::new(hit_record_1.t + 0.0001, f64::MAX))
        {
            Some(hitted_record) => hitted_record,
            None => {
                return None;
            }
        };

        if debugging {
            eprintln!(
                "\nray_t_min={}, ray_t_max={}",
                hit_record_1.t, hit_record_2.t
            );
        }

        if hit_record_1.t < ray_t.min {
            hit_record_1.t = ray_t.min;
        }
        if hit_record_2.t > ray_t.max {
            hit_record_2.t = ray_t.max;
        }

        if hit_record_1.t >= hit_record_2.t {
            return None;
        }

        if hit_record_1.t < 0.0 {
            hit_record_1.t = 0.0;
        }

        let ray_length = ray.direction().length();
        let distance_inside_boundary = (hit_record_2.t - hit_record_1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rng.gen::<f64>().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let mut hit_record = HitRecord::empty();
        hit_record.t = hit_record_1.t + hit_distance / ray_length;
        hit_record.point = ray.at(hit_record.t);

        if debugging {
            eprintln!("hit_distance = {}", hit_distance);
            eprintln!("hit_record.t = {}", hit_record.t);
            eprintln!("hit_record.point = {}", hit_record.point);
        }

        hit_record.normal = DVec3::new(1.0, 0.0, 0.0); // arbitrary
        hit_record.front_face = true; // arbitrary
        hit_record.material = self.phase_function.clone();

        Some(hit_record)
    }

    fn bounding_box(&self) -> &crate::aabb::Aabb {
        &self.boundary.bounding_box()
    }
}
