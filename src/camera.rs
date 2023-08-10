use std::io;
use ray_tracing::{INFINITY, random_double};
use crate::color::{Color, write_color};
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy, PartialEq)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,

    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32, samples_per_pixel: i32, max_depth: i32) -> Self {
        Camera {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            image_height: 0,
            center: Point3::new(0.,0.,0.),
            pixel00_loc: Point3::new(0.,0.,0.),
            pixel_delta_u: Vec3::new(0.,0.,0.),
            pixel_delta_v: Vec3::new(0.,0.,0.),
        }
    }

    pub fn render(mut self, world: &dyn Hittable) {
        self.initialize();

        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        let mut stdout = io::stdout();

        for j in 0..self.image_height {
            let remaining = self.image_height - j;
            eprintln!("\rScanlines remaining: {remaining}");
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.,0.,0.);
                    for _sample in 0..self.samples_per_pixel {
                        let r = self.get_ray(i, j);
                        pixel_color = pixel_color + Camera::ray_color(r, self.max_depth, world);
                }
                write_color(&mut stdout, pixel_color, self.samples_per_pixel);
            }
        }

        eprintln!("\rDone.                  \n")
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;

        if self.image_height < 1 {
            self.image_height = 1;
        }

        // Camera
        let focal_length = 1.;

        let viewport_height = 2.;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.center = Point3::new(0., 0., 0.);

        let viewport_u = Vec3::new(viewport_width, 0., 0.);
        let viewport_v = Vec3::new(0., -viewport_height, 0.);

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left = self.center - Vec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color(r: Ray, depth: i32, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord::default();

        if depth <= 0 {
            return Color::new(0.,0.,0.)
        }

        if world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
            let direction = Vec3::random_on_hemisphere(rec.normal);
            return 0.5 * Camera::ray_color(Ray::new(rec.p, direction), depth - 1, world)
        }

        let unit_direction:Vec3 = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);

        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    fn get_ray(self, i: i32, j: i32) -> Ray {
        let pixel_center = self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn pixel_sample_square(self) -> Vec3 {
        let px = -0.5 + random_double();
        let py = -0.5 + random_double();
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }
}