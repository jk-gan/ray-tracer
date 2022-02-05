use glam::DVec3;
use indicatif::ProgressBar;
use ray_tracer::{
    color::{write_color, Color},
    ray::Ray,
    Point3,
};

// Image
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const WIDTH: u32 = 400;
const HEIGHT: u32 = (WIDTH as f32 / ASPECT_RATIO) as u32;

fn ray_color(r: &Ray) -> Color {
    let unit_direction = r.direction().normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = DVec3::new(viewport_width as f64, 0.0, 0.0);
    let vertical = DVec3::new(0.0, viewport_height as f64, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - DVec3::new(0.0, 0.0, focal_length);

    // Render
    println!("P3");
    println!("{} {}", WIDTH, HEIGHT);
    println!("n255");

    let pb = ProgressBar::new(HEIGHT as u64);

    for j in (0..HEIGHT).rev() {
        pb.set_position((HEIGHT - j) as u64);

        for i in 0..WIDTH {
            let u = i as f64 / (WIDTH as f64 - 1.0);
            let v = j as f64 / (HEIGHT as f64 - 1.0);

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&r);

            // let pixel_color = Color::new(r, g, b);
            write_color(pixel_color);
        }
    }
}
