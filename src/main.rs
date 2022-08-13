use indicatif::ProgressBar;
use ray_tracer::{
    camera::Camera,
    color::{write_color, Color},
    hittable::{HitRecord, Hittable, HittableList, Sphere},
    random_f64, random_in_unit_sphere,
    ray::Ray,
    Point3, random_unit_vertor, random_in_hemisphere,
};

// Image
const ASPECT_RATIO: f32 = 16.0 / 9.0;
// const WIDTH: u32 = 600;
const WIDTH: u32 = 900;
const HEIGHT: u32 = (WIDTH as f32 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: usize = 100;
const MAX_DEPTH: usize = 50;

fn hit_sphere(sphere_center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin().clone() - sphere_center.clone();
    let a = ray.direction().length_squared();
    let half_b = oc.dot(*ray.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}

fn ray_color(ray: &Ray, world: &impl Hittable, depth: usize) -> Color {
    let mut hit_record = HitRecord::default();

    // if we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(ray, 0.001, f64::INFINITY, &mut hit_record) {
        // let target = hit_record.point + hit_record.normal + random_unit_vertor();
        let target = hit_record.point + random_in_hemisphere(&hit_record.normal);
        return 0.5
            * ray_color(
                &Ray::new(hit_record.point, target - hit_record.point),
                world,
                depth - 1,
            );
    }

    let unit_direction = ray.direction().normalize();
    let t = 0.5 * (unit_direction.y + 1.0);

    // lerp
    // blended_value = (1 - t) * start_value + t * end_value
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // World
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::new();

    // Render
    println!("P3");
    println!("{} {}", WIDTH, HEIGHT);
    println!("n255");

    let total_count = HEIGHT * WIDTH as u32;
    let progress_bar = ProgressBar::new(total_count as u64);

    for j in (0..HEIGHT).rev() {
        // progress_bar.set_position((HEIGHT - j) as u64);

        for i in 0..WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            progress_bar.inc(1);

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_f64()) / (WIDTH as f64 - 1.0);
                let v = (j as f64 + random_f64()) / (HEIGHT as f64 - 1.0);

                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, MAX_DEPTH);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
}
