use crate::ray::*;
use cgmath::*;

pub struct Camera {
    origin: Vector3<f64>,
    lower_left_corner: Vector3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
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
            lower_left_corner: Vector3::<f64>::new(0.0, 0.0, 0.0) - Vector3::<f64>::new(viewport_width, 0.0, 0.0)/2.0 - Vector3::<f64>::new(0.0, viewport_height, 0.0)/2.0 - Vector3::<f64>::new(0.0, 0.0, focal_length),
        }
    }

    pub fn get_ray(self: &Self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
    }
}