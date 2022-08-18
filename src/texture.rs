use crate::{color::Color, interval::Interval, rt_image::RTImage, Point3};
use std::{path::Path, sync::Arc};

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

pub struct ImageTexture {
    image: RTImage,
}

impl ImageTexture {
    pub fn new(path: &Path) -> Self {
        Self {
            image: RTImage::new(path),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _point: &Point3) -> Color {
        // Clamp input texture coordinates to [0,1] x [1,0]
        let u = Interval::new(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v); // Flip V to image coordinates

        let mut i = (u * self.image.width() as f64) as usize;
        let mut j = (v * self.image.height() as f64) as usize;
        if i > self.image.width() as usize - 1 {
            i = self.image.width() as usize - 1
        }
        if j > self.image.height() as usize - 1 {
            j = self.image.height() as usize - 1
        }

        let index = 3 * i + 3 * self.image.width() as usize * j;
        let r = self.image.data[index] as f64 / 255.0;
        let g = self.image.data[index + 1] as f64 / 255.0;
        let b = self.image.data[index + 2] as f64 / 255.0;
        Color::new(r, g, b)
    }
}
