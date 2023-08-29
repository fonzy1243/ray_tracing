use ray_tracing::{degrees_to_radians, INFINITY};

use crate::aabb::*;
use crate::interval::Interval;
use crate::material::{Dielectric, Material};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::f64::NEG_INFINITY;
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub(crate) p: Point3,
    pub(crate) normal: Vec3,
    pub(crate) mat: Arc<dyn Material + Send>,
    pub(crate) t: f64,
    pub(crate) u: f64,
    pub(crate) v: f64,
    pub(crate) front_face: bool,
}

impl HitRecord {
    pub fn new(
        p: Point3,
        normal: Vec3,
        mat: Arc<dyn Material + Send>,
        t: f64,
        front_face: bool,
    ) -> Self {
        HitRecord {
            p,
            normal,
            mat,
            t,
            u: 0.,
            v: 0.,
            front_face,
        }
    }
    /**
     * Sets the hit record normal vector
     * NOTE: outward_normal is assumed to be a unit vector.
     */
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.;

        if self.front_face {
            self.normal = outward_normal
        } else {
            self.normal = -outward_normal
        }
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Point3::new(0., 0., 0.),
            normal: Vec3::new(0., 0., 0.),
            mat: Arc::new(Dielectric::default()),
            t: 0.,
            u: 0.,
            v: 0.,
            front_face: false,
        }
    }
}

pub trait Hittable: Sync + HittableClone {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> Aabb;
}

pub trait HittableClone {
    fn clone_box(&self) -> Box<dyn Hittable>;
}

impl<T> HittableClone for T
where
    T: 'static + Hittable + Clone,
{
    fn clone_box(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Hittable> {
    fn clone(&self) -> Box<dyn Hittable> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct Translate {
    object: Box<dyn Hittable>,
    offset: Vec3,
    bbox: Aabb,
}

impl Translate {
    pub fn new(p: Box<dyn Hittable>, displacement: Vec3) -> Self {
        let object = p.clone();
        Self {
            object,
            offset: displacement,
            bbox: p.bounding_box() + displacement,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let offset_r = Ray::new_with_time(r.origin() - self.offset, r.direction(), r.time());

        if !self.object.hit(offset_r, ray_t, rec) {
            return false;
        }

        rec.p += self.offset;

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

#[derive(Clone)]
pub struct RotateY {
    object: Box<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl RotateY {
    pub fn new(p: Box<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox = p.bounding_box();

        let object = p.clone();

        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1 - i) as f64 * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1 - j) as f64 * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1 - k) as f64 * bbox.z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        bbox = Aabb::new_from_points(min, max);

        Self {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];

        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];

        let rotated_r = Ray::new_with_time(origin, direction, r.time());

        if !self.object.hit(rotated_r, ray_t, rec) {
            return false;
        }

        let mut p = rec.p;
        p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
        p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

        let mut normal = rec.normal;
        normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
        normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

        rec.p = p;
        rec.normal = normal;

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
