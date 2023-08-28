use crate::aabb::*;
use crate::hittable::*;
use crate::interval::*;
use crate::material::*;
use crate::ray::*;
use crate::vec3::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    normal: Vec3,
    d: f64,
    mat: Arc<dyn Material + Send>,
    bbox: Aabb,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material + Send>) -> Self {
        let n = u.cross(v);
        let normal = n.unit_vector();
        let d = normal.dot(q);
        let w = n / n.dot(n);

        let mut quad = Self {
            q,
            u,
            v,
            w,
            normal,
            d,
            mat,
            bbox: Aabb::default(),
        };
        quad.set_bounding_box();

        quad
    }

    pub fn set_bounding_box(&mut self) {
        self.bbox = Aabb::new_from_points(self.q, self.q + self.u + self.v).pad();
    }

    fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool {
        if !(0. ..=1.).contains(&a) || !(0. ..=1.).contains(&b) {
            return false;
        }

        rec.u = a;
        rec.v = b;
        true
    }
}

impl Hittable for Quad {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let denom = self.normal.dot(r.direction());

        // if ray is parallel
        if denom.abs() < 1e-8 {
            return false;
        }

        // if the hit is outside the ray interval
        let t = (self.d - self.normal.dot(r.origin())) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        // determine that the hit lies within the planar shape
        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = self.w.dot(planar_hitpt_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hitpt_vector));

        if !Self::is_interior(alpha, beta, rec) {
            return false;
        }

        rec.t = t;
        rec.p = intersection;
        rec.mat = self.mat.clone();
        rec.set_face_normal(r, self.normal);

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
