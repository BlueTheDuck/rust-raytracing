use crate::scene::Ray;

/// Perspective camera
pub struct Camera {
    origin: na::Vector3<f64>,
    forward: na::Vector3<f64>,
    up: na::Vector3<f64>,
    right: na::Vector3<f64>,
    width: f64,
    height: f64,
}
impl Camera {
    pub fn new(fov: f64, target: na::Vector3<f64>, origin: na::Vector3<f64>) -> Self {
        debug_assert!(fov > 0.0 && fov < 90.0);
        const UPGUIDE: na::Vector3<f64> = na::Vector3::new(0.0, -1.0, -1.0);
        let forward = (target - origin).normalize();
        let right = forward.cross(&UPGUIDE).normalize();
        let up = right.cross(&forward).normalize();
        let height = fov.tan();
        const ASPECT_RATIO: f64 = 16.0 / 9.0;
        let width = height * ASPECT_RATIO;
        Camera {
            origin,
            forward,
            up,
            right,
            width,
            height,
        }
    }
    pub fn ray(&self, x: f64, y: f64) -> Ray {
        debug_assert!(x >= -1.0 && x <= 1.0, "({x}; {y})");
        debug_assert!(y >= -1.0 && y <= 1.0, "({x}; {y})");
        let direction: na::Vector3<f64> =
            self.forward + self.right * x * self.width + self.up * y * self.height;
        Ray {
            origin: self.origin,
            direction: direction.normalize(),
        }
    }
}
