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

#[derive(Clone, Debug)]
pub struct ImageTexture {
    image: Option<RgbImage>,
}

impl ImageTexture {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let image = Reader::open(path)?.decode().ok().map(|x| x.to_rgb8());
        Ok(Self { image })
    }
}

impl Texture for ImageTexture {
    fn value(&self, mut u: f64, mut v: f64, _p: Point3) -> Color {
        match &self.image {
            None => Color::new(0., 1., 1.),
            Some(image) => {
                u = Interval::new(0., 1.).clamp(u);
                v = 1. - Interval::new(0., 1.).clamp(v);

                let mut i = (u * image.width() as f64) as u32;
                let mut j = (v * image.height() as f64) as u32;

                if i >= image.width() {
                    i = image.width() - 1;
                }

                if j >= image.height() {
                    j = image.height() - 1;
                }

                let pixel = image.get_pixel(i, j).0;

                let color_scale = 1. / 255.;

                let r = color_scale * pixel[0] as f64;
                let g = color_scale * pixel[1] as f64;
                let b = color_scale * pixel[2] as f64;

                Color::new(r, g, b)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image() {
        let image_texture = ImageTexture::new("test_image.png");
        assert!(image_texture.is_ok(), "Image failed to load.");
        assert!(
            image_texture.as_ref().is_ok_and(|x| x.image.is_some()),
            "Image loaded but struct has None."
        );
        assert!(
            if let Some(image) = &image_texture.unwrap().image {
                if image.get_pixel(330, 830).0 == [255, 255, 255] {
                    true
                } else {
                    let pixel = image.get_pixel(330, 830).0;
                    eprintln!("R: {} G: {} B: {}", pixel[0], pixel[1], pixel[2]);
                    false
                }
            } else {
                false
            },
            "Image color does not match."
        );
    }
}
