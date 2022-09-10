mod camera;
mod ray;

pub use camera::Camera;
use image::RgbImage;
use itertools::iproduct;
pub use ray::Ray;

use crate::{color::Color, shapes::Object};

pub fn render(
    framebuffer: &mut RgbImage,
    objects: &[Box<dyn Object>],
    camera: &Camera,
    (width, height): (u32, u32),
) {
    for (px, py, pixel) in framebuffer.enumerate_pixels_mut() {
        let x = (px as f64 / width as f64) * 2.0 - 1.0;
        let y = (py as f64 / height as f64) * 2.0 - 1.0;
        let ray = camera.ray(x, y);
        for obj in objects {
            if obj.intersects(&ray) {
                *pixel = obj.color();
            }
        }
    }
}
