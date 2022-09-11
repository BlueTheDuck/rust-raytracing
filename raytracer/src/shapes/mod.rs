use crate::{scene::Ray, color::{Color, WHITE}};

pub type Vector = na::Vector3<f64>;

mod plane;
mod sphere;

pub use plane::Plane;
pub use sphere::Sphere;

pub enum Intersection {
    Hit(f64),
    Miss,
}

pub trait Object {
    fn intersects(&self, ray: &Ray) -> bool {
        let t = self.distance(ray);
        match t {
            Intersection::Hit(t) if t >= 0.0 => true,
            _ => false,           
        }
    }
    fn distance(&self, ray: &Ray) -> Intersection;
    fn color(&self) -> Color {
        WHITE
    }
}
