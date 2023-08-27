use crate::vec3::*;
use ray_tracing::random_int;

#[derive(Clone, Copy)]
pub struct Perlin {
    point_count: usize,
    ranvec: [Vec3; Self::POINT_COUNT],
    perm_x: [usize; Self::POINT_COUNT],
    perm_y: [usize; Self::POINT_COUNT],
    perm_z: [usize; Self::POINT_COUNT],
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let point_count = Self::POINT_COUNT;

        let mut ranvec = [Vec3::default(); Self::POINT_COUNT];
        for i in &mut ranvec {
            *i = Vec3::random_range(-1., 1.).unit_vector();
        }

        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();

        Self {
            point_count,
            ranvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;
        let mut c = [[[Vec3::default(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]];
                }
            }
        }

        Self::perlin_interp(&mut c, u, v, w)
    }

    fn perlin_generate_perm() -> [usize; Self::POINT_COUNT] {
        let mut p = [0usize; Self::POINT_COUNT];

        for (i, value) in p.iter_mut().enumerate() {
            *value = i;
        }

        Self::permute(&mut p, Self::POINT_COUNT);

        p
    }

    fn permute(p: &mut [usize; Self::POINT_COUNT], n: usize) {
        for i in (1..n).rev() {
            let target = usize::try_from(random_int(0, i.try_into().unwrap())).unwrap();
            p.swap(i, target);
        }
    }

    fn perlin_interp(c: &mut [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3. - 2. * u);
        let vv = v * v * (3. - 2. * v);
        let ww = w * w * (3. - 2. * w);
        let mut accum = 0.;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1 - i) as f64 * (1. - uu))
                        * (j as f64 * vv + (1 - j) as f64 * (1. - vv))
                        * (k as f64 * ww + (1 - k) as f64 * (1. - ww))
                        * c[i][j][k].dot(weight_v);
                }
            }
        }

        accum
    }

    pub fn turb(&self, p: Point3, depth: u32) -> f64 {
        if depth == 0 {
            return 0.;
        }

        let mut accum = 0.;
        let mut temp_p = p;
        let mut weight = 1.;

        for _i in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.;
        }

        accum.abs()
    }
}
