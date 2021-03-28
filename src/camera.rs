use crate::ray::*;
use cgmath::*;

pub struct Camera {
    origin: Vector3<f64>,
    lower_left_corner: Vector3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
    lens_radius: f64,
    u: Vector3<f64>,
    v: Vector3<f64>,
    w: Vector3<f64>,
}

impl Camera {
    pub fn new( look_from: Vector3<f64>, 
                look_at: Vector3<f64>, 
                v_up: Vector3<f64>, 
                vfov: f64, 
                aspect_ratio: f64,
                aperture: f64,
                focus_dist: f64) -> Camera {

        let theta = crate::deg_to_rad(vfov);
        let h = (theta/2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        // w is our camera's Z axis.
        let w = (look_from - look_at).normalize();
        // u is our camera's X axis.
        let u = v_up.cross(w).normalize();
        // v is our camera's Y axis.
        let v = w.cross(u);

        Self {
            // Camera is at 0,0,0
            origin: look_from,
            // width of the screen
            horizontal: focus_dist * viewport_width * u,
            // height of the screen
            vertical: focus_dist * viewport_height * v,
            // lower left corner of the viewport.
            // Remember that lower_left_corner is pushed out from origin.
            lower_left_corner: look_from - (focus_dist * viewport_width * u)/2.0 - (focus_dist * viewport_height * v)/2.0 - focus_dist*w,
            
            lens_radius: aperture/2.0,
            u: u,
            v: v,
            w: w,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * crate::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(self.origin + offset, self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset)
    }
}