use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub(crate) p: Point3,
    pub(crate) normal: Vec3,
    pub(crate) t: f64,
    pub(crate) front_face: bool,
}

impl HitRecord {
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

pub trait Hittable {
    fn hit(&self, r: Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
}