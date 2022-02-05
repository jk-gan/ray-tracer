use crate::Point3;
use glam::DVec3;

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
        self.origin + (t * self.direction)
    }
}
