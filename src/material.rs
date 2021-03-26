use cgmath::*;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use rand::*;

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    return if x < min { min } else if x > max { max } else { x }
}

fn reflect(v: Vector3<f64>, n: Vector3<f64>) -> Vector3<f64> {
    v - 2.0 * v.dot(n)*n
}

fn random_unit_vector() -> Vector3<f64> {
    random_in_unit_sphere().normalize()
}

fn random_in_unit_sphere() -> Vector3<f64> {
    let mut rng = rand::thread_rng();
    let unit = Vector3::new(1.0, 1.0, 1.0);
    loop {
        let p = 2.0 * Vector3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) - unit;
        if p.magnitude2() < 1.0 {
            return p
        }
    }
}

fn near_zero(vec: Vector3<f64>) -> bool {
    let s = 1e-8;
    return vec.x < s && vec.y < s && vec.z < s;
}

fn refract(uv: Vector3<f64>, n: Vector3<f64>, etai_over_etat: f64) -> Vector3<f64> {
    let cos_theta = if (-uv).dot(n) > 1.0 { 1.0 } else { (-uv).dot(n) };

    let r_out_perp = etai_over_etat * (uv + cos_theta*n);
    let r_out_parallel = -((1.0 - r_out_perp.magnitude2()).abs().sqrt()) * n;
    r_out_perp + r_out_parallel
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f64>)>;
}

pub struct Metal {
    albedo: Vector3<f64>,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vector3<f64>, fuzz: f64) -> Self {
        Metal {
            albedo: albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let mut reflected = reflect(ray.dir.normalize(), hit.normal);
        if self.fuzz > 0.0 { reflected += self.fuzz * random_in_unit_sphere() }
        if reflected.dot(hit.normal) > 0.0 {
            let scattered = Ray::new(hit.point, reflected);
            Some((scattered, self.albedo))
        }
        else {
            None
        }
    }
}

pub struct Lambertian {
    albedo: Vector3<f64>,
}

impl Lambertian {
    pub fn new(albedo: Vector3<f64>) -> Self {
        Lambertian {
            albedo: albedo,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let mut scatter_direction = hit.normal + random_unit_vector();

        if near_zero(scatter_direction) {
            scatter_direction = hit.normal;
        }

        Some((Ray::new(hit.point, scatter_direction), self.albedo))
    }
}

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Dielectric {
            ir: ir,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let attenuation = Vector3::<f64>::new(1.0, 1.0, 1.0);

        let refraction_ratio = if hit.front_face {1.0/self.ir} else {self.ir};
        let unit_direction = ray.dir.normalize();

        let cos_theta = clamp((-unit_direction).dot(hit.normal), 1.0, f64::MAX);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vector3<f64>;

        if cannot_refract || reflectance(cos_theta, refraction_ratio) > random::<f64>() {
            direction = reflect(unit_direction, hit.normal);
        }
        else {
            direction = refract(unit_direction, hit.normal, refraction_ratio);
        }

        Some((Ray::new(hit.point, direction), attenuation))
    }
}

fn reflectance(cosine: f64, refraction: f64) -> f64 {
    // Schlick's approximation
    let mut r0 = (1.0 - refraction) / (1.0 + refraction);
    r0 = r0*r0;
    r0 + (1.0 - r0)*(1.0 - cosine).powf(5.0)
}