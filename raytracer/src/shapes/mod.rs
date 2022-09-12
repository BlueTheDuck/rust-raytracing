use std::any::Any;

use crate::{
    color::{Color, WHITE},
    scene::Ray,
};

pub type Vector = na::Vector3<f64>;

mod plane;
mod sphere;

pub use plane::Plane;
pub use sphere::Sphere;

#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type")
)]
#[derive(Clone, Copy)]

pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
}
impl Object for Shape {
    fn distance(&self, ray: &Ray) -> Intersection {
        match self {
            Shape::Sphere(sphere) => sphere.distance(ray),
            Shape::Plane(plane) => plane.distance(ray),
        }
    }
    fn color(&self) -> Color {
        match self {
            Shape::Sphere(sphere) => sphere.color(),
            Shape::Plane(plane) => plane.color(),
        }
    }

    fn into_shape(self) -> Shape {
        self
    }    
}

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
    fn into_shape(self) -> Shape;
}
