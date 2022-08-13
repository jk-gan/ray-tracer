use std::sync::{atomic::AtomicPtr, Arc};

use glam::DVec3;
use indicatif::ProgressBar;
use ray_tracer::{
    camera::Camera,
    color::{write_color, Color},
    hittable::{HitRecord, Hittable, HittableList, Sphere},
    material::{Dielectric, Lambertian, Metal},
    random_f64, random_f64_range, random_in_hemisphere, random_in_unit_sphere, random_unit_vertor,
    ray::Ray,
    Point3, PI,
};

// Image
const ASPECT_RATIO: f64 = 3.0 / 2.0;
// const WIDTH: u32 = 600;
const WIDTH: u32 = 1200;
const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;
// const SAMPLES_PER_PIXEL: usize = 100;
const SAMPLES_PER_PIXEL: usize = 500;
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
    // if we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(hitted_record) = world.hit(ray, 0.001, f64::INFINITY) {
        let mut scattered_ray = Ray::default();
        let mut attenuation = Color::default();

        if hitted_record
            .material
            .scatter(ray, &hitted_record, &mut attenuation, &mut scattered_ray)
        {
            return attenuation * ray_color(&scattered_ray, world, depth - 1);
        }

        return Color::new(0.0, 0.0, 0.0);

        // let target = hitted_record.point + random_in_hemisphere(&hitted_record.normal);
        // return 0.5
        //     * ray_color(
        //         &Ray::new(hitted_record.point, target - hitted_record.point),
        //         world,
        //         depth - 1,
        //     );
    }

    let unit_direction = ray.direction().normalize();
    let t = 0.5 * (unit_direction.y + 1.0);

    // lerp
    // blended_value = (1 - t) * start_value + t * end_value
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn random_scene() -> HittableList {
    let mut world = HittableList::default();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground.clone(),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = random_f64();
            let center = Point3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_material < 0.8 {
                    // diffuse
                    let albedo = Color::new(random_f64(), random_f64(), random_f64())
                        * Color::new(random_f64(), random_f64(), random_f64());
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material.clone())));
                } else if choose_material < 0.95 {
                    // metal
                    let albedo = Color::new(
                        random_f64_range(0.5, 1.0),
                        random_f64_range(0.5, 1.0),
                        random_f64_range(0.5, 1.0),
                    );
                    let fuzz = random_f64_range(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material.clone())));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material.clone())));
                }
            }
        }
    }

    let material_1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_1.clone(),
    )));
    let material_2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2.clone(),
    )));
    let material_3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material_3.clone(),
    )));

    world
}

fn main() {
    // World
    let world = random_scene();

    // Camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = DVec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

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
