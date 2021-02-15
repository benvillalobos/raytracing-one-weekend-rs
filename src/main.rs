// Following: https://raytracing.github.io/books/RayTracingInOneWeekend.html
use cgmath::*;
use crate::ray;

fn main() {
    let img_width = 800;
    let img_height = 600;
    
    output_ppm(img_width, img_height);
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

fn write_color(color: Vector3<f64>) {
    println!("{} {} {}", (255.999 * color.x) as i32, (255.999 * color.y) as i32, (255.999 * color.z) as i32);
}