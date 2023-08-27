use crate::bvh::*;
use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::sphere::Sphere;
use crate::texture::*;
use crate::vec3::{Point3, Vec3};
use ray_tracing::{random_double, random_double_r, INFINITY, PI};
use std::sync::Arc;

// Based on the book "Ray Tracing in One Weekend": https://raytracing.github.io/books/RayTracingInOneWeekend.html
// and written in Rust

mod aabb;
mod bvh;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod perlin;
mod ray;
mod sphere;
mod texture;
mod vec3;

fn random_spheres() {
    // Image
    let aspect_ratio = 16. / 9.;
    let image_width = 1200;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HittableList::default();

    let checker =
        CheckerTexture::new_from_colors(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    let ground_material = Lambertian::new(checker);
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
                let mut center2: Option<Vec3> = None;
                let sphere_material: Arc<dyn Material + Send> = if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    center2 = Some(center + Vec3::new(0., random_double_r(0., 0.5), 0.));
                    Arc::new(Lambertian::new_from_color(albedo))
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.);
                    let fuzz = random_double_r(0f64, 0.5);
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    Arc::new(Dielectric::new(1.5))
                };

                if choose_mat < 0.8 {
                    world.add(Box::new(Sphere::new_moving(
                        center,
                        center2.unwrap_or_default(),
                        0.2,
                        sphere_material,
                    )))
                } else {
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)))
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Box::new(Sphere::new(
        Point3::new(0., 1., 0.),
        1.0,
        Arc::new(material1),
    )));

    let material2 = Lambertian::new_from_color(Color::new(0.4, 0.2, 0.1));
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

    world = HittableList::new(BvhNode::new(world));
    let mut camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth);

    camera.vfov = 20.;
    camera.lookfrom = Point3::new(13., 2., 3.);
    camera.lookat = Point3::new(0., 0., 0.);
    camera.vup = Vec3::new(0., 1., 0.);

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.;

    camera.render(&world);
}

fn test() {
    let mut world = HittableList::default();

    let material_ground = Lambertian::new_from_color(Color::new(0.8, 0.8, 0.));
    let material_center = Lambertian::new_from_color(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.);

    let sphere1_material: Arc<dyn Material + Send> = Arc::new(material_ground);
    let sphere2_material: Arc<dyn Material + Send> = Arc::new(material_center);

    world.add(Box::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        sphere1_material,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        sphere2_material,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        0.5,
        Arc::new(material_left),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        -0.4,
        Arc::new(material_left),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        Arc::new(material_right),
    )));

    let mut camera = Camera::new(16. / 9., 400, 100, 50);
    camera.vfov = 90.;
    camera.lookfrom = Point3::new(-2., 2., 1.);
    camera.lookat = Point3::new(0., 0., -1.);
    camera.vup = Vec3::new(0., 1., 0.);

    camera.render(&world);
}

fn two_spheres() {
    let mut world = HittableList::default();

    let checker =
        CheckerTexture::new_from_colors(0.8, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));

    let material = Lambertian::new(checker);

    world.add(Box::new(Sphere::new(
        Point3::new(0., -10., 0.),
        10.,
        Arc::new(material.clone()),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0., 10., 0.),
        10.,
        Arc::new(material.clone()),
    )));

    // world = HittableList::new(BvhNode::new(world));

    let mut camera = Camera::new(16. / 9., 600, 100, 50);

    camera.vfov = 20.;
    camera.lookfrom = Point3::new(13., 2., 3.);
    camera.lookat = Point3::new(0., 0., 0.);
    camera.vup = Vec3::new(0., 1., 0.);

    camera.defocus_angle = 0.;
    camera.focus_dist = 10.;

    camera.render(&world);
}

fn test2() {
    let material = Lambertian::new_from_color(Color::new(0., 155., 70.));
    let sphere = Box::new(Sphere::new(Point3::new(0., 0., 0.), 2., Arc::new(material)));

    let mut world = HittableList::default();
    world.add(sphere);

    let mut camera = Camera::new(16. / 9., 1080, 400, 50);

    camera.vfov = 20.;
    camera.lookfrom = Point3::new(0., 0., 12.);
    camera.lookat = Point3::new(0., 0., 0.);
    camera.vup = Vec3::new(0., 1., 0.);

    camera.defocus_angle = 0.;

    camera.render(&world);
}

fn earth() {
    let load_texture = ImageTexture::new("earthmap.jpg");
    match load_texture {
        Err(_err) => (),
        Ok(earth_texture) => {
            let earth_surface = Lambertian::new(earth_texture);
            let globe_mat: Arc<dyn Material + Send> = Arc::new(earth_surface);
            let globe = Sphere::new(Point3::new(0., 0., 0.), 2., globe_mat);

            let mut world = HittableList::default();
            world.add(Box::new(globe));

            let mut camera = Camera::new(16. / 9., 1080, 400, 50);

            camera.vfov = 20.;
            camera.lookfrom = Point3::new(0., 0., 12.);
            camera.lookat = Point3::new(0., 0., 0.);
            camera.vup = Vec3::new(0., 1., 0.);

            camera.defocus_angle = 0.;

            camera.render(&world);
        }
    }
}

fn two_perlin_spheres() {
    let mut world = HittableList::default();

    let pertext = NoiseTexture::new(4.);
    world.add(Box::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        Arc::new(Lambertian::new(pertext)),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0., 2., 0.),
        2.,
        Arc::new(Lambertian::new(pertext)),
    )));

    let mut camera = Camera::new(16. / 9., 400, 100, 50);

    camera.vfov = 20.;
    camera.lookfrom = Point3::new(13., 2., 3.);
    camera.lookat = Point3::new(0., 0., 0.);
    camera.vup = Vec3::new(0., 1., 0.);

    camera.defocus_angle = 0.;
    camera.render(&world)
}

fn main() {
    match 4 {
        1 => random_spheres(),
        2 => two_spheres(),
        3 => earth(),
        4 => two_perlin_spheres(),
        _ => test(),
    }
}
