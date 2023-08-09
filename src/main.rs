use std::io;
use std::ops::{Div, Sub};
use crate::color::{Color, write_color};
use crate::ray::Ray;
use crate::vec3::Vec3;

// Based on the book "Ray Tracing in One Weekend": https://raytracing.github.io/books/RayTracingInOneWeekend.html
// and written in Rust

mod ray;
mod color;
mod vec3;

fn ray_color(r: Ray) -> Color {
    let unit_direction:Vec3 = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);

    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let aspect_ratio = 16. / 9.;

    let image_width = 400;
    let mut image_height = (image_width as f64 / aspect_ratio) as i32;

    if image_height < 1 {
        image_height = 1;
    }

    let focal_length = 1.;
    let viewport_height = 2.;


    print!("P3\n{image_width} {image_height}\n255\n");

    let mut stdout = io::stdout();

    for j in 0..image_height {
        let remaining = image_height - j;
        eprintln!("\rScanlines remaining: {remaining}");
        for i in 0..image_width {
            let pixel_color = Color::new(i as f64 / (image_width - 1) as f64, j as f64 / (image_height - 1) as f64, 0.0);
            write_color(*stdout, pixel_color);
        }
    }

    eprintln!("\rDone.                  \n")
}