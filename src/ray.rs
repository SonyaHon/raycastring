use crate::vector_3::Vector3;

#[derive(Debug, PartialEq)]
pub struct Ray {
    _origin: Vector3,
    _direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Ray {
            _origin: origin,
            _direction: direction,
        }
    }

    pub fn origin(&self) -> Vector3 {
        self._origin
    }

    pub fn direction(&self) -> Vector3 {
        self._direction
    }

    pub fn at(&self, t: f64) -> Vector3 {
        self._origin + self._direction * t
    }

    pub fn to_color_vector(&self) -> Vector3 {
        let unit_direction = Vector3::unit_vector(self._direction);
        let t = 0.5 * (unit_direction.y() + 1.0);
        Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0) * t
    }
}