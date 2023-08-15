use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3};
use ray_tracing::{random_double, random_double_r, INFINITY, PI};
use std::io::Write;
use std::ops::{Div, Sub};
use std::rc::Rc;
use std::sync::Arc;
use std::sync::mpsc::Sender;

// Based on the book "Ray Tracing in One Weekend": https://raytracing.github.io/books/RayTracingInOneWeekend.html
// and written in Rust

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec3;

fn main() {
    let r = (PI / 4.).cos();

    // Image
    let aspect_ratio = 16. / 9.;
    let image_width = 1200;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // World
    let mut world = HittableList::default();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        Arc::new(ground_material),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                let sphere_material: Arc<dyn Material + Send> = if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    Arc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.);
                    let fuzz = random_double_r(0f64, 0.5);
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    Arc::new(Dielectric::new(1.5))
                };
                world.add(Box::new(Sphere::new(center, 0.2, sphere_material)))
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Box::new(Sphere::new(
        Point3::new(0., 1., 0.),
        1.0,
        Arc::new(material1),
    )));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Box::new(Sphere::new(
        Point3::new(-4., 1., 0.),
        1.0,
        Arc::new(material2),
    )));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Box::new(Sphere::new(
        Point3::new(4., 1., 0.),
        1.0,
        Arc::new(material3),
    )));

    let mut camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth);

    camera.vfov = 20.;
    camera.lookfrom = Point3::new(13., 2., 3.);
    camera.lookat = Point3::new(0., 0., 0.);
    camera.vup = Vec3::new(0., 1., 0.);

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.;

    camera.render(&world);
}
