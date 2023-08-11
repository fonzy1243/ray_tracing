use std::cmp::min;
use std::ops::{Add, Div, Index, IndexMut, Mul, MulAssign, Neg, Sub};
use ray_tracing::{random_double, random_double_r};

#[derive(Clone, Copy, PartialEq)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(e1: f64, e2: f64, e3: f64) -> Self {
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

    pub fn near_zero(self) -> bool {
        let s = 1e-8;
        (self[0].abs() < s) && (self[1].abs() < s) && (self[2].abs() < s)
    }

    pub fn dot(self, v: Vec3) -> f64 {
        self[0] * v[0] + self[1] * v[1] + self[2] * v[2]
    }

    pub fn cross(self, v: Vec3) -> Self {
        Vec3 {
            e: [
                self[1] * v[2] - self[2] * v[1],
                self[2] * v[0] - self[0] * v[2],
                self[0] * v[1] - self[1] * v[0],
            ],
        }
    }

    pub fn unit_vector(self) -> Self {
        self / self.length()
    }

    pub fn random_in_unit_sphere() -> Vec3{
        loop {
            let p = Vec3::random_range(-1.,1.);
            if p.length_squared() < 1. {
                return p
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0. {
            on_unit_sphere
        }
        else {
            -on_unit_sphere
        }
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2f64 * v.dot(n) * n
    }

    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = -uv.dot(n).min(1.);
        let r_out_perp = etai_over_etat * (uv + cos_theta * n);
        let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * n;

        r_out_perp + r_out_parallel
    }

    pub fn random(self) -> Self {
        Self { e: [random_double(), random_double(), random_double()] }
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Self { e: [random_double_r(min, max), random_double_r(min, max), random_double_r(min, max)] }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
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

    fn add(self, rhs: Self) -> Self::Output {
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
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            e: [self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]]
        }
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

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
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

pub type Point3 = Vec3;