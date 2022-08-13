use crate::Point3;
use glam::DVec3;

#[derive(Default)]
pub struct Ray {
    origin: Point3,
    direction: DVec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: DVec3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &DVec3 {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        // P(t) = A + tb
        // where
        //      A = ray origin
        //      b = ray direction

        self.origin + (t * self.direction)
    }
}
