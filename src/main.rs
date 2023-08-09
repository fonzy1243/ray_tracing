use std::io;
use std::io::{stdout, Write};
use std::ops::{Div, Sub};
use crate::color::{Color, write_color};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

// Based on the book "Ray Tracing in One Weekend": https://raytracing.github.io/books/RayTracingInOneWeekend.html
// and written in Rust

mod ray;
mod color;
mod vec3;
mod hittable;
mod sphere;

fn hit_sphere(center: Point3, radius: f64, r: Ray) -> f64 {
    let oc = r.origin() - center;

    let a = r.direction().length_squared();
    let half_b = oc.dot(r.direction());
    let c = oc.length_squared() - radius.powi(2);

    let discriminant = half_b.powi(2) - a * c;

    if discriminant < 0. {
        -1.0
    }
    else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(r: Ray) -> Color {
    let t = hit_sphere(Point3::new(0., 0., -1.), 0.5, r);

    if t > 0. {
        let N = (r.at(t) - Vec3::new(0., 0., -1.)).unit_vector();
        return  0.5 * Color::new(N.x() + 1., N.y() + 1., N.z() + 1.);
    }

    let unit_direction:Vec3 = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);

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
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

    let camera_center = Point3::new(0., 0., 0.);

    let viewport_u = Vec3::new(viewport_width, 0., 0.);
    let viewport_v = Vec3::new(0., -viewport_height, 0.);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left = camera_center - Vec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    print!("P3\n{image_width} {image_height}\n255\n");

    let mut stdout = io::stdout();

    for j in 0..image_height {
        let remaining = image_height - j;
        eprintln!("\rScanlines remaining: {remaining}");
        for i in 0..image_width {
            let pixel_center = pixel00_loc + ((i as f64).round() * pixel_delta_u) + ((j as f64).round() * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(r);
            write_color(&mut stdout, pixel_color);
        }
    }

    eprintln!("\rDone.                  \n")
}