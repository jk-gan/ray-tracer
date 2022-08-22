use crate::{
    camera::Camera,
    color::{write_color, Color},
    hittable::{Hittable, HittableList},
    interval::Interval,
    random_f64,
    ray::Ray,
};
use indicatif::ProgressBar;
use rayon::prelude::*;
use std::time::Instant;

pub struct Scene {
    pub world: HittableList,
    pub camera: Camera,
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    pub samples_per_pixel: usize,
    pub max_depth: usize,
    pub background_color: Color,
}

impl Scene {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: usize,
        max_depth: usize,
    ) -> Self {
        Self {
            world: HittableList::default(),
            camera: Camera::default(),
            aspect_ratio,
            image_width,
            image_height: (image_width as f64 / aspect_ratio) as u32,
            samples_per_pixel,
            max_depth,
            background_color: Color::new(0.0, 0.0, 0.0),
        }
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f64) {
        self.aspect_ratio = aspect_ratio;
        self.image_height = calculate_image_height(self.image_width, aspect_ratio);
    }

    pub fn set_image_width(&mut self, image_width: u32) {
        self.image_width = image_width;
        self.image_height = calculate_image_height(image_width, self.aspect_ratio);
    }

    pub fn render(&mut self) {
        let start = Instant::now();
        self.camera.init(self.aspect_ratio);

        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("n255");

        let total_count = self.image_height * self.image_width as u32;
        let progress_bar = ProgressBar::new(total_count as u64);

        let image = (0..self.image_height)
            .into_par_iter()
            .rev()
            .map(|j| {
                (0..self.image_width)
                    .into_par_iter()
                    .map(|i| {
                        let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                        for _ in 0..self.samples_per_pixel {
                            let u = (i as f64 + random_f64()) / (self.image_width as f64 - 1.0);
                            let v = (j as f64 + random_f64()) / (self.image_height as f64 - 1.0);

                            let ray = self.camera.get_ray(u, v);
                            pixel_color += self.ray_color(ray, self.max_depth);
                        }
                        progress_bar.inc(1);
                        pixel_color
                    })
                    .collect::<Vec<Color>>()
            })
            .collect::<Vec<Vec<Color>>>();

        for row in image {
            for color in row {
                write_color(color, self.samples_per_pixel);
            }
        }

        eprintln!("Took {:?} wall time", start.elapsed());
    }

    fn ray_color(&self, ray: Ray, depth: usize) -> Color {
        // if we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        // if the ray hits noting, return the background_color
        if let Some(hitted_record) = self.world.hit(&ray, Interval::new(0.001, f64::MAX)) {
            // let mut scattered_ray = Ray::default();
            // let mut attenuation = Color::default();
            let color_from_emission = hitted_record.material.emitted(
                hitted_record.u,
                hitted_record.v,
                &hitted_record.point,
            );

            if let Some((attenuation, scattered_ray)) =
                hitted_record.material.scatter(&ray, &hitted_record)
            {
                let color_from_scatter = attenuation * self.ray_color(scattered_ray, depth - 1);

                return color_from_emission + color_from_scatter;
            } else {
                return color_from_emission;
            }
        } else {
            return self.background_color;
        }

        // let unit_direction = ray.direction().normalize();
        // let t = 0.5 * (unit_direction.y + 1.0);

        // // lerp
        // // blended_value = (1 - t) * start_value + t * end_value
        // (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn calculate_image_height(image_width: u32, aspect_ratio: f64) -> u32 {
    (image_width as f64 / aspect_ratio) as u32
}
