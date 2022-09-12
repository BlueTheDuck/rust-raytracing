mod camera;
mod ray;

pub use camera::Camera;
use image::RgbImage;
pub use ray::Ray;

use crate::{
    color::Color,
    shapes::{Intersection, Object, Shape},
};

pub fn render(
    framebuffer: &mut RgbImage,
    objects: &[Shape],
    camera: &Camera,
) {
    let width = framebuffer.width();
    let height = framebuffer.height();
    for (px, py, pixel) in framebuffer.enumerate_pixels_mut() {
        let x = (px as f64 / width as f64) * 2.0 - 1.0;
        let y = (py as f64 / height as f64) * 2.0 - 1.0;
        let ray = camera.ray(x, y);
        let mut closest: Option<(f64, Color)> = None;
        for obj in objects {
            if let Intersection::Hit(t) = obj.distance(&ray) {
                debug_assert!(t >= 0.0 && t.is_finite());
                if let Some((closest_t, _)) = closest {
                    if t < closest_t {
                        closest = Some((t, obj.color()));
                    }
                } else {
                    closest = Some((t, obj.color()));
                }
            }
        }
        if let Some((_, color)) = closest {
            *pixel = color;
        }
    }
}
