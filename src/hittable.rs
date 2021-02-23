use cgmath::Vector3;
use crate::ray::Ray;

pub trait Hittable {
    fn hit(self: &Self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct HitRecord {
    pub point: Vector3::<f64>,
    pub normal: Vector3::<f64>,
    pub t: f64
}

impl HitRecord {
    pub fn new() -> HitRecord {
        Self {
            point: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            normal: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            t: 0.0,
        }
    }
}