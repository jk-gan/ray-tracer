use glam::DVec3;
use ray_tracer::{
    bvh::Bvh,
    color::Color,
    constant_medium::ConstantMedium,
    hittable::{create_box, HittableList, MovingSphere, Quad, RotationY, Sphere, Translate},
    material::Material,
    random_f64, random_f64_range,
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

fn final_scene(scene: &mut Scene) {
    scene.set_image_width(600);
    scene.set_aspect_ratio(1.0);
    scene.samples_per_pixel = 3000;
    scene.background_color = Color::new(0.0, 0.0, 0.0);

    scene.camera.aperture = 0.0;
    scene.camera.vfov = 40.0;
    scene.camera.look_from = Point3::new(478.0, 278.0, -600.0);
    scene.camera.look_at = Point3::new(278.0, 278.0, 0.0);

    let mut boxes_1 = HittableList::default();
    let ground = Arc::new(Material::Lambertian {
        albedo: Arc::new(SolidColor::new(Color::new(0.48, 0.84, 0.53))),
    });

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_f64_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes_1.add(Arc::new(create_box(
                &Point3::new(x0, y0, z0),
                &Point3::new(x1, y1, z1),
                ground.clone(),
            )))
        }
    }

    let world = &mut scene.world;

    world.add(Arc::new(Bvh::from_list(boxes_1)));

    let light = Arc::new(Material::DiffuseLight {
        emit: Arc::new(SolidColor::new(Color::new(7.0, 7.0, 7.0))),
    });
    world.add(Arc::new(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        DVec3::new(300.0, 0.0, 0.0),
        DVec3::new(0.0, 0.0, 265.0),
        light.clone(),
    )));

    let center_1 = Point3::new(400.0, 400.0, 200.0);
    let center_2 = center_1 + DVec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Arc::new(Material::Lambertian {
        albedo: Arc::new(SolidColor::new(Color::new(0.7, 0.3, 0.1))),
    });
    world.add(Arc::new(MovingSphere::new(
        center_1,
        center_2,
        50.0,
        moving_sphere_material.clone(),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Material::Dielectric {
            index_of_refraction: 1.5,
        }),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Material::Metal {
            albedo: Color::new(0.8, 0.8, 0.8),
            fuzz: 1.0,
        }),
    )));

    let boundary_1 = Arc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Material::Dielectric {
            index_of_refraction: 1.5,
        }),
    ));
    world.add(boundary_1.clone());
    world.add(Arc::new(ConstantMedium::from_color(
        boundary_1.clone(),
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    let boundary_2 = Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Material::Dielectric {
            index_of_refraction: 1.5,
        }),
    ));
    world.add(Arc::new(ConstantMedium::from_color(
        boundary_2.clone(),
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));

    let earth_texture = Arc::new(ImageTexture::new(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("assets/images/earthmap.jpg")
            .as_path(),
    ));
    let earth_material = Arc::new(Material::Lambertian {
        albedo: earth_texture.clone(),
    });
    world.add(Arc::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        earth_material.clone(),
    )));

    let perlin_texture = Arc::new(NoiseTexture::new(0.1));
    let perlin_material = Arc::new(Material::Lambertian {
        albedo: perlin_texture.clone(),
    });
    world.add(Arc::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        perlin_material.clone(),
    )));

    let mut boxes_2 = HittableList::default();
    let white = Arc::new(Material::Lambertian {
        albedo: Arc::new(SolidColor::new(Color::new(0.73, 0.73, 0.73))),
    });
    let ns = 1000;
    for _ in 0..ns {
        boxes_2.add(Arc::new(Sphere::new(
            Point3::new(
                random_f64_range(0.0, 165.0),
                random_f64_range(0.0, 165.0),
                random_f64_range(0.0, 165.0),
            ),
            10.0,
            white.clone(),
        )));
    }

    world.add(Arc::new(Translate::new(
        Arc::new(RotationY::new(Arc::new(Bvh::from_list(boxes_2)), 15.0)),
        &DVec3::new(-100.0, 270.0, 395.0),
    )));
}

