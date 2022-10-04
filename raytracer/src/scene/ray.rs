use std::fmt::Display;

use crate::shapes::Vector;

pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}
impl Ray {
    /// Creates a new ray from an origin, pointing towards a target.
    pub fn new_with_from_target(from: Vector, target: &Vector) -> Self {
        Self {
            origin: from,
            direction: (target - from).normalize(),
        }
    }
    pub fn at(&self, t: f64) -> Vector {
        self.origin + self.direction * t
    }
    pub fn bounce(&self, t: f64, normal: Vector) -> Ray {
        let direction = self.direction - 2.0 * (self.direction.dot(&normal)) * normal;
        let origin = self.at(t);
        Self { origin, direction }
    }
}
impl core::ops::Sub<Vector> for &Ray {
    type Output = Ray;
    fn sub(self, rhs: Vector) -> Self::Output {
        Ray {
            origin: self.origin - rhs,
            direction: self.direction,
        }
    }
}
impl Display for Ray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({ox}, {oy}, {oz}) + t({dx}, {dy}, {dz})",
            ox = self.origin[0],
            oy = self.origin[1],
            oz = self.origin[2],
            dx = self.direction[0],
            dy = self.direction[1],
            dz = self.direction[2]
        )
    }
}
