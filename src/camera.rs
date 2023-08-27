use crate::color::{write_color, Color};
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use ray_tracing::{degrees_to_radians, random_double, INFINITY};
use rayon::prelude::*;
use std::io;

#[derive(Clone, Copy, PartialEq)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vfov: f64,

    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,

    pub defocus_angle: f64,
    pub focus_dist: f64,

    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: i32,
        samples_per_pixel: i32,
        max_depth: i32,
    ) -> Self {
        Camera {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,

            vfov: 0.,
            lookfrom: Point3::new(0., 0., 0.),
            lookat: Point3::new(0., 0., 0.),
            vup: Vec3::new(0., 0., 0.),

            defocus_angle: 0.,
            focus_dist: 10.,

            image_height: 0,
            center: Point3::new(0., 0., 0.),
            pixel00_loc: Point3::new(0., 0., 0.),
            pixel_delta_u: Vec3::new(0., 0., 0.),
            pixel_delta_v: Vec3::new(0., 0., 0.),
            u: Vec3::new(0., 0., 0.),
            v: Vec3::new(0., 0., 0.),
            w: Vec3::new(0., 0., 0.),
            defocus_disk_u: Vec3::new(0., 0., 0.),
            defocus_disk_v: Vec3::new(0., 0., 0.),
        }
    }

    pub fn render(mut self, world: &dyn Hittable) {
        self.initialize();

        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        let mut stdout = io::stdout().lock();

        let pixel_colors: Vec<Vec<Color>> = (0..self.image_height)
            .into_par_iter()
            .map(|j| {
                let remaining = self.image_height - j;
                eprintln!("\rScanlines remaining: {remaining}");
                let row_colors: Vec<Color> = (0..self.image_width)
                    .into_par_iter()
                    .map(|i| {
                        let mut pixel_color: Color = Color::new(0., 0., 0.);
                        for _sample in 0..self.samples_per_pixel {
                            let r = self.get_ray(i, j);
                            pixel_color = pixel_color + Camera::ray_color(r, self.max_depth, world);
                        }
                        pixel_color
                    })
                    .collect();
                row_colors
            })
            .collect();

        for row in pixel_colors {
            for pixel_color in row {
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

        self.center = self.lookfrom;

        // Camera

        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w = (self.lookfrom - self.lookat).unit_vector();
        self.u = (self.vup.cross(self.w)).unit_vector();
        self.v = self.w.cross(self.u);

        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2. - viewport_v / 2.;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * degrees_to_radians(self.defocus_angle / 2.).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn ray_color(r: Ray, depth: i32, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord::default();

        if depth <= 0 {
            return Color::new(0., 0., 0.);
        }

        if world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
            let mut scattered = Ray::new(Point3::new(0., 0., 0.), Vec3::new(0., 0., 0.));
            let mut attenuation = Color::new(0., 0., 0.);

            if rec.mat.scatter(&r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * Self::ray_color(scattered, depth - 1, world);
            }

            return Color::new(0., 0., 0.);
        }

        let unit_direction: Vec3 = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);

        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    fn get_ray(self, i: i32, j: i32) -> Ray {
        let pixel_center =
            self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0. {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_double();

        Ray::new_with_time(ray_origin, ray_direction, ray_time)
    }

    fn defocus_disk_sample(self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }

    fn pixel_sample_square(self) -> Vec3 {
        let px = -0.5 + random_double();
        let py = -0.5 + random_double();
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }
}
