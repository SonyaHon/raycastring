pub mod vector_3;
pub mod ray;
pub mod image_utils;
mod shapes;
mod world;
mod camera;
mod widget_test;

use crate::image_utils::{gen_image, GenImageCallback};
use crate::vector_3::Vector3;
use crate::ray::Ray;
use crate::shapes::{RayCastShape, Sphere};
use crate::world::World;
use crate::camera::Camera;

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
    camera: Camera,
    world: World,
}

impl GenImageCallback for RayCast {
    fn get_color(&self, u: f64, v: f64) -> Vector3 {
        let ray = self.camera.get_ray(u, v);

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

    let mut camera = Camera::new(aspect_ratio, 1.0, Vector3::zero());

    let mut world = World::new();
    world.add(Box::from(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::from(Sphere::new(Vector3::new(0.0, -50.0, 0.0), 49.0)));

    gen_image(img_width, img_height, "ray_cast.ppm", RayCast {
        camera,
        world,
    });
}
