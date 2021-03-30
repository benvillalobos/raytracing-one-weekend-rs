// Following: https://raytracing.github.io/books/RayTracingInOneWeekend.html
// Inspiration for rust-specific implementation taken from https://github.com/fralken/ray-tracing-in-one-weekend
//#![allow(unused_variables)]
#![allow(dead_code)]

use raytracing::random_double;
use raytracing::random_color;
use raytracing::hittable::Hittable;
use raytracing::camera::Camera;
use cgmath::*;
use raytracing::ray::*;
use raytracing::sphere::*;
use raytracing::hittable_list::*;
use raytracing::material::*;
use rand::*;
use rayon::prelude::*;

static PI: f64 = 3.1415926535897932385;
static INFINITY: f64 = f64::MAX;


// cargo run > img.ppm
fn main() {
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let img_width = 300;
    let img_height = (img_width as f64/aspect_ratio) as i32;

    // Number of "nearby" colors to get an average of for accurate color
    // Antialiasing
    let samples_per_pixel = 20;
    // Number of rays to shoot per reflection.
    let max_depth = 50;

    // Where our camera is located
    let look_from = Vector3 { x: 13.0, y: 2.0, z: 3.0 };
    // What our camera is looking at
    let look_at = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
    // Our camera's "up"
    let vup = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
    // Distance that is in focus
    let dist_to_focus = 10.0;
    // Size of the "disk" we shoot rays out from for blurring
    let aperture = 0.1;

    // Constants
    let camera = Camera::new(look_from, 
                             look_at, 
                             vup, 
                             20.0, 
                             aspect_ratio,
                             aperture,
                             dist_to_focus);

    // World
    let mut objects = HittableList::new();

    //generate_v1_world(&mut objects);
    generate_v2_world(&mut objects);
    //random_scene(&mut objects);

    println!("P3\n{} {}\n255", img_width, img_height);

    render_par(img_height, img_width, samples_per_pixel, &camera, &objects, max_depth);
    eprintln!("Done");
}

fn render(  img_height: i32, 
            img_width: i32, 
            samples_per_pixel: i32, 
            camera: Camera, 
            objects: HittableList, 
            max_depth: i32) {

    for y in (0..img_height).rev() {
        eprintln!("Scanlines remaining: {}", y);
            for x in 0..img_width {
                let mut sampled_pixel = Vector3::<f64>::new(0.0, 0.0, 0.0);
                // Antialiasing: The edges of a pixel should be the "average" of colors around it.
                for _ in 0..samples_per_pixel {
                    let u: f64 = (x  as f64 + random_double())/(img_width-1) as f64;
                    let v: f64 = (y  as f64 + random_double())/(img_height-1) as f64;
        
                    let r = camera.get_ray(u, v);
        
                    sampled_pixel += ray_color(&r, &objects, max_depth);
                }
    
                write_color(sampled_pixel, samples_per_pixel as f64);
            }
        }
}

