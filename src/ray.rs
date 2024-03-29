use crate::Point3;
use glam::DVec3;

#[derive(Clone, Copy, Default)]
pub struct Ray {
    pub origin: Point3,
    pub direction: DVec3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: DVec3, time: f64) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        // P(t) = A + tb
        // where
        //      A = ray origin
        //      b = ray direction

        self.origin + (t * self.direction)
    }
}
