use cgmath::*;
use crate::ray::Ray;

pub trait Hittable {
    // Rust doesn't like using "out parameters", don't pass the Hittable as a param.
    fn hit(self: &Self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub point: Vector3::<f64>,
    pub normal: Vector3::<f64>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        Self {
            point: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            normal: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            t: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(self: &mut Self, ray: &Ray, outward_normal: Vector3<f64>) {
        self.front_face = ray.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal } ;
    }
}