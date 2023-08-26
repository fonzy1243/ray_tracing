use crate::color::*;
use crate::interval::Interval;
use crate::vec3::*;
use image::io::Reader;
use image::*;
use std::io::Error;
use std::path::Path;

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}

#[derive(Clone, Copy, Default)]
pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(c: Color) -> Self {
        Self { color_value: c }
    }

    pub fn new_from_rgb(red: f64, green: f64, blue: f64) -> Self {
        Self::new(Color::new(red, green, blue))
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        self.color_value
    }
}

#[derive(Clone)]
pub struct CheckerTexture<T: Texture, U: Texture> {
    inv_scale: f64,
    even: T,
    odd: U,
}

impl<T: Texture, U: Texture> CheckerTexture<T, U> {
    pub fn new(scale: f64, even: T, odd: U) -> Self {
        Self {
            inv_scale: 1. / scale,
            even,
            odd,
        }
    }
}

impl CheckerTexture<SolidColor, SolidColor> {
    pub fn new_from_colors(scale: f64, c1: Color, c2: Color) -> Self {
        Self {
            inv_scale: 1. / scale,
            even: SolidColor::new(c1),
            odd: SolidColor::new(c2),
        }
    }
}
impl<T: Texture, U: Texture> Texture for CheckerTexture<T, U> {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let x_integer = (self.inv_scale * p.x()).floor() as i32;
        let y_integer = (self.inv_scale * p.y()).floor() as i32;
        let z_integer = (self.inv_scale * p.z()).floor() as i32;

        let is_even = (x_integer + y_integer + z_integer) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

pub struct ImageTexture {
    image: Option<RgbaImage>,
}

impl ImageTexture {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let image = Reader::open(path)?.decode().ok().map(|x| x.into_rgba8());
        Ok(Self { image })
    }
}

impl Texture for ImageTexture {
    fn value(&self, mut u: f64, mut v: f64, p: Point3) -> Color {
        match &self.image {
            None => Color::new(0., 1., 1.),
            Some(image) => {
                u = Interval::new(0., 1.).clamp(u);
                v = 1. - Interval::new(0., 1.).clamp(v);

                let i = u as u32 * image.height();
                let j = v as u32 * image.width();
                let pixel = image.get_pixel(i, j);

                let color_scale = 1. / 255.;
                Color::new(
                    color_scale * pixel[0] as f64,
                    color_scale * pixel[1] as f64,
                    color_scale * pixel[2] as f64,
                )
            }
        }
    }
}
