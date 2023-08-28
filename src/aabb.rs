use crate::hittable::Hittable;
use crate::interval::*;
use crate::ray::*;
use crate::vec3::*;

#[derive(Copy, Clone, Default)]
pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Aabb {
    pub fn new(ix: Interval, iy: Interval, iz: Interval) -> Self {
        Self {
            x: ix,
            y: iy,
            z: iz,
        }
    }

    pub fn new_from_points(a: Point3, b: Point3) -> Self {
        Self {
            x: Interval::new(a[0].min(b[0]), a[0].max(b[0])),
            y: Interval::new(a[1].min(b[1]), a[1].max(b[1])),
            z: Interval::new(a[2].min(b[2]), a[2].max(b[2])),
        }
    }

    pub fn pad(&self) -> Self {
        let delta = 0.0001;
        let new_x = if self.x.size() >= delta {
            self.x
        } else {
            self.x.expand(delta)
        };

        let new_y = if self.y.size() >= delta {
            self.y
        } else {
            self.y.expand(delta)
        };

        let new_z = if self.z.size() >= delta {
            self.z
        } else {
            self.z.expand(delta)
        };

        Self::new(new_x, new_y, new_z)
    }

    pub fn axis(self, n: i32) -> Interval {
        if n == 1 {
            self.y
        } else if n == 2 {
            self.z
        } else {
            self.x
        }
    }

    pub fn hit(self, r: Ray, ray_t: &mut Interval) -> bool {
        for a in 0..3 {
            let inv_d = 1. / r.direction()[a];
            let orig = r.origin()[a];

            let mut t0 = (self.axis(i32::try_from(a).unwrap()).min - orig) * inv_d;
            let mut t1 = (self.axis(i32::try_from(a).unwrap()).max - orig) * inv_d;

            if inv_d < 0. {
                std::mem::swap(&mut t0, &mut t1)
            }

            if t0 > ray_t.min {
                ray_t.min = t0
            }
            if t1 < ray_t.max {
                ray_t.max = t1
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }

        true
    }

    pub fn aabb(box0: Aabb, box1: Aabb) -> Aabb {
        Self {
            x: Interval::new_from_intervals(box0.x, box1.x),
            y: Interval::new_from_intervals(box0.y, box1.y),
            z: Interval::new_from_intervals(box0.z, box1.z),
        }
    }
}
