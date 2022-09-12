use image::Rgb;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

pub type Color = Rgb<u8>;
pub const WHITE: Color = Rgb([255, 255, 255]);
pub const BLACK: Color = Rgb([0, 0, 0]);
pub const MAGENTA: Color = Rgb([255, 0, 255]);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(remote = "Rgb::<u8>"))]
pub(crate) struct RgbDef([u8; 3]);

/* pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0 };

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}
impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }
    pub fn gamma(&self, exposure: f64, gamma: f64) -> Self {
        Self {
            r: (self.r * exposure).powf(gamma),
            g: (self.g * exposure).powf(gamma),
            b: (self.b * exposure).powf(gamma),
        }
    }
    pub fn as_rgb8(&self) -> [u8; 3] {
        let red = (self.r * 255.0).floor() as u8;
        let green = (self.g * 255.0).floor() as u8;
        let blue = (self.b * 255.0).floor() as u8;
        [red, green, blue]
    }
}
impl core::ops::Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}
impl core::ops::Mul for Color {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}
impl Default for Color {
    fn default() -> Self {
        Color::new(0.0, 0.0, 0.0)
    }
}
 */
