// Following: https://raytracing.github.io/books/RayTracingInOneWeekend.html
//#![allow(unused_variables)]
#![allow(dead_code)]

use cgmath::*;
use raytracing::ray::*;

// cargo run > img.ppm
fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 400;
    let img_height = (img_width as f64/aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vector3::<f64>::new(0.0, 0.0, 0.0);
    let horizontal = Vector3::<f64>::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::<f64>::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vector3::<f64>::new(0.0, 0.0, focal_length);
    
    println!("P3\n{} {}\n255", img_width, img_height);

    for y in (0..img_height).rev() {
    eprintln!("Scalines remaining: {}", y);
        for x in 0..img_width {
            let u: f64 = x as f64/(img_width-1) as f64;
            let v: f64 = y as f64/(img_height-1) as f64;

            // Remember that lower_left_corner is pushed out from origin.
            let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);
            let pixel = ray_color(r);

            write_color(pixel);
        }
    }
    eprintln!("Done");
}

fn write_color(color: Vector3<f64>) {
    println!("{} {} {}", (255.999 * color.x) as i32, (255.999 * color.y) as i32, (255.999 * color.z) as i32);
}

fn ray_color(ray: Ray) -> Vector3<f64>{
    let t = hit_sphere(Vector3::<f64>::new(0.0, 0.0, -1.0), 0.5, &ray);

    // hit
    if t > 0.0 {
        let n = ray.at(t) - Vector3::<f64>::new(0.0, 0.0, -1.0);
        return 0.5*Vector3::<f64>::new(n.x+1.0, n.y+1.0, n.z+1.0);
    }
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
    return (1.0-t)*Vector3::new(1.0, 1.0, 1.0) + t*Vector3::new(0.5, 0.7, 1.0);
}

fn hit_sphere(center: Vector3<f64>, radius: f64, ray: &Ray) -> f64 {
    let oc: Vector3<f64> = ray.origin - center;

    let a = ray.dir.dot(ray.dir);
    let b = 2.0 * oc.dot(ray.dir);
    let c = oc.dot(oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    if discriminant < 0.0 {
        -1.0
    } else {
        return (-b - discriminant).sqrt() / (2.0 * a);
    }
}

fn output_ppm(width: i32, height: i32) {
    println!("P3\n{} {}\n255", width, height);

    for y in (0..height).rev() {
    eprintln!("Scalines remaining: {}", y);
        for x in 0..width {
            let r: f64 = x as f64/(width-1) as f64;
            let g: f64 = y as f64/(height-1) as f64;
            let b: f64 = 0.25;

            let color: Vector3<f64> = Vector3::<f64>::new(r, g, b);

            write_color(color);
        }
    }
    eprintln!("Done");
}