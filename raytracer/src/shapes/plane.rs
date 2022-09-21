use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::{Intersection, Object, Shape, Vector};
use crate::{
    color::{Color, MAGENTA},
    scene::Ray,
};

#[derive(Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Plane {
    pub origin: Vector,
    pub normal: Vector,
    #[cfg_attr(feature = "serde", serde(with = "crate::color::RgbDef"))]
    pub color: Color,
}
impl Plane {
    pub fn new(origin: Vector, normal: Vector) -> Self {
        Plane {
            origin: origin.normalize(),
            normal: normal.normalize(),
            color: MAGENTA,
        }
    }
    pub fn new_with_color(origin: Vector, normal: Vector, color: Color) -> Self {
        let this = Plane::new(origin, normal);
        Plane { color, ..this }
    }
}
impl Object for Plane {
    fn distance(&self, ray: &Ray) -> Intersection {
        let denom = self.normal.dot(&ray.direction);
        if denom.abs() < f64::EPSILON {
            return Intersection::Miss;
        }
        let t = (self.origin - ray.origin).dot(&self.normal) / denom;
        if t < f64::EPSILON {
            return Intersection::Miss;
        }
        // println!("{t}");
        /* TODO: Check for really big `t` t < 3.0 && */
        return Intersection::Hit(t);
    }
    fn tangent(&self, point: Vector) -> Vector {
        self.normal
    }

    fn color(&self) -> Color {
        self.color
    }
    fn set_color(&mut self, color: Color) {
        self.color = color;
    }
    fn into_shape(self) -> Shape {
        Shape::Plane(self)
    }
    fn pos(&self) -> &Vector {
        &self.origin
    }
    fn set_pos(&mut self, pos: Vector) {
        self.origin = pos;
    }
}

impl Display for Plane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // N(X - P) = 0 <-> NX = NP
        write!(
            f,
            "({nx}, {ny}, {nz})·(X - ({ox}, {oy}, {oz})) = 0",
            nx = self.normal[0],
            ny = self.normal[1],
            nz = self.normal[2],
            ox = self.origin[0],
            oy = self.origin[1],
            oz = self.origin[2],
        )
    }
}
