use glam::DVec3;
use rand::Rng;
use ray_tracer::{
    bvh::Bvh,
    color::Color,
    hittable::{HittableList, MovingSphere, Sphere},
    material::Material,
    scene::Scene,
    texture::{CheckerTexture, SolidColor},
    Point3,
};
use std::sync::Arc;

// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const WIDTH: u32 = 1600;
// const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: usize = 100;
const MAX_DEPTH: usize = 50;

fn random_scene(scene: &mut Scene) {
    scene.set_aspect_ratio(16.0 / 9.0);
    scene.set_image_width(1000);
    scene.samples_per_pixel = 500;

    scene.camera.look_from = Point3::new(13.0, 2.0, 3.0);
    scene.camera.look_at = Point3::new(0.0, 0.0, 0.0);
    scene.camera.vup = DVec3::new(0.0, 1.0, 0.0);
    scene.camera.vfov = 20.0;
    scene.camera.aperture = 0.1;
    scene.camera.focus_dist = 10.0;

    let world = &mut scene.world;

    let checker = Arc::new(CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let material_ground = Arc::new(Material::Lambertian {
        albedo: checker.clone(),
    });
    // let material_ground = Arc::new(Material::Lambertian {
    //     albedo: Color::new(0.5, 0.5, 0.5),
    // });
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground.clone(),
    )));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rng.gen::<f64>();
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_material < 0.8 {
                    // diffuse
                    let albedo = Color::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
                        * Color::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>());
                    let sphere_material = Arc::new(Material::Lambertian {
                        albedo: Arc::new(SolidColor::new(albedo)),
                    });
                    let center_2 = center + DVec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                    world.add(Box::new(MovingSphere::new(
                        center,
                        center_2,
                        0.2,
                        sphere_material.clone(),
                    )));
                } else if choose_material < 0.95 {
                    // metal
                    let albedo = Color::new(
                        rng.gen_range(0.5..1.0),
                        rng.gen_range(0.5..1.0),
                        rng.gen_range(0.5..1.0),
                    );
                    let fuzz: f64 = rng.gen_range(0.0..0.5);
                    let sphere_material = Arc::new(Material::Metal { albedo, fuzz });
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material.clone())));
                } else {
                    // glass
                    let sphere_material = Arc::new(Material::Dielectric {
                        index_of_refraction: 1.5,
                    });
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material.clone())));
                }
            }
        }
    }

    let material_1 = Arc::new(Material::Dielectric {
        index_of_refraction: 1.5,
    });
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_1.clone(),
    )));
    let material_2 = Arc::new(Material::Lambertian {
        albedo: Arc::new(SolidColor::new(Color::new(0.4, 0.2, 0.1))),
    });
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2.clone(),
    )));
    let material_3 = Arc::new(Material::Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material_3.clone(),
    )));

    scene.world = HittableList::new(Box::new(Bvh::from_list(world)));
}

fn main() {
    let mut scene = Scene::new(ASPECT_RATIO, WIDTH, SAMPLES_PER_PIXEL, MAX_DEPTH);
    random_scene(&mut scene);
    scene.render();
}
