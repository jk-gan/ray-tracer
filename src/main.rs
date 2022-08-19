use glam::DVec3;
use rand::Rng;
use ray_tracer::{
    bvh::Bvh,
    color::Color,
    hittable::{HittableList, MovingSphere, Quad, Sphere},
    material::Material,
    scene::Scene,
    texture::{CheckerTexture, ImageTexture, NoiseTexture, SolidColor},
    Point3,
};
use std::sync::Arc;

// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const WIDTH: u32 = 1600;
// const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: usize = 100;
const MAX_DEPTH: usize = 50;

fn cornell_box(scene: &mut Scene) {
    scene.set_image_width(600);
    scene.set_aspect_ratio(1.0);
    scene.samples_per_pixel = 500;
    scene.background_color = Color::new(0.0, 0.0, 0.0);

    scene.camera.aperture = 0.0;
    scene.camera.vfov = 40.0;
    scene.camera.look_from = Point3::new(278.0, 278.0, -800.0);
    scene.camera.look_at = Point3::new(278.0, 278.0, 0.0);

    let world = &mut scene.world;

    // Materials
    let red = Arc::new(Material::Lambertian {
        albedo: Arc::new(SolidColor::new(Color::new(0.65, 0.05, 0.05))),
    });
    let white = Arc::new(Material::Lambertian {
        albedo: Arc::new(SolidColor::new(Color::new(0.73, 0.73, 0.73))),
    });
    let green = Arc::new(Material::Lambertian {
        albedo: Arc::new(SolidColor::new(Color::new(0.12, 0.45, 0.15))),
    });
    let light = Arc::new(Material::DiffuseLight {
        emit: Arc::new(SolidColor::new(Color::new(15.0, 15.0, 15.0))),
    });

    // Quads
    world.add(Box::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        DVec3::new(0.0, 555.0, 0.0),
        DVec3::new(0.0, 4.0, 555.0),
        green.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        DVec3::new(0.0, 555.0, 0.0),
        DVec3::new(0.0, 0.0, 555.0),
        red.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        DVec3::new(-130.0, 0.0, 0.0),
        DVec3::new(0.0, 0.0, -105.0),
        light.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        DVec3::new(555.0, 0.0, 0.0),
        DVec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        DVec3::new(-555.0, 0.0, 0.0),
        DVec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        DVec3::new(555.0, 0.0, 0.0),
        DVec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));
}

fn simple_light(scene: &mut Scene) {
    scene.set_image_width(400);
    scene.set_aspect_ratio(16.0 / 9.0);
    scene.samples_per_pixel = 100;
    scene.background_color = Color::new(0.0, 0.0, 0.0);

    scene.camera.aperture = 0.0;
    scene.camera.vfov = 20.0;
    scene.camera.look_from = Point3::new(26.0, 3.0, 6.0);
    scene.camera.look_at = Point3::new(0.0, 2.0, 0.0);

    let world = &mut scene.world;

    let perlin_texture = Arc::new(NoiseTexture::new(4.0));
    let perlin_material = Arc::new(Material::Lambertian {
        albedo: perlin_texture.clone(),
    });
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        perlin_material.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        perlin_material.clone(),
    )));

    let diffuse_light = Arc::new(Material::DiffuseLight {
        emit: Arc::new(SolidColor::new(Color::new(4.0, 4.0, 4.0))),
    });
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        diffuse_light.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        DVec3::new(2.0, 0.0, 0.0),
        DVec3::new(0.0, 2.0, 0.0),
        diffuse_light.clone(),
    )));
}

