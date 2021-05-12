use crate::ray::Ray;
use crate::vector_3::Vector3;

pub trait RayCastShape {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    _center: Vector3,
    _radius: f64,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct HitRecord {
    pub point: Vector3,
    pub normal: Vector3,
    pub front_face: bool,
    pub t: f64,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64) -> Self {
        Sphere {
            _center: center,
            _radius: radius,
        }
    }
    pub fn center(&self) -> Vector3 {
        self._center
    }
    pub fn radius(&self) -> f64 {
        self._radius
    }
}

impl RayCastShape for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center();
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self._radius * self._radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let point = ray.at(root);
        let normal = (point - self._center) / self._radius;
        let front_face = ray.direction().dot(normal) < 0.0;
        Some(HitRecord {
            point,
            front_face,
            normal: if front_face { normal } else { normal * -1.0 },
            t: root,
        })
    }
}

