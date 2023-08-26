use crate::aabb::*;
use crate::hittable::*;
use crate::hittable_list::*;
use crate::interval::*;
use crate::ray::*;
use ray_tracing::random_int;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct BvhNode {
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(list: HittableList) -> Box<dyn Hittable> {
        Self::new_split(list.objects.clone(), 0, list.objects.len())
    }

    pub fn new_split(
        mut src_objects: Vec<Box<dyn Hittable>>,
        start: usize,
        end: usize,
    ) -> Box<dyn Hittable> {
        let objects = &mut src_objects;

        let axis = random_int(0, 2);
        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };

        let object_span = end - start;
        let left: Box<dyn Hittable>;
        let right: Box<dyn Hittable>;

        if object_span == 1 {
            left = objects[start].clone();
            right = objects[start].clone();
        } else if object_span == 2 {
            if comparator(&*objects[start], &*objects[start + 1]) == Ordering::Less {
                left = objects[start].clone();
                right = objects[start + 1].clone();
            } else {
                left = objects[start + 1].clone();
                right = objects[start].clone();
            }
        } else {
            objects[start..end].sort_by(|a, b| comparator(&**a, &**b));

            let mid = start + object_span / 2;
            left = Self::new_split(objects.to_vec(), start, mid);
            right = Self::new_split(objects.to_vec(), mid, end);
        }

        let bbox = Aabb::aabb(left.bounding_box(), right.bounding_box());
        Box::new(Self { left, right, bbox })
    }

    fn box_compare(a: &dyn Hittable, b: &dyn Hittable, axis_index: i32) -> Ordering {
        let a_min = a.bounding_box().axis(axis_index).min;
        let b_min = b.bounding_box().axis(axis_index).min;

        if a_min < b_min {
            Ordering::Less
        } else if a_min > b_min {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    fn box_x_compare(a: &dyn Hittable, b: &dyn Hittable) -> Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &dyn Hittable, b: &dyn Hittable) -> Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &dyn Hittable, b: &dyn Hittable) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: Ray, mut ray_t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, &mut ray_t) {
            return false;
        }

        let hit_left: bool = self.left.hit(r, ray_t, rec);
        let hit_right: bool = self.right.hit(
            r,
            Interval::new(ray_t.min, if hit_left { rec.t } else { ray_t.max }),
            rec,
        );

        hit_left || hit_right
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