fn quads(scene: &mut Scene) {
    scene.set_image_width(400);
    scene.set_aspect_ratio(1.0);
    scene.samples_per_pixel = 100;

    scene.camera.aperture = 0.0;
    scene.camera.vfov = 80.0;
    scene.camera.look_from = Point3::new(0.0, 0.0, 9.0);
    scene.camera.look_at = Point3::new(0.0, 0.0, 0.0);

    let world = &mut scene.world;

    // Materials
    let left_red = Arc::new(Material::Lambertian {
        albedo: Arc::new(SolidColor::new(Color::new(1.0, 0.2, 0.2))),
    });
    let back_green = Arc::new(Material::Lambertian {
        albedo: Arc::new(SolidColor::new(Color::new(0.2, 1.0, 0.2))),
    });
    let right_blue = Arc::new(Material::Lambertian {
        albedo: Arc::new(SolidColor::new(Color::new(0.2, 0.2, 1.0))),
    });
    let upper_orange = Arc::new(Material::Lambertian {
        albedo: Arc::new(SolidColor::new(Color::new(1.0, 0.5, 0.0))),
    });
    let lower_teal = Arc::new(Material::Lambertian {
        albedo: Arc::new(SolidColor::new(Color::new(0.2, 0.8, 0.8))),
    });

    // Quads
    world.add(Box::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        DVec3::new(0.0, 0.0, -4.0),
        DVec3::new(0.0, 4.0, 0.0),
        left_red.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        DVec3::new(4.0, 0.0, 0.0),
        DVec3::new(0.0, 4.0, 0.0),
        back_green.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        DVec3::new(0.0, 0.0, 4.0),
        DVec3::new(0.0, 4.0, 0.0),
        right_blue.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        DVec3::new(4.0, 0.0, 0.0),
        DVec3::new(0.0, 0.0, 4.0),
        upper_orange.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        DVec3::new(4.0, 0.0, 0.0),
        DVec3::new(0.0, 0.0, -4.0),
        lower_teal.clone(),
    )));
}

fn two_perlin_spheres(scene: &mut Scene) {
    scene.set_image_width(800);
    scene.set_aspect_ratio(16.0 / 9.0);
    scene.samples_per_pixel = 300;

    scene.camera.aperture = 0.0;
    scene.camera.vfov = 20.0;
    scene.camera.look_from = Point3::new(13.0, 2.0, 3.0);
    scene.camera.look_at = Point3::new(0.0, 0.0, 0.0);

    let world = &mut scene.world;

    let perlin_texture = Arc::new(NoiseTexture::new(4.0));
    let perlin_material = Arc::new(Material::Lambertian {
        albedo: perlin_texture.clone(),
    });
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        perlin_material.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        perlin_material.clone(),
    )));
}

fn earth(scene: &mut Scene) {
    scene.set_image_width(800);
    scene.set_aspect_ratio(16.0 / 9.0);
    scene.samples_per_pixel = 300;

    scene.camera.aperture = 0.0;
    scene.camera.vfov = 20.0;
    scene.camera.look_from = Point3::new(0.0, 0.0, 12.0);
    scene.camera.look_at = Point3::new(0.0, 0.0, 0.0);

    let world = &mut scene.world;

    let earth_texture = Arc::new(ImageTexture::new(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("assets/images/earthmap.jpg")
            .as_path(),
    ));
    let earth_surface = Arc::new(Material::Lambertian {
        albedo: earth_texture.clone(),
    });
    let globe = Box::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        earth_surface.clone(),
    ));
    world.add(globe);
}

fn two_spheres(scene: &mut Scene) {
    scene.set_image_width(400);
    scene.set_aspect_ratio(16.0 / 9.0);
    scene.samples_per_pixel = 100;

    scene.camera.aperture = 0.0;
    scene.camera.vfov = 20.0;
    scene.camera.look_from = Point3::new(13.0, 2.0, 3.0);
    scene.camera.look_at = Point3::new(0.0, 0.0, 0.0);

    let world = &mut scene.world;

    let checker = Arc::new(CheckerTexture::from_colors(
        0.8,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let material_checker = Arc::new(Material::Lambertian {
        albedo: checker.clone(),
    });
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        material_checker.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        material_checker.clone(),
    )));
}

fn random_scene(scene: &mut Scene) {
    scene.set_aspect_ratio(16.0 / 9.0);
    scene.set_image_width(400);
    scene.samples_per_pixel = 100;

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

    scene.background_color = Color::new(0.7, 0.8, 1.0);
    scene.camera.vup = DVec3::new(0.0, 1.0, 0.0);
    scene.camera.focus_dist = 10.0;

    // random_scene(&mut scene);
    // two_spheres(&mut scene);
    // earth(&mut scene);
    // two_perlin_spheres(&mut scene);
    // quads(&mut scene);
    // simple_light(&mut scene);
    cornell_box(&mut scene);
    scene.render();
}
