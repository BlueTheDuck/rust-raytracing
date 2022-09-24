use itertools::Itertools;

mod camera;
mod ray;

pub use camera::Camera;
use image::{Pixel, Rgb, Rgb32FImage};
pub use ray::Ray;

use crate::{
    color::Color,
    shapes::{Intersection, Object, Shape, Vector},
};

const MIN_T: f64 = 0.001;

pub fn render(framebuffer: &mut Rgb32FImage, objects: &[Shape], camera: &Camera, light: &Vector) {
    let width = framebuffer.width();
    let height = framebuffer.height();
    for (px, py, pixel) in framebuffer.enumerate_pixels_mut() {
        let mut color: Color = Rgb([0., 0., 0.]);
        for (dx, dy) in [0, 1].iter().cartesian_product([0, 1]) {
            let x = ((px as i32 + dx) as f64 / width as f64) * 2.0 - 1.0;
            let y = ((py as i32 + dy) as f64 / height as f64) * 2.0 - 1.0;
            let ray = camera.ray(x, y);
            let closest = find_closest(&ray, objects);
            if let Some((t, obj)) = closest {
                let at = ray.at(t);
                let shadow_ray = Ray {
                    origin: at,
                    direction: (light - at).normalize(),
                };

                let brightness = light_brightness(at, &obj.tangent(at), objects, light);
                let this_color = obj.color().map(|c| c * brightness);
                color.apply2(&this_color, |c1, c2| c1 + c2);
            }
        }
        color.apply(|c| c / 4.0);
        *pixel = color;
    }
}

fn light_brightness(at: Vector, normal: &Vector, objects: &[Shape], light: &Vector) -> f32 {
    // Light vector pointing towards `at`
    let light_dir = (light - at).normalize();
    let light_ray = Ray {
        origin: at,
        direction: light_dir,
    };
    let closest = find_closest(&light_ray, objects);
    if closest.is_some() {
        0.0
    } else {
        light_dir.dot(&normal).max(0.0) as _
    }
}

fn find_closest<'s>(ray: &Ray, objects: &'s [Shape]) -> Option<(f64, &'s Shape)> {
    let mut closest = None;
    for obj in objects {
        if let Intersection::Hit(t) = obj.distance(&ray) {
            debug_assert!(t.is_finite(), "hit produced an inf");
            // debug_assert!(t >= 0.0 && t.is_finite(), "t is {t:#?}. Ray {ray} -> {obj}");
            if t.is_sign_negative() {
                continue;
            }
            // Don't collide with self
            if t.abs() < MIN_T {
                continue;
            }
            if let Some((closest_t, _)) = closest {
                if t < closest_t {
                    closest = Some((t, obj));
                }
            } else {
                closest = Some((t, obj));
            }
        }
    }

    return closest;
}
