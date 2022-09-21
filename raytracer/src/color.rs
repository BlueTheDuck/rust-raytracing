use image::Rgb;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type Color = Rgb<f32>;
pub const WHITE: Color = Rgb([1.0, 1.0, 1.0]);
pub const BLACK: Color = Rgb([0.0, 0.0, 0.0]);
pub const MAGENTA: Color = Rgb([1.0, 0.0, 1.0]);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(remote = "Rgb::<f32>"))]
pub(crate) struct RgbDef([f32; 3]);
