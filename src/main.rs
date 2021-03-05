// Following: https://raytracing.github.io/books/RayTracingInOneWeekend.html
//#![allow(unused_variables)]
#![allow(dead_code)]

use crate::rngs::ThreadRng;
use raytracing::camera::Camera;
use cgmath::*;
use raytracing::ray::*;
use raytracing::sphere::*;
use raytracing::hittable_list::*;
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
    objects.push(Sphere::new(Vector3::<f64>::new(0.0, 0.0, -1.0), 0.5));
    objects.push(Sphere::new(Vector3::<f64>::new(0.0, -100.5, -1.0), 100.0));

    println!("P3\n{} {}\n255", img_width, img_height);

    for y in (0..img_height).rev() {
    eprintln!("Scanlines remaining: {}", y);
        for x in 0..img_width {
            let mut sampled_pixel = Vector3::<f64>::new(0.0, 0.0, 0.0);
            // Antialiasing: The edges of a pixel should be the "average" of colors around it.
            for _ in 0..samples_per_pixel {
                let u: f64 = (x  as f64 + rng.gen_range(0.0, 1.0))/(img_width-1) as f64;
                let v: f64 = (y  as f64 + rng.gen_range(0.0, 1.0))/(img_height-1) as f64;
    
                // Remember that lower_left_corner is pushed out from origin.
                let r = camera.get_ray(u, v);
    
                sampled_pixel += ray_color(r, &objects, &mut rng, max_depth);
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
    r *= scale;
    g *= scale;
    b *= scale;

    println!("{} {} {}", (256.0 * clamp(r, 0.0, 0.999)) as i32, (256.0 * clamp(g, 0.0, 0.999)) as i32, (256.0 * clamp(b, 0.0, 0.999)) as i32);
}

fn ray_color(ray: Ray, drawables: &HittableList, rng: &mut ThreadRng, depth: i32) -> Vector3<f64>{

    // Don't let the stack overflow
    if depth <= 0 {
        return Vector3::<f64>::new(0.0, 0.0, 0.0);
    }

    for sprite in &drawables.objects {
        if let Some(hit) = sprite.hit(&ray, 0.0, INFINITY) {
            let target = hit.point + hit.normal + random_in_unit_sphere(rng);

            // Generate a color using a reflection in a random direction from where the
            // object was hit.
            return 0.5 * ray_color(Ray::new(hit.point, target - hit.point), drawables, rng, depth-1);
        }
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

fn hit_sphere(center: Vector3<f64>, radius: f64, ray: &Ray) -> f64 {
    let oc: Vector3<f64> = ray.origin - center;

    let a = ray.origin.distance2(ray.dir);
    let half_b = oc.dot(ray.dir);
    let c = oc.magnitude2() - radius*radius;
    let discriminant = half_b*half_b - a*c;
    if discriminant < 0.0 {
        -1.0
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
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

fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vector3<f64> {
    loop {
        let p = random_vec3(rng, -1.0, 1.0);
        if p.magnitude2() >= 1.0 {continue;}
        return p;
    }
}