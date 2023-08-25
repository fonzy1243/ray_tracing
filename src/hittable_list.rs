use crate::aabb::*;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

#[derive(Clone, Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
    bbox: Aabb,
}

impl HittableList {
    pub fn new(object: Box<dyn Hittable>) -> Self {
        let mut list = HittableList::default();
        list.add(object);

        list
    }
    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        let object_bbox = object.bounding_box();
        self.objects.push(object);
        self.bbox = Aabb::aabb(self.bbox, object_bbox);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> crate::aabb::Aabb {
        self.bbox
    }
}
