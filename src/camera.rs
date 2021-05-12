use crate::vector_3::Vector3;
use crate::ray::Ray;

pub struct Camera {
    origin: Vector3,
    left_lower_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, focal_length: f64, origin: Vector3) -> Self {
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;

        let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
        let vertical = Vector3::new(0.0, viewport_height, 0.0);

        Camera {
            origin,
            horizontal,
            vertical,
            left_lower_corner: origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, focal_length),

        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.left_lower_corner + u * self.horizontal + v * self.vertical - self.origin)
    }
}