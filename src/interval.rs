use crate::INFINITY;
use std::cmp::min;

#[derive(Clone, Copy, PartialEq)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
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
