pub mod vector_3;
pub mod ray;
pub mod image_utils;
mod shapes;
mod world;

use crate::image_utils::{gen_image, GenImageCallback};
use crate::vector_3::Vector3;
use crate::ray::Ray;
use crate::shapes::{RayCastShape, Sphere};
use crate::world::World;

struct Gradient;

impl GenImageCallback for Gradient {
    fn get_color(&self, u: f64, v: f64) -> Vector3 {
        Vector3::new(u, v, 0.25)
    }
}

struct RayGradient {
    origin: Vector3,
    lower_left_corner: Vector3,
    vertical: Vector3,
    horizontal: Vector3,
}

impl GenImageCallback for RayGradient {
    fn get_color(&self, u: f64, v: f64) -> Vector3 {
        let ray = Ray::new(self.origin, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin);
        ray.to_color_vector()
    }
}

struct RayCast {
    origin: Vector3,
    lower_left_corner: Vector3,
    vertical: Vector3,
    horizontal: Vector3,
    world: World,
}

impl GenImageCallback for RayCast {
    fn get_color(&self, u: f64, v: f64) -> Vector3 {
        let ray = Ray::new(self.origin, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin);

        let hit_record = self.world.hit_anything(&ray, 0.0, std::f64::INFINITY);
        if hit_record.is_some() {
            let hit_record = hit_record.unwrap();

            return (hit_record.normal + Vector3::new(1.0, 1.0, 1.0)) * 0.5;
        }

        ray.to_color_vector()
    }
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 600;
    let img_height: i32 = (img_width as f64 / aspect_ratio) as i32;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;

    let focal_length = 1.0;

    let origin = Vector3::zero();
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, focal_length);

    let mut world = World::new();
    world.add(Box::from(Sphere::new(Vector3::new(0.0, 0.5, -1.0), 0.3)));
    world.add(Box::from(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::from(Sphere::new(Vector3::new(0.0, -50.0, 0.0), 49.0)));

    gen_image(img_width, img_height, "ray_cast.ppm", RayCast {
        origin,
        lower_left_corner,
        vertical,
        horizontal,
        world,
    });
}
