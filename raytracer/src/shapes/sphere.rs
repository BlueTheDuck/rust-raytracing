use super::{Intersection, Object, Vector};
use crate::{
    color::{Color, MAGENTA},
    scene::Ray,
};

pub struct Sphere {
    pub origin: Vector,
    pub radius: f64,
    pub color: Color,
}
impl Sphere {
    pub fn new(origin: Vector, radius: f64) -> Self {
        debug_assert!(radius > 0.0);
        Sphere {
            origin,
            radius,
            color: MAGENTA,
        }
    }
    pub fn new_with_color(origin: Vector, radius: f64, color: Color) -> Self {
        let this = Sphere::new(origin, radius);
        Sphere { color, ..this }
    }
}
impl Object for Sphere {
    fn intersects(&self, ray: &Ray) -> bool {
        /*
          We can check if the ray crosses the sphere
          by moving `dt` along the ray. If we are touching the sphere
          then we can say the ray intersects the sphere
        */

        /* Distance between origin and center of sphere */
        let dt = ray.origin.metric_distance(&self.origin);
        /* Move until we are (supposedly) next to the center of the sphere */
        let maybe_center = ray.at(dt);
        maybe_center.metric_distance(&self.origin) < self.radius
    }
    fn distance(&self, ray: &Ray) -> Intersection {
        /*
        /* Distance between origin and center of sphere */
        let dt = ray.origin.metric_distance(&self.origin);
        /* Move until we are (supposedly) next to the center of the sphere */
        let maybe_center = ray.at(dt);
        /* Distance between ray[dt] and the center of the sphere */
        let here_center = maybe_center.metric_distance(&self.origin); */

        // Translate ray to center of sphere
        let ray = ray - self.origin;
        /*
          We need to find the point on the ray that is closest to the center of the sphere
          Ray: R(t) = P + t * d
          Sphere: ∀p∈ℝ³ /||p|| = r
          Distance from point R(t) to the center: ||R(t)||
          Point R(t) at distance r: ||R(t)|| = r
          Solve:
            ||R(t)|| = r
            ||P + t * d|| = r²
            (P + dt)(P + dt) = r²
            P² + 2Pdt + dt² - r² = 0
            (d²)t² + 2Pdt + (||P||² - r²) = 0
            a = ||d||²
            b = 2Pd
            c = ||P||² - r²
        */
        // ||d||²
        let a = ray.direction.norm_squared();
        // 2Pd
        let b = 2.0 * ray.origin.dot(&ray.direction);
        // ||P||² - r²
        let c = ray.origin.norm_squared() - self.radius.powi(2);
        // Discriminant
        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant > 0.0 {
            // 2 hit points / ray goes through sphere
            let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b - discriminant.sqrt()) / (2.0 * a);
            Intersection::Hit(t1.min(t2))
        } else if discriminant == 0.0 {
            // 1 hit point / ray is tangent to sphere
            Intersection::Hit(-b / (2.0 * a))
        } else {
            // No hit points / ray misses sphere
            Intersection::Miss
        }
    }
    fn color(&self) -> Color {
        self.color
    }
}
