use crate::{scene::Ray, color::{Color, WHITE}};

pub type Vector = na::Vector3<f64>;

mod plane;
mod sphere;

pub use plane::Plane;
pub use sphere::Sphere;

pub trait Object {
    fn intersects(&self, ray: &Ray) -> bool;
    fn color(&self) -> Color {
        WHITE
    }
}
