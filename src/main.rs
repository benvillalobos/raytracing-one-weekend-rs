// Following: https://raytracing.github.io/books/RayTracingInOneWeekend.html
//#![allow(unused_variables)]
#![allow(dead_code)]

use raytracing::hittable::Hittable;
use crate::rngs::ThreadRng;
use raytracing::camera::Camera;
use cgmath::*;
use raytracing::ray::*;
use raytracing::sphere::*;
use raytracing::hittable_list::*;
use raytracing::material::*;
use rand::*;

static PI: f64 = 3.1415926535897932385;
static INFINITY: f64 = f64::MAX;


// cargo run > img.ppm
fn main() {
    // Tools
    let mut rng = thread_rng();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 400;
    let img_height = (img_width as f64/aspect_ratio) as i32;

    // Constants
    let camera = Camera::new();
    let samples_per_pixel = 100;
    let max_depth = 50;
    
    let mut objects = HittableList::new();

    objects.push(Sphere::new(Vector3 { x: 0.0, y: 0.0, z: -1.0 }, 0.5, Vector3 { x: 0.7, y: 0.3, z: 0.3 }, Lambertian::new(0.0, Vector3 { x: 0.7, y: 0.3, z: 0.3 })));
    objects.push(Sphere::new(Vector3 { x: -1.0, y: 0.0, z: -1.0 }, 0.5, Vector3 { x: 0.8, y: 0.8, z: 0.8 }, Lambertian::new(0.0, Vector3 { x: 0.7, y: 0.3, z: 0.3 })));
    objects.push(Sphere::new(Vector3 { x: 1.0, y: 0.0, z: -1.0 }, 0.5, Vector3 { x: 0.8, y: 0.6, z: 0.2 }, Lambertian::new(0.0, Vector3 { x: 0.7, y: 0.3, z: 0.3 })));
    objects.push(Sphere::new(Vector3 { x: 0.0, y: -100.5, z: -1.0 }, 100.0, Vector3 { x: 0.8, y: 0.8, z: 0.0 }, Lambertian::new(0.0, Vector3 { x: 0.7, y: 0.3, z: 0.3 })));

    println!("P3\n{} {}\n255", img_width, img_height);

    for y in (0..img_height).rev() {
    eprintln!("Scanlines remaining: {}", y);
        for x in 0..img_width {
            let mut sampled_pixel = Vector3::<f64>::new(0.0, 0.0, 0.0);
            // Antialiasing: The edges of a pixel should be the "average" of colors around it.
            for _ in 0..samples_per_pixel {
                let u: f64 = (x  as f64 + rng.gen_range(0.0, 1.0))/(img_width-1) as f64;
                let v: f64 = (y  as f64 + rng.gen_range(0.0, 1.0))/(img_height-1) as f64;
    
                let r = camera.get_ray(u, v);
    
                sampled_pixel += ray_color(&r, &objects, &mut rng, max_depth);
            }

            write_color(sampled_pixel, samples_per_pixel as f64);
        }
    }
    eprintln!("Done");
}

fn write_color(color: Vector3<f64>, samples_per_pixel: f64) {
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;

    // Divide color by number of samples
    let scale = 1.0 / samples_per_pixel;
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    println!("{} {} {}", (256.0 * clamp(r, 0.0, 0.999)) as i32, (256.0 * clamp(g, 0.0, 0.999)) as i32, (256.0 * clamp(b, 0.0, 0.999)) as i32);
}

fn ray_color(ray: &Ray, drawables: &HittableList, rng: &mut ThreadRng, depth: i32) -> Vector3<f64>{

    // Don't let the stack overflow
    if depth <= 0 {
        return Vector3::<f64>::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = drawables.hit(ray, 0.001, INFINITY) {
        let r = hit.material.scatter(&ray, &hit, rng);
        let col = ray_color(&r, &drawables, rng, depth-1);
        return Vector3 {x: col.x * hit.color.clone().x, y: col.y * hit.color.clone().y, z: col.z * hit.color.clone().z };
    }
    get_background_color(&ray)
}

fn get_background_color(ray: &Ray) -> Vector3<f64> {
        // Normalize vector so we have y between -1 and 1.
        let unit_dir = ray.dir.normalize();

        // Add 1 to y so y's bounds are [0.0, 2.0]
        // Multiply that by 0.5 so the bounds are [0.0, 1.0]
        // Let t be the scale (from [0.0, 1.0]) of white or blue.
        let t = 0.5 * (unit_dir.y + 1.0);
    
        // When t is 1 (max height), the first segment
        // of addition is (1.0 - 1.0)*white so no white is produced at top
        // When t is 0 (min height), second segment becomes 
        // 0*blue (no blue produced at bottom of image)
        // In other words, linear interpolation.
        (1.0-t)*Vector3::new(1.0, 1.0, 1.0) + t*Vector3::new(0.5, 0.7, 1.0)
}

fn deg_to_rad(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    return if x < min { min } else if x > max { max } else { x }
}

fn random_vec3(rng: &mut ThreadRng, min: f64, max: f64) -> Vector3<f64> {
    Vector3::<f64>::new(rng.gen_range(min, max), rng.gen_range(min, max), rng.gen_range(min, max))
}

fn random_unit_vector(rng: &mut ThreadRng) -> Vector3<f64> {
    random_in_unit_sphere(rng).normalize()
}

fn random_in_hemisphere(normal: Vector3<f64>, rng: &mut ThreadRng) -> Vector3<f64> {
    let in_unit_sphere = random_in_unit_sphere(rng);
    // In the same hemisphere as the normal?
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vector3<f64> {
    loop {
        let p = random_vec3(rng, -1.0, 1.0);
        if p.magnitude2() >= 1.0 {continue;}
        return p;
    }
}

fn near_zero(vec: Vector3<f64>) -> bool {
    let s = 1e-8;
    return vec.x < s && vec.y < s && vec.z < s;
}

fn reflect(v: Vector3<f64>, n: Vector3<f64>) -> Vector3<f64> {
    v - 2.0 * v.dot(n)*n
}

fn refract(uv: Vector3<f64>, n: Vector3<f64>, etai_over_etat: f64) -> Vector3<f64> {
    let cos_theta = clamp((-uv).dot(n), 1.0, INFINITY);

    let r_out_perp = etai_over_etat * (uv + cos_theta*n);
    let r_out_parallel = -((1.0 - r_out_perp.magnitude2()).abs().sqrt()) * n;
    r_out_perp + r_out_parallel
}