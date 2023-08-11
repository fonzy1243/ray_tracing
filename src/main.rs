use std::cell::RefCell;
use std::io::Write;
use std::ops::{Div, Sub};
use std::rc::Rc;
use ray_tracing::INFINITY;
use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::material::{Lambertian, Metal};
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
mod material;

fn main() {
    // Image
    let aspect_ratio = 16. / 9.;
    let image_width = 1080;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HittableList::default();

    let material_ground = Lambertian::new(Color::new(0.8,0.8,0.0));
    let material_center = Lambertian::new(Color::new(0.7,0.3,0.3));
    let material_left = Metal::new(Color::new(0.8,0.8,0.8));
    let material_right = Metal::new(Color::new(0.8,0.6,0.2));

    world.add(Box::new(Sphere::new(Point3::new(0.,-100.5,-1.), 100., Rc::new(RefCell::new(material_ground)))));
    world.add(Box::new(Sphere::new(Point3::new(0.,0.,-1.), 0.5, Rc::new(RefCell::new(material_center)))));
    world.add(Box::new(Sphere::new(Point3::new(-1.,0.,-1.), 0.5, Rc::new(RefCell::new(material_left)))));
    world.add(Box::new(Sphere::new(Point3::new(1.,0.,-1.), 0.5, Rc::new(RefCell::new(material_right)))));

    let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth);
    camera.render(&world);
}