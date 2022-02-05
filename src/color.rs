use glam::DVec3;

pub type Color = DVec3;

pub fn write_color(pixel_color: Color) {
    println!(
        "{} {} {}",
        (255.999 * pixel_color.x) as i64,
        (255.999 * pixel_color.y) as i64,
        (255.999 * pixel_color.z) as i64
    );
}
