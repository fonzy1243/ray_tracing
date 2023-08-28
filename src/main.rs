use crate::bvh::*;
use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::quad::*;
use crate::sphere::Sphere;
use crate::texture::*;
use crate::vec3::{Point3, Vec3};
use material::DiffuseLight;
use ray_tracing::{random_double, random_double_r, INFINITY, PI};
use std::sync::Arc;
use std::time::Instant;

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
mod quad;
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
    camera.background = Color::new(0.7, 0.8, 1.);

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
    camera.background = Color::new(0.7, 0.8, 1.);

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
    camera.background = Color::new(0.7, 0.8, 1.);

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
    camera.background = Color::new(0.7, 0.8, 1.);

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
            camera.background = Color::new(0.7, 0.8, 1.);

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
    camera.background = Color::new(0.7, 0.8, 1.);

    camera.defocus_angle = 0.;
    camera.render(&world)
}

fn quads() {
    let mut world = HittableList::default();

    let left_red = Arc::new(Lambertian::new_from_color(Color::new(1., 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::new_from_color(Color::new(0.2, 1., 0.2)));
    let right_blue = Arc::new(Lambertian::new_from_color(Color::new(0.2, 0.2, 1.)));
    let upper_orange = Arc::new(Lambertian::new_from_color(Color::new(1., 0.5, 0.)));
    let lower_teal = Arc::new(Lambertian::new_from_color(Color::new(0.2, 0.8, 0.8)));

    world.add(Box::new(Quad::new(
        Point3::new(-3., -2., 5.),
        Vec3::new(0., 0., -4.),
        Vec3::new(0., 4., 0.),
        left_red,
    )));
    world.add(Box::new(Quad::new(
        Point3::new(-2., -2., 0.),
        Vec3::new(4., 0., 0.),
        Vec3::new(0., 4., 0.),
        back_green,
    )));
    world.add(Box::new(Quad::new(
        Point3::new(3., -2., 1.),
        Vec3::new(0., 0., 4.),
        Vec3::new(0., 4., 0.),
        right_blue,
    )));
    world.add(Box::new(Quad::new(
        Point3::new(-2., 3., 1.),
        Vec3::new(4., 0., 0.),
        Vec3::new(0., 0., 4.),
        upper_orange,
    )));
    world.add(Box::new(Quad::new(
        Point3::new(-2., -3., 5.),
        Vec3::new(4., 0., 0.),
        Vec3::new(0., 0., -4.),
        lower_teal,
    )));

    let mut cam = Camera::new(1., 800, 500, 50);

    cam.vfov = 80.;
    cam.lookfrom = Point3::new(0., 0., 9.);
    cam.lookat = Point3::new(0., 0., 0.);
    cam.vup = Vec3::new(0., 1., 0.);
    cam.background = Color::new(0.7, 0.8, 1.);

    cam.defocus_angle = 0.;

    cam.render(&world);
}

fn simple_light() {
    let mut world = HittableList::default();

    let pertext = Arc::new(Lambertian::new(NoiseTexture::new(4.)));
    world.add(Box::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        pertext.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0., 2., 0.),
        2.,
        pertext.clone(),
    )));

    let difflight = Arc::new(DiffuseLight::new_with_color(Color::new(4., 4., 4.)));
    world.add(Box::new(Sphere::new(
        Point3::new(0., 7., 0.),
        2.,
        difflight.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(3., 1., -2.),
        Vec3::new(2., 0., 0.),
        Vec3::new(0., 2., 0.),
        difflight.clone(),
    )));

    world = HittableList::new(BvhNode::new(world));

    let mut cam = Camera::new(16. / 9., 1600, 1500, 50);

    cam.background = Color::new(0., 0., 0.);
    cam.vfov = 20.;
    cam.lookfrom = Point3::new(26., 3., 6.);
    cam.lookat = Point3::new(0., 2., 0.);
    cam.vup = Vec3::new(0., 1., 0.);

    cam.defocus_angle = 0.;

    cam.render(&world);
}

fn cornell_box() {
    let mut world = HittableList::default();

    let red = Arc::new(Lambertian::new_from_color(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_from_color(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_from_color(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_with_color(Color::new(15., 15., 15.)));

    world.add(Box::new(Quad::new(
        Point3::new(555., 0., 0.),
        Vec3::new(0., 555., 0.),
        Vec3::new(0., 0., 555.),
        green,
    )));
    world.add(Box::new(Quad::new(
        Point3::new(0., 0., 0.),
        Vec3::new(0., 555., 0.),
        Vec3::new(0., 0., 555.),
        red,
    )));
    world.add(Box::new(Quad::new(
        Point3::new(343., 554., 332.),
        Vec3::new(-130., 0., 0.),
        Vec3::new(0., 0., -105.),
        light,
    )));
    world.add(Box::new(Quad::new(
        Point3::new(0., 0., 0.),
        Vec3::new(555., 0., 0.),
        Vec3::new(0., 0., 555.),
        white.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(555., 555., 555.),
        Vec3::new(-555., 0., 0.),
        Vec3::new(0., 0., -555.),
        white.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(0., 0., 555.),
        Vec3::new(555., 0., 0.),
        Vec3::new(0., 555., 0.),
        white.clone(),
    )));

    world.add(r#box(
        Point3::new(130., 0., 65.),
        Point3::new(295., 165., 230.),
        white.clone(),
    ));
    world.add(r#box(
        Point3::new(265., 0., 295.),
        Point3::new(430., 330., 460.),
        white.clone(),
    ));

    world = HittableList::new(BvhNode::new(world));

    let mut cam = Camera::new(1., 600, 20000, 50);
    cam.background = Color::default();

    cam.vfov = 40.;
    cam.lookfrom = Point3::new(278., 278., -800.);
    cam.lookat = Point3::new(278., 278., 0.);
    cam.vup = Vec3::new(0., 1., 0.);

    cam.defocus_angle = 0.;

    cam.render(&world);
}

fn main() {
    let before = Instant::now();
    match 7 {
        1 => random_spheres(),
        2 => two_spheres(),
        3 => earth(),
        4 => two_perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        _ => test(),
    }
    eprintln!("Elapsed time: {:.2?}", before.elapsed());
}
