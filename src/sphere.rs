use crate::hittable::*;
use cgmath::*;
use crate::ray::Ray;

pub struct Sphere {
    center: Vector3::<f64>,
    radius: f64
}

impl Sphere {
    pub fn new(center: Vector3::<f64>, radius: f64) -> Sphere{
        Self {
            center: center,
            radius: radius,
        }
    }
}

impl Hittable for Sphere {
    
fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vector3<f64> = ray.origin - self.center;

        let a = ray.origin.distance2(ray.dir);
        let half_b = oc.dot(ray.dir);
        let c = oc.magnitude2() - self.radius*self.radius;
        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            None
        } else {
            let sqrt_discriminant = discriminant.sqrt();

            // find nearest root that lies in the acceptable range
            let mut root = (-half_b - sqrt_discriminant) / a;
            if root < t_min || root > t_max {
                root = (-half_b + sqrt_discriminant) / a;
                if root < t_min || root > t_max {
                    return None
                }
            }

            let point = ray.at(root);
            let outward_normal = (point - self.center) / self.radius;

            let mut hit: HitRecord = HitRecord { point: point, normal: outward_normal, t: root, front_face: false };
            hit.set_face_normal(ray, outward_normal);

            Some(hit)
        }
    }
}