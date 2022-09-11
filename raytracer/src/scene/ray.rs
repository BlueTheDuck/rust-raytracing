use crate::shapes::Vector;

pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}
impl Ray {
    pub fn at(&self, t: f64) -> Vector {
        self.origin + self.direction * t
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
