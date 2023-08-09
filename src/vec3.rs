use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(self, e1: f64, e2: f64, e3: f64) -> Vec3 {
        Self { e: [e1, e2, e3] }
    }

    pub fn x(self) -> f64 {
        self.e[0]
    }

    pub fn y(self) -> f64 {
        self.e[1]
    }

    pub fn z(self) -> f64 {
        self.e[2]
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    pub fn dot(self, v: Vec3) -> f64 {
        self[0] * v[0] + self[1] * v[1] + self[2] * v[2]
    }
}

impl Index<i32> for Vec3 {
    type Output = f64;

    fn index(&self, index: i32) -> &Self::Output {
        self.e[index]
    }
}

impl IndexMut<i32> for Vec3 {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        self.e[index]
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            e: [
                -self[0],
                -self[1],
                -self[2],
            ],
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: self) -> Self::Output {
        Vec3 {
            e: [
                self[0] + rhs[0],
                self[1] + rhs[1],
                self[2] + rhs[2],
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [
                self[0] - rhs[0],
                self[1] - rhs[1],
                self[2] - rhs[2],
            ],
        }
    }
}

impl Mul for Vec3 {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2]
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            e: [
                self[0] * rhs,
                self[1] * rhs,
                self[2] * rhs,
            ],
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [
                self * rhs[0],
                self * rhs[1],
                self * rhs[2],
            ],
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            e: [
                self[0] / rhs,
                self[1] / rhs,
                self[2] / rhs,
            ],
        }
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [
                self / rhs[0],
                self / rhs[1],
                self / rhs[2],
            ],
        }
    }
}