use crate::shapes::{RayCastShape, HitRecord};
use crate::ray::Ray;

pub struct World {
    shapes: Vec<Box<dyn RayCastShape>>,
}

impl World {
    pub fn new() -> Self {
        World {
            shapes: vec![]
        }
    }

    pub fn add(&mut self, shape: Box<dyn RayCastShape>) {
        self.shapes.push(shape);
    }

    pub fn clear(&mut self) {
        self.shapes.clear();
    }

    pub fn hit_anything(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything = false;
        let mut closest = t_max;
        let mut result: Option<HitRecord> = None;
        for shape in &self.shapes {
            let record = shape.as_ref().hit(ray, t_min, closest);

            if record.is_none() {
                continue;
            }

            let record = record.unwrap();
            hit_anything = true;
            closest = record.t;
            result = Some(record);
        }

        return if hit_anything == false {
            None
        } else {
            result
        };
    }
}