fn cornell_smoke(scene: &mut Scene) {
    scene.set_image_width(600);
    scene.set_aspect_ratio(1.0);
    scene.samples_per_pixel = 50;
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
        emit: Arc::new(SolidColor::new(Color::new(7.0, 7.0, 7.0))),
    });

    // Quads
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        DVec3::new(0.0, 555.0, 0.0),
        DVec3::new(0.0, 0.0, 555.0),
        green.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        DVec3::new(0.0, 555.0, 0.0),
        DVec3::new(0.0, 0.0, 555.0),
        red.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(113.0, 554.0, 127.0),
        DVec3::new(330.0, 0.0, 0.0),
        DVec3::new(0.0, 0.0, 305.0),
        light.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 555.0, 0.0),
        DVec3::new(555.0, 0.0, 0.0),
        DVec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        DVec3::new(555.0, 0.0, 0.0),
        DVec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        DVec3::new(555.0, 0.0, 0.0),
        DVec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    // Boxes
    let box_1 = Arc::new(create_box(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let rotated_box_1 = Arc::new(RotationY::new(box_1.clone(), 15.0));
    let translated_box_1 = Translate::new(rotated_box_1.clone(), &DVec3::new(265.0, 0.0, 295.0));
    // world.add(Arc::new(translated_box_1));

    let box_2 = Arc::new(create_box(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    let rotated_box_2 = Arc::new(RotationY::new(box_2.clone(), -18.0));
    let translated_box_2 = Translate::new(rotated_box_2.clone(), &DVec3::new(130.0, 0.0, 65.0));

    world.add(Arc::new(ConstantMedium::from_color(
        Arc::new(translated_box_1),
        0.01,
        Color::new(0.0, 0.0, 0.0),
    )));
    world.add(Arc::new(ConstantMedium::from_color(
        Arc::new(translated_box_2),
        0.01,
        Color::new(1.0, 1.0, 1.0),
    )));
}

fn cornell_box(scene: &mut Scene) {
    scene.set_image_width(600);
    scene.set_aspect_ratio(1.0);
    scene.samples_per_pixel = 10000;
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
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        DVec3::new(0.0, 555.0, 0.0),
        DVec3::new(0.0, 0.0, 555.0),
        green.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        DVec3::new(0.0, 555.0, 0.0),
        DVec3::new(0.0, 0.0, 555.0),
        red.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        DVec3::new(-130.0, 0.0, 0.0),
        DVec3::new(0.0, 0.0, -105.0),
        light.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        DVec3::new(555.0, 0.0, 0.0),
        DVec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        DVec3::new(-555.0, 0.0, 0.0),
        DVec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        DVec3::new(555.0, 0.0, 0.0),
        DVec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    // Boxes
    let box_1 = Arc::new(create_box(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let rotated_box_1 = Arc::new(RotationY::new(box_1.clone(), 15.0));
    let translated_box_1 = Translate::new(rotated_box_1.clone(), &DVec3::new(265.0, 0.0, 295.0));
    world.add(Arc::new(translated_box_1));

    // let box_2 = Arc::new(create_box(
    //     &Point3::new(0.0, 0.0, 0.0),
    //     &Point3::new(165.0, 165.0, 165.0),
    //     white.clone(),
    // ));
    // let rotated_box_2 = Arc::new(RotationY::new(box_2.clone(), -18.0));
    // let translated_box_2 = Translate::new(rotated_box_2.clone(), &DVec3::new(130.0, 0.0, 65.0));
    // world.add(Arc::new(translated_box_2));
    world.add(Arc::new(Sphere::new(
        Point3::new(160.0, 100.0, 145.0),
        100.0,
        Arc::new(Material::Dielectric {
            index_of_refraction: 1.5,
        }),
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
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        perlin_material.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        perlin_material.clone(),
    )));

    let diffuse_light = Arc::new(Material::DiffuseLight {
        emit: Arc::new(SolidColor::new(Color::new(4.0, 4.0, 4.0))),
    });
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        diffuse_light.clone(),
    )));
    world.add(Arc::new(Quad::new(
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
    world.add(Arc::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        DVec3::new(0.0, 0.0, -4.0),
        DVec3::new(0.0, 4.0, 0.0),
        left_red.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        DVec3::new(4.0, 0.0, 0.0),
        DVec3::new(0.0, 4.0, 0.0),
        back_green.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        DVec3::new(0.0, 0.0, 4.0),
        DVec3::new(0.0, 4.0, 0.0),
        right_blue.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        DVec3::new(4.0, 0.0, 0.0),
        DVec3::new(0.0, 0.0, 4.0),
        upper_orange.clone(),
    )));
    world.add(Arc::new(Quad::new(
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
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        perlin_material.clone(),
    )));
    world.add(Arc::new(Sphere::new(
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
    let globe = Arc::new(Sphere::new(
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
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        material_checker.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        material_checker.clone(),
    )));
}

fn random_scene(scene: &mut Scene) {
    // scene.set_aspect_ratio(16.0 / 9.0);
    scene.set_aspect_ratio(1.0);
    scene.set_image_width(300);
    scene.samples_per_pixel = 100;

    scene.camera.look_from = Point3::new(13.0, 2.0, 3.0);
    scene.camera.look_at = Point3::new(0.0, 0.0, 0.0);
    scene.camera.vup = DVec3::new(0.0, 1.0, 0.0);
    scene.camera.vfov = 20.0;
    scene.camera.aperture = 0.1;
    scene.camera.focus_dist = 10.0;

    // let world = &mut scene.world;
    let mut world = HittableList::default();

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
    world.add(Arc::new(Sphere::new(
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
                    let sphere_material = Arc::new(Material::Lambertian {
                        albedo: Arc::new(SolidColor::new(albedo)),
                    });
                    let center_2 = center + DVec3::new(0.0, random_f64_range(0.0, 0.5), 0.0);
                    // world.add(Arc::new(MovingSphere::new(
                    //     center,
                    //     center_2,
                    //     0.2,
                    //     sphere_material.clone(),
                    // )));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material.clone())));
                } else if choose_material < 0.95 {
                    // metal
                    let albedo = Color::new(
                        random_f64_range(0.5, 1.0),
                        random_f64_range(0.5, 1.0),
                        random_f64_range(0.5, 1.0),
                    );
                    let fuzz = random_f64_range(0.0, 0.5);
                    let sphere_material = Arc::new(Material::Metal { albedo, fuzz });
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material.clone())));
                } else {
                    // glass
                    let sphere_material = Arc::new(Material::Dielectric {
                        index_of_refraction: 1.5,
                    });
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material.clone())));
                }
            }
        }
    }

    let material_1 = Arc::new(Material::Dielectric {
        index_of_refraction: 1.5,
    });
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_1.clone(),
    )));
    let material_2 = Arc::new(Material::Lambertian {
        albedo: Arc::new(SolidColor::new(Color::new(0.4, 0.2, 0.1))),
    });
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2.clone(),
    )));
    let material_3 = Arc::new(Material::Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material_3.clone(),
    )));

    scene.world = HittableList::new(Arc::new(Bvh::from_list(world)));
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
    // cornell_box(&mut scene);
    // cornell_smoke(&mut scene);
    final_scene(&mut scene);
    // scene.set_image_width(400);
    // scene.samples_per_pixel = 100;
    // scene.max_depth = 4;

    scene.render();
}
