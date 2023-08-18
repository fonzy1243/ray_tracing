use crate::vec3::*;

#[derive(Clone, Copy, PartialEq)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
    tm: f64,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir, tm: 0. }
    }

    pub fn new_with_time(orig: Point3, dir: Vec3, time: f64) -> Self { Self { orig, dir, tm: time } }

    pub fn at(self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }

    pub fn origin(self) -> Point3 {
        self.orig
    }

    pub fn direction(self) -> Vec3 {
        self.dir
    }

    pub fn time(self) -> f64 { self.tm }
}
