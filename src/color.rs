use std::io::Write;
use crate::interval::Interval;
use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(out: &mut dyn Write, pixel_color: Color, samples_per_pixel: i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide the color by the number of samples.
    let scale = 1.0 / samples_per_pixel as f64;
    r = scale * r;
    g = scale * g;
    b = scale * b;

    // Clamp the color components to the valid range [0.0, 0.999].
    let intensity = Interval::new(0.0, 0.999);
    r = intensity.clamp(r);
    g = intensity.clamp(g);
    b = intensity.clamp(b);

    // Write the translated [0,255] value of each color component.
    writeln!(out, "{} {} {}", (256.0 * r) as i32, (256.0 * g) as i32, (256.0 * b) as i32)
        .expect("TODO: panic message");
}