use serde::{Deserialize, Serialize};

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
    pub fn new_with_color(
        origin: Vector,
        normal: Vector,
        color: Color,
    ) -> Self {
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
    fn color(&self) -> Color {
        self.color
    }
    fn into_shape(self) -> Shape {
        Shape::Plane(self)
    }
}
