use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec3::Vec3;
use ray_tracing::random_double;

pub trait Material: Sync {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Clone, Copy, Default)]
pub struct Lambertian<T: Texture> {
    albedo: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(a: T) -> Self {
        Self { albedo: a }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
            eprintln!("NEAR ZERO")
        }

        *scattered = Ray::new_with_time(rec.p, scatter_direction, r_in.time());
        *attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        true
    }
}

#[derive(Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(a: Color, f: f64) -> Self {
        Self { albedo: a, fuzz: f }
    }
}

impl Default for Metal {
    fn default() -> Self {
        Self {
            albedo: Color::new(1., 1., 1.),
            fuzz: 5.,
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(r_in.direction().unit_vector(), rec.normal);
        *scattered = Ray::new_with_time(
            rec.p,
            reflected + self.fuzz * Vec3::random_unit_vector(),
            r_in.time(),
        );
        *attenuation = self.albedo;
        true
    }
}

#[derive(Clone, Copy)]
pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }

    /**
     * Schlick's approximation for reflectance.
     */
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0: f64 = (1f64 - ref_idx) / (1f64 + ref_idx);
        r0 = r0.powi(2);
        r0 + (1f64 - r0) * (1f64 - cosine).powi(5)
    }
}

impl Default for Dielectric {
    fn default() -> Self {
        Self { ir: 1. }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1. / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = -unit_direction.dot(rec.normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.;
        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > random_double()
        {
            Vec3::reflect(unit_direction, rec.normal)
        } else {
            Vec3::refract(unit_direction, rec.normal, refraction_ratio)
        };

        *scattered = Ray::new_with_time(rec.p, direction, r_in.time());
        true
    }
}
