use crate::{interval::Interval, ray::Ray, Point3};
use glam::DVec3;
use std::ops::Add;

#[derive(Clone, Default)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_points(a: &Point3, b: &Point3) -> Self {
        Self {
            x: Interval::new(f64::min(a.x, b.x), f64::max(a.x, b.x)),
            y: Interval::new(f64::min(a.y, b.y), f64::max(a.y, b.y)),
            z: Interval::new(f64::min(a.z, b.z), f64::max(a.z, b.z)),
        }
    }

    pub fn from_aabbs(box_0: &Aabb, box_1: &Aabb) -> Self {
        Self {
            x: Interval::from_intervals(&box_0.x, &box_1.x),
            y: Interval::from_intervals(&box_0.y, &box_1.y),
            z: Interval::from_intervals(&box_0.z, &box_1.z),
        }
    }

    pub fn pad(&self) -> Aabb {
        // Return an AABB that has no side narrower than some delta, padding if necessary.
        let delta = 0.0001;
        let new_x = if self.x.size() >= delta {
            self.x
        } else {
            self.x.expand(delta)
        };
        let new_y = if self.y.size() >= delta {
            self.y
        } else {
            self.y.expand(delta)
        };
        let new_z = if self.z.size() >= delta {
            self.z
        } else {
            self.z.expand(delta)
        };

        Self::new(new_x, new_y, new_z)
    }

    pub fn axis(&self, index: usize) -> &Interval {
        match index {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> bool {
        for a in 0..3 {
            // let t0 = f64::min(
            //     (self.axis(a).min - ray.origin()[a]) / ray.direction()[a],
            //     (self.axis(a).max - ray.origin()[a]) / ray.direction()[a],
            // );

            // let t1 = f64::max(
            //     (self.axis(a).min - ray.origin()[a]) / ray.direction()[a],
            //     (self.axis(a).max - ray.origin()[a]) / ray.direction()[a],
            // );

            // ray_t.min = f64::max(t0, ray_t.min);
            // ray_t.max = f64::min(t1, ray_t.max);

            let inv_d = 1.0 / ray.direction[a];
            let mut t0 = (self.axis(a).min - ray.origin[a]) * inv_d;
            let mut t1 = (self.axis(a).max - ray.origin[a]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            let ray_t_min = if t0 > ray_t.min { t0 } else { ray_t.min };
            let ray_t_max = if t1 < ray_t.max { t1 } else { ray_t.max };

            if ray_t_max <= ray_t_min {
                return false;
            }
        }
        true
    }
}

impl Add<&DVec3> for &Aabb {
    type Output = Aabb;

    fn add(self, offset: &DVec3) -> Self::Output {
        Aabb::new(&self.x + offset.x, &self.y + offset.y, &self.z + offset.z)
    }
}

impl Add<&Aabb> for &DVec3 {
    type Output = Aabb;

    fn add(self, bounding_box: &Aabb) -> Self::Output {
        bounding_box + self
    }
}
