use rand::Rng;
use cgmath::InnerSpace;
use cgmath::Vector3;

pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod hittable_list;
pub mod camera;
pub mod material;

static PI: f64 = 3.1415926535897932385;

pub fn random_unit_vector() -> Vector3<f64> {
    random_in_unit_sphere().normalize()
}

pub fn random_in_unit_sphere() -> Vector3<f64> {
    let mut rng = rand::thread_rng();
    let unit = Vector3::new(1.0, 1.0, 1.0);
    loop {
        let p = 2.0 * Vector3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) - unit;
        if p.magnitude2() < 1.0 {
            return p
        }
    }
}

pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0, 1.0)
}

pub fn random_color() -> Vector3<f64> {
    let mut rng = rand::thread_rng();
    Vector3{ x: rng.gen_range(0.0, 1.0) * 255.0, y: rng.gen_range(0.0, 1.0)*255.0, z: rng.gen_range(0.0, 1.0)*255.0 }
}

fn random_in_unit_disk() -> Vector3<f64> {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vector3 { x: rng.gen_range(-1.0, 1.0), y: rng.gen_range(-1.0, 1.0), z: 0.0 };
        if p.magnitude2() < 1.0 { return p; }
    }
}

fn deg_to_rad(degrees: f64) -> f64 {
    degrees * crate::PI / 180.0
}