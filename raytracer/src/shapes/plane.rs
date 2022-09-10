use super::Object;
use crate::{scene::Ray, color::{Color, MAGENTA}};

pub struct Plane {
    pub origin: na::Vector3<f64>,
    pub normal: na::Vector3<f64>,
    pub color: Color,
}
impl Plane {
    pub fn new(origin: na::Vector3<f64>, normal: na::Vector3<f64>) -> Self {
        Plane {
            origin: origin.normalize(),
            normal: normal.normalize(),
            color: MAGENTA,
        }
    }
    pub fn new_with_color(
        origin: na::Vector3<f64>,
        normal: na::Vector3<f64>,
        color: Color,
    ) -> Self {
        let this = Plane::new(origin, normal);
        Plane { color, ..this }
    }
}
impl Object for Plane {
    fn intersects(&self, ray: &Ray) -> bool {
        let denom = self.normal.dot(&ray.direction);
        if denom.abs() < f64::EPSILON {
            return false;
        }
        let t = (self.origin - ray.origin).dot(&self.normal) / denom;
        /* println!("{t}"); */
        /* TODO: Check for really big `t` t < 3.0 && */
        t > 0.0
    }
    fn color(&self) -> Color {
        self.color
    }
}
