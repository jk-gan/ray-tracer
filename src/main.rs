fn main() {
    const IMAGE_WIDTH: isize = 256;
    const IMAGE_HEIGHT: isize = 256;

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let r = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let g = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);
            let b = 0.25;

            let ir: isize = (255.999 * r) as isize;
            let ig: isize = (255.999 * g) as isize;
            let ib: isize = (255.999 * b) as isize;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
