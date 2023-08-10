use std::io::Write;
use std::ops::{Div, Sub};
use ray_tracing::INFINITY;
use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::vec3::Point3;

// Based on the book "Ray Tracing in One Weekend": https://raytracing.github.io/books/RayTracingInOneWeekend.html
// and written in Rust

mod ray;
mod color;
mod vec3;
mod hittable;
mod sphere;
mod hittable_list;
mod interval;
mod camera;

fn main() {
    // Image
    let aspect_ratio = 16. / 9.;
    let image_width = 1080;
    let samples_per_pixel = 10;

    // World
    let mut world = HittableList::default();

    world.add(Box::new(Sphere::new(Point3::new(0.,0.,-1.), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.,-100.5,-1.), 100.)));

    let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel);
    camera.render(&world);
}