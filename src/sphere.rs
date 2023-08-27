use ray_tracing::PI;

use crate::aabb::*;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;

// TODO: Chapter 2.4 Add Moving Spheres

#[derive(Clone)]
pub struct Sphere {
    center1: Point3,
    radius: f64,
    mat: Arc<dyn Material + Send>,
    is_moving: bool,
    center_vec: Vec3,
    bbox: Aabb,
}

impl Sphere {
    pub(crate) fn new(center: Point3, radius: f64, mat: Arc<dyn Material + Send>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);

        Self {
            center1: center,
            radius,
            mat,
            is_moving: false,
            center_vec: Vec3::default(),
            bbox: Aabb::new_from_points(center - rvec, center + rvec),
        }
    }

    pub(crate) fn new_moving(
        center: Point3,
        center2: Point3,
        radius: f64,
        mat: Arc<dyn Material + Send>,
    ) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = Aabb::new_from_points(center - rvec, center + rvec);
        let box2 = Aabb::new_from_points(center2 - rvec, center2 + rvec);
        Self {
            center1: center,
            radius,
            mat,
            is_moving: true,
            center_vec: center2 - center,
            bbox: Aabb::aabb(box1, box2),
        }
    }

    fn center(&self, time: f64) -> Point3 {
        self.center1 + time * self.center_vec
    }

    pub(crate) fn get_sphere_uv(p: Point3, rec: &mut HitRecord) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        rec.u = phi / (2. * PI);
        rec.v = theta / PI;
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let center: Point3 = if self.is_moving {
            self.center(r.time())
        } else {
            self.center1
        };

        let oc = r.origin() - center;

        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0. {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;

            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);

        let outward_normal = (rec.p - center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        Self::get_sphere_uv(outward_normal, rec);
        rec.mat = self.mat.clone();

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
