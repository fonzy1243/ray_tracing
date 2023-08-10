use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy, PartialEq)]
pub struct HitRecord {
    pub(crate) p: Point3,
    pub(crate) normal: Vec3,
    pub(crate) t: f64,
    pub(crate) front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64, front_face: bool) -> Self {
        HitRecord {
            p,
            normal,
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
        }
        else {
            self.normal = -outward_normal
        }
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Point3::new(0., 0., 0.,),
            normal: Vec3::new(0., 0., 0.),
            t: 0.,
            front_face: false,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
}