fn render_par(  img_height: i32, 
    img_width: i32, 
    samples_per_pixel: i32, 
    camera: &Camera, 
    objects: &HittableList, 
    max_depth: i32) {

        let image: Vec<Vector3<f64>> = (0..img_height).into_par_iter().rev().flat_map(|y| {
            (0..img_width).into_par_iter().map(move |x| {
                let sampled_pixel = (0..samples_per_pixel).map(move |_| {
                    let u: f64 = (x  as f64 + random_double())/(img_width-1) as f64;
                    let v: f64 = (y  as f64 + random_double())/(img_height-1) as f64;
        
                    let r = camera.get_ray(u, v);
                    ray_color(&r, &objects, max_depth)
                }).sum();
                sampled_pixel
            })
        }).collect();

        // We've collected all colors into a list of vector3, print them.
        for col in image {
            write_color(col, samples_per_pixel as f64);
        }
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

fn ray_color(ray: &Ray, drawables: &HittableList, depth: i32) -> Vector3<f64> {

    // Don't let the stack overflow
    if depth <= 0 {
        return Vector3::<f64>::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = drawables.hit(ray, 0.001, INFINITY) {
        if let Some((r, atten)) = hit.material.scatter(&ray, &hit) {
            return atten.zip(ray_color(&r, &drawables, depth-1), |l, r| l * r);
        }
        return Vector3::<f64>::new(0.0, 0.0, 0.0);
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

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    return if x < min { min } else if x > max { max } else { x }
}

fn generate_v1_world(objects: &mut HittableList) {
    let ground_material = Lambertian::new(Vector3 { x: 0.8, y: 0.8, z: 0.0 });
    let center_material = Lambertian::new(Vector3 { x: 0.1, y: 0.2, z: 0.5 });
    let left_material = Dielectric::new(1.5);
    let left_inner_material = Dielectric::new(1.5);
    let right_material = Metal::new(Vector3 { x: 0.8, y: 0.6, z: 0.2 }, 0.0);

    let ground_sphere = Sphere::new(Vector3 { x: 0.0, y: -100.5, z: -1.0 }, 100.0, ground_material);
    let center_sphere = Sphere::new(Vector3 { x: 0.0, y: 0.0, z: -1.0 }, 0.5, center_material);
    let left_sphere = Sphere::new(Vector3 { x: -1.0, y: 0.0, z: -1.0 }, 0.5, left_material);
    let left_inner_sphere = Sphere::new(Vector3 { x: -1.0, y: 0.0, z: -1.0 }, -0.4, left_inner_material);
    let right_sphere = Sphere::new(Vector3 { x: 1.0, y: 0.0, z: -1.0 }, 0.5, right_material);

    objects.push(ground_sphere);
    objects.push(center_sphere);
    objects.push(left_sphere);
    objects.push(left_inner_sphere);
    objects.push(right_sphere);
}

fn generate_v2_world(objects: &mut HittableList) {
    let radius = (crate::PI/4.0).cos();
    let right_material = Lambertian::new(Vector3 { x: 0.0, y: 0.0, z: 1.0 });
    let right_sphere = Sphere::new(Vector3 { x: -radius, y: 0.0, z: -1.0 }, radius, right_material);

    let left_material = Lambertian::new(Vector3 { x: 1.0, y: 0.0, z: 0.0 });
    let left_sphere = Sphere::new(Vector3 { x: radius, y: 0.0, z: -1.0 }, radius, left_material);

    objects.push(left_sphere);
    objects.push(right_sphere);
}

fn random_scene(objects: &mut HittableList) {
    let mut rng = rand::thread_rng();
    let origin = Vector3 { x: 4.0, y: 0.2, z: 0.0 };
    let ground_material = Lambertian::new(Vector3 { x: 0.5, y: 0.5, z: 0.5 });
    let world = Sphere::new(Vector3 { x: 0.0, y: -1000.0, z: 0.0 }, 1000.0, ground_material);
    objects.push(world);

    for a in -11..12 {
        for b in -11..12 {
            let material_to_use = rng.gen_range(0.0, 1.0);
            let center = Vector3 { x: a as f64 + 0.9*rng.gen_range(0.0, 1.0), y: 0.2, z: b as f64 + 0.9*rng.gen_range(0.0, 1.0) };

            if (center - origin).magnitude() > 0.9 {
                if material_to_use < 0.8 { // Diffuse
                    let albedo = random_color();
                    let sphere_material = Lambertian::new(albedo);
                    let sphere = Sphere::new(center, 0.2, sphere_material);
                    objects.push(sphere);
                }
                else if material_to_use < 0.95 {
                    let albedo = random_color();
                    let fuzz = random_double();
                    let sphere_material = Metal::new(albedo, fuzz);
                    let sphere = Sphere::new(center, 0.2, sphere_material);
                    objects.push(sphere);
                }
                else {
                    let sphere_material = Dielectric::new(1.5);
                    let sphere = Sphere::new(center, 0.2, sphere_material);
                    objects.push(sphere);
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    let material2 = Lambertian::new(Vector3{x: 0.4, y: 0.2, z: 0.1});
    let material3 = Metal::new(Vector3{x: 0.7, y: 0.6, z: 0.5}, 0.0);

    let sphere1 = Sphere::new(Vector3{x: 0.0, y: 1.0, z: 0.0}, 1.9, material1);
    let sphere2 = Sphere::new(Vector3{x: -4.0, y: 1.0, z: 0.0}, 1.0, material2);
    let sphere3 = Sphere::new(Vector3{x: 4.0, y: 1.0, z: 0.0}, 1.0, material3);

    objects.push(sphere1);
    objects.push(sphere2);
    objects.push(sphere3);
}