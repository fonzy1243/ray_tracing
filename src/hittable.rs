use crate::aabb::*;
use crate::interval::Interval;
use crate::material::{Lambertian, Material};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub(crate) p: Point3,
    pub(crate) normal: Vec3,
    pub(crate) mat: Arc<dyn Material + Send>,
    pub(crate) t: f64,
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
            front_face,
        }
    }
    /**
     * Sets the hit record normal vector
     * NOTE: outward_normal is assumed to be a unit vector.
     */
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0f64;

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
            mat: Arc::new(Lambertian::default()),
            t: 0.,
            front_face: false,
        }
    }
}

pub trait Hittable: Sync + HittableClone {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> AABB;
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
