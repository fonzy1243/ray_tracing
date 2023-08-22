use crate::interval::*;
use crate::ray::*;
use crate::vec3::*;

#[derive(Default)]
struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
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

    pub fn axis(self, n: i32) -> Interval {
        if n == 1 {
            self.y
        } else if n == 2 {
            self.z
        } else {
            self.x
        }
    }

    pub fn hit(self, r: Ray, ray_t: Interval) -> bool {
        for a in 0..3 {}
    }
}
