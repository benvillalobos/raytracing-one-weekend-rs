use crate::ray::*;
use cgmath::*;

pub struct Camera {
    origin: Vector3<f64>,
    lower_left_corner: Vector3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
}

impl Camera {
    pub fn new(vfov: f64, aspect_ratio: f64) -> Camera {
        let theta = deg_to_rad(vfov);
        let h = (theta/2.0).tan();
        let aspect_ratio = aspect_ratio;
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        Self {
            // Camera is at 0,0,0
            origin: Vector3::<f64>::new(0.0, 0.0, 0.0),
            // width of the screen
            horizontal: Vector3::<f64>::new(viewport_width, 0.0, 0.0),
            // height of the screen
            vertical: Vector3::<f64>::new(0.0, viewport_height, 0.0),
            // lower left corner of the viewport.
            // Remember that lower_left_corner is pushed out from origin.
            lower_left_corner: Vector3::<f64>::new(0.0, 0.0, 0.0) - Vector3::<f64>::new(viewport_width, 0.0, 0.0)/2.0 - Vector3::<f64>::new(0.0, viewport_height, 0.0)/2.0 - Vector3::<f64>::new(0.0, 0.0, focal_length),
        }
    }

    pub fn get_ray(self: &Self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
    }
}

fn deg_to_rad(degrees: f64) -> f64 {
    degrees * crate::PI / 180.0
}
