use std::sync::Arc;

use crate::{color::Color, Point3};

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, point: &Point3) -> Color;
}

#[derive(Default)]
pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self { color }
    }

    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self {
            color: Color::new(r, g, b),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _point: &Point3) -> Color {
        self.color
    }
}

pub struct CheckerTexture {
    inv_scale: f64,
    pub even: Arc<dyn Texture>,
    pub odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn from_colors(scale: f64, color_1: Color, color_2: Color) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Arc::new(SolidColor::new(color_1)),
            odd: Arc::new(SolidColor::new(color_2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, point: &Point3) -> Color {
        let x = (self.inv_scale * point.x).floor() as i64;
        let y = (self.inv_scale * point.y).floor() as i64;
        let z = (self.inv_scale * point.z).floor() as i64;

        let is_even = (x + y + z) % 2 == 0;
        if is_even {
            self.even.value(u, v, point)
        } else {
            self.odd.value(u, v, point)
        }
    }
}
