use glam::DVec3;

use crate::{random_in_unit_disk, ray::Ray, Point3};

pub struct Camera {
    // aspect_ratio: f32,
    // viewport_height: f32,
    // viewport_width: f32,
    // focal_length: f32,
    pub vfov: f64,
    pub aperture: f64,
    pub focus_dist: f64,

    pub look_from: Point3,
    pub look_at: Point3,
    pub vup: DVec3,

    origin: Point3,
    lower_left_corner: DVec3,
    horizontal: DVec3,
    vertical: DVec3,

    u: DVec3,
    v: DVec3,
    w: DVec3,
    lens_radius: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            vfov: 40.0,
            aperture: 0.0,
            focus_dist: 10.0,
            look_from: Point3::new(0.0, 0.0, -1.0),
            look_at: Point3::new(0.0, 0.0, 0.0),
            vup: DVec3::new(0.0, 1.0, 0.0),
            origin: Default::default(),
            lower_left_corner: Default::default(),
            horizontal: Default::default(),
            vertical: Default::default(),
            u: Default::default(),
            v: Default::default(),
            w: Default::default(),
            lens_radius: Default::default(),
        }
    }
}

impl Camera {
    pub fn init(&mut self, aspect_ratio: f64) {
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        self.w = (self.look_from - self.look_at).normalize();
        self.u = self.vup.cross(self.w).normalize();
        self.v = self.w.cross(self.u);

        self.origin = self.look_from;
        self.horizontal = self.focus_dist * viewport_width * self.u;
        self.vertical = self.focus_dist * viewport_height * self.v;
        self.lower_left_corner =
            self.origin - self.horizontal / 2.0 - self.vertical / 2.0 - self.focus_dist * self.w;
        self.lens_radius = self.aperture / 2.0;
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
