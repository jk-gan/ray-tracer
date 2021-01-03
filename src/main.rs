use std::io::{stderr, Write};
mod ray;
mod vec3;

use vec3::{Color, Vec3};

fn main() {
    const IMAGE_WIDTH: isize = 256;
    const IMAGE_HEIGHT: isize = 256;

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {} {:?}", j, stderr().flush());
        for i in 0..IMAGE_WIDTH {
            let r = (i as f32) / ((IMAGE_WIDTH - 1) as f32);
            let g = (j as f32) / ((IMAGE_HEIGHT - 1) as f32);
            let b = 0.25;
            let color = Color::new(r, g, b);
            print!("{}", color);
        }
    }

    eprintln!("\nDone");
}
