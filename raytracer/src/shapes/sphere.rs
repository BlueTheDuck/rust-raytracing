use super::Object;
use crate::{scene::Ray, color::{MAGENTA, Color}};

pub struct Sphere {
    pub origin: na::Vector3<f64>,
    pub radius: f64,
    pub color: Color,
}
impl Sphere {
    pub fn new(origin: na::Vector3<f64>, radius: f64) -> Self {
        debug_assert!(radius > 0.0);
        Sphere {
            origin,
            radius,
            color: MAGENTA,
        }
    }
    pub fn new_with_color(origin: na::Vector3<f64>, radius: f64, color: Color) -> Self {
        let this = Sphere::new(origin, radius);
        Sphere { color, ..this }
    }
}
impl Object for Sphere {
    fn intersects(&self, ray: &Ray) -> bool {
        let dt = ray.origin.metric_distance(&self.origin);
        let point = ray.at(dt);
        let distance = point.metric_distance(&self.origin);
        distance < self.radius
    }
    fn color(&self) -> Color {
        self.color
    }
}
