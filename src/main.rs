use indicatif::{ProgressBar, ProgressStyle};

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

fn main() {
    println!("P3");
    println!("{} {}", WIDTH, HEIGHT);
    println!("n255");

    let pb = ProgressBar::new(HEIGHT as u64);

    for j in (0..HEIGHT).rev() {
        pb.set_position((HEIGHT - j) as u64);

        for i in 0..WIDTH {
            let r = i as f64 / (WIDTH as f64 - 1.0);
            let g = j as f64 / (HEIGHT as f64 - 1.0);
            let b = 0.25;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
