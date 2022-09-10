pub struct Ray {
    pub origin: na::Vector3<f64>,
    pub direction: na::Vector3<f64>,
}
impl Ray {
    pub fn at(&self, t: f64) -> na::Vector3<f64> {
        self.origin + self.direction * t
    }
}
