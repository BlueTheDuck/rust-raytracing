use std::fmt::Display;

use crate::{
    color::{Color, WHITE},
    scene::Ray,
};

mod plane;
mod sphere;

pub use plane::Plane;
pub use sphere::Sphere;

pub type Vector = na::Vector3<f64>;

pub const ORIGIN: Vector = na::Vector3::new(0.0, 0.0, 0.0);
pub const ZP: Vector = na::Vector3::new(0.0, 0.0, 1.0);
pub const ZN: Vector = na::Vector3::new(0.0, 0.0, -1.0);
pub const XP: Vector = na::Vector3::new(1.0, 0.0, 0.0);
pub const XN: Vector = na::Vector3::new(-1.0, 0.0, 0.0);
pub const YP: Vector = na::Vector3::new(0.0, 1.0, 0.0);
pub const YN: Vector = na::Vector3::new(0.0, -1.0, 0.0);

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
impl Shape {
    pub fn new_sphere(origin: [f64; 3], radius: f64) -> Self {
        let [x, y, z] = origin;
        Shape::Sphere(Sphere::new(na::Vector3::new(x, y, z), radius))
    }
    pub fn new_plane(origin: [f64; 3], normal: [f64; 3]) -> Self {
        let [x, y, z] = origin;
        let [nx, ny, nz] = normal;
        Shape::Plane(Plane::new(
            na::Vector3::new(x, y, z),
            na::Vector3::new(nx, ny, nz),
        ))
    }
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
    fn set_color(&mut self, color: Color) {
        match self {
            Shape::Sphere(sphere) => sphere.set_color(color),
            Shape::Plane(plane) => plane.set_color(color),
        }
    }

    fn into_shape(self) -> Shape {
        self
    }
    fn pos(&self) -> &Vector {
        match self {
            Shape::Sphere(sphere) => sphere.pos(),
            Shape::Plane(plane) => plane.pos(),
        }
    }
    fn set_pos(&mut self, pos: Vector) {
        match self {
            Shape::Sphere(sphere) => sphere.set_pos(pos),
            Shape::Plane(plane) => plane.set_pos(pos),
        }
    }

    fn normal(&self, point: Vector) -> Vector {
        match self {
            Shape::Sphere(sphere) => sphere.normal(point),
            Shape::Plane(plane) => plane.normal(point),
        }
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
    fn normal(&self, point: Vector) -> Vector;
    fn color(&self) -> Color {
        WHITE
    }
    fn set_color(&mut self, color: Color);
    fn into_shape(self) -> Shape;
    fn pos(&self) -> &Vector;
    fn set_pos(&mut self, pos: Vector);  
}
impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shape::Sphere(sphere) => write!(f, "{}", sphere),
            Shape::Plane(plane) => write!(f, "{}", plane),
        }
    }
}
