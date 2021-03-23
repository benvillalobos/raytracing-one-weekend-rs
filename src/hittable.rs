use crate::material::Material;
use cgmath::*;
use crate::ray::Ray;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitRecord<'a> {
    pub point: Vector3::<f64>,
    pub normal: Vector3::<f64>,
    pub t: f64,
    pub front_face: bool,
    pub material: &'a dyn Material
}

impl HitRecord<'_> {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3<f64>) {
        self.front_face = ray.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal } ;
    }
}