use crate::INFINITY;

pub struct Interval {
    min: f64,
    max: f64
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self {
            min,
            max,
        }
    }

    pub fn contains(self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}

const EMPTY: Interval = Interval::new(INFINITY, -INFINITY);
const UNIVERSE: Interval = Interval::new(-INFINITY, INFINITY);