use rayon::prelude::*;

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
    const SAMPLES: usize = 4;
    let width: f64 = framebuffer.width() as _;
    let height: f64 = framebuffer.height() as _;
    let rng = fastrand::Rng::new();
    // let numbers = repeat_with(|| rng.f64()).take((width * height * 2).try_into().unwrap());

    for (px, py, pixel) in framebuffer.enumerate_pixels_mut() {
        let mut color: Color = Rgb([0., 0., 0.]);
        for _ in 0..SAMPLES {
            let (dx, dy) = ((rng.f64() - 0.5) / width, (rng.f64() - 0.5) / height);

            let x = (px as f64 / width) * 2.0 - 1.0 + dx;
            let y = (py as f64 / height) * 2.0 - 1.0 + dy;
            let ray = camera.ray(x, y);
            let closest = find_closest(&ray, objects);
            if let Some((t, obj)) = closest {
                let at = ray.at(t);
                let brightness = light_brightness(at, &obj.normal(at), objects, light);
                debug_assert!(brightness.is_sign_positive(), "brightness = {brightness}");
                let this_color = obj.color().map(|c| c * brightness);
                color.apply2(&this_color, |c1, c2| c1 + c2);
            }
        }
        color.apply(|c| c / 4.0);
        color.apply(gamma_correction);
        *pixel = color;
    }
}

pub fn parallel_render(
    framebuffer: &mut Rgb32FImage,
    objects: &[Shape],
    camera: &Camera,
    light: &Vector,
) {
    let width: f64 = framebuffer.width() as _;
    let height: f64 = framebuffer.height() as _;
    framebuffer
        .enumerate_pixels_mut()
        .par_bridge()
        .for_each(|pix| {
            render_pixel(pix, (width, height), objects, camera, light);
        });
}

fn render_pixel(
    (px, py, pixel): (u32, u32, &mut Rgb<f32>),
    (width, height): (f64, f64),
    objects: &[Shape],
    camera: &Camera,
    light: &Vector,
) {
    const DITH_COUNT: usize = 4;
    // let rng = fastrand::Rng::new();

    let mut color: Color = Rgb([0., 0., 0.]);
        for _ in 0..DITH_COUNT {
            // let (dx, dy) = ((rng.f64() - 0.5) / width, (rng.f64() - 0.5) / height);
            let (dx, dy) = (0., 0.);

            let x = (px as f64 / width) * 2.0 - 1.0 + dx;
            let y = (py as f64 / height) * 2.0 - 1.0 + dy;
            let ray = camera.ray(x, y);
            let closest = find_closest(&ray, objects);
            if let Some((t, obj)) = closest {
                let at = ray.at(t);
                let brightness = light_brightness(at, &obj.normal(at), objects, light);
                debug_assert!(brightness.is_sign_positive(), "brightness = {brightness}");
                let this_color = obj.color().map(|c| c * brightness);
                color.apply2(&this_color, |c1, c2| c1 + c2);
            }
        }
        color.apply(|c| c / 4.0);
        color.apply(gamma_correction);
        *pixel = color;
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

fn gamma_correction(channel: f32) -> f32 {
    const EXP: f32 = 1.0;
    const GAMMA: f32 = 2.2;
    (channel * EXP).powf(GAMMA)
}
