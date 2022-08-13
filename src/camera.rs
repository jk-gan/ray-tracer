use glam::DVec3;

use crate::{ray::Ray, Point3};

const ASPECT_RATIO: f32 = 16.0 / 9.0;

pub struct Camera {
    // aspect_ratio: f32,
    // viewport_height: f32,
    // viewport_width: f32,
    // focal_length: f32,
    origin: Point3,
    horizontal: DVec3,
    vertical: DVec3,
    lower_left_corner: DVec3,
}

impl Camera {
    pub fn new() -> Self {
        let viewport_height = 2.0;
        let viewport_width = ASPECT_RATIO * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = DVec3::new(viewport_width as f64, 0.0, 0.0);
        let vertical = DVec3::new(0.0, viewport_height as f64, 0.0);

        Self {
            origin: DVec3::new(0.0, 0.0, 0.0),
            horizontal: DVec3::new(viewport_width as f64, 0.0, 0.0),
            vertical: DVec3::new(0.0, viewport_height as f64, 0.0),
            lower_left_corner: origin
                - horizontal / 2.0
                - vertical / 2.0
                - DVec3::new(0.0, 0.0, focal_length),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
