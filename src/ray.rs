use cgmath::*;

pub struct Ray {
    pub origin: Vector3<f64>,
    pub dir: Vector3<f64>,
}

impl Ray {
    pub fn new(orig: Vector3<f64>, direction: Vector3<f64>) -> Ray {
        Self {
            origin: orig,
            dir: direction,
        }
    }

    pub fn at(self: &Self, t: f64) -> Vector3<f64> {
        self.origin + t*self.dir
    }
}