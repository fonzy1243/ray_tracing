use crate::INFINITY;
use std::cmp::min;
use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, MulAssign, Neg, Sub};

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn new_from_intervals(a: Interval, b: Interval) -> Self {
        Self {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }

    pub fn contains(self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }

    pub fn size(self) -> f64 {
        self.max - self.min
    }

    pub fn expand(self, delta: f64) -> Self {
        let padding = delta / 2.;
        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
    }
}

const EMPTY: Interval = Interval {
    min: INFINITY,
    max: -INFINITY,
};

const UNIVERSE: Interval = Interval {
    min: -INFINITY,
    max: INFINITY,
};

impl Add<f64> for Interval {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Self {
            min: self.min + rhs,
            max: self.max + rhs,
        }
    }
}

impl AddAssign<f64> for Interval {
    fn add_assign(&mut self, rhs: f64) {
        *self = *self + rhs;
    }
}

impl Add<Interval> for f64 {
    type Output = Interval;

    fn add(self, rhs: Interval) -> Interval {
        rhs + self
    }
}
