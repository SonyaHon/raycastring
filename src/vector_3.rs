use std::ops;
use rand::Rng;
use std::ops::RangeTo;
use rand::distributions::uniform::SampleRange;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector3 {
    data: [f64; 3],
}

impl Vector3 {
    pub fn zero() -> Self {
        Vector3 {
            data: [0.0, 0.0, 0.0]
        }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 {
            data: [x, y, z]
        }
    }

    pub fn unit_vector(value: Vector3) -> Vector3 {
        value / value.length()
    }

    pub fn gen_random() -> Self {
        let mut rng = rand::thread_rng();
        Vector3 {
            data: [
                rng.gen(),
                rng.gen(),
                rng.gen(),
            ]
        }
    }

    pub fn get_random_in_range(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        Vector3 {
            data: [
                rng.gen_range(min..max),
                rng.gen_range(min..max),
                rng.gen_range(min..max),
            ]
        }
    }

    pub fn gen_random_point_in_unit_sphere() -> Self {
        loop {
            let point = Self::get_random_in_range(-1.0, 1.0);
            if point.length_squared() < 1.0 {
                return point;
            }
        }
    }

    pub fn x(&self) -> f64 {
        self.data[0]
    }

    pub fn y(&self) -> f64 {
        self.data[1]
    }

    pub fn z(&self) -> f64 {
        self.data[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.data[0] * self.data[0] + self.data[1] * self.data[1] + self.data[2] * self.data[2]
    }

    pub fn dot(&self, other: Vector3) -> f64 {
        self.data[0] * other.data[0] + self.data[1] * other.data[1] + self.data[2] * other.data[2]
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            data: [
                self.data[0] + rhs.data[0],
                self.data[1] + rhs.data[1],
                self.data[2] + rhs.data[2]
            ]
        }
    }
}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        self + (rhs * -1.0)
    }
}

impl ops::Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3 {
            data: [
                self.data[0] * rhs,
                self.data[1] * rhs,
                self.data[2] * rhs
            ]
        }
    }
}

impl ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            data: [
                rhs.data[0] * self,
                rhs.data[1] * self,
                rhs.data[2] * self
            ]
        }
    }
}

impl ops::Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Vector3 {
    pub fn to_color_str(&self) -> String {
        format!("{} {} {}", (255.999 * self.data[0]) as i32, (255.999 * self.data[1]) as i32, (255.999 * self.data[2]) as i32)
    }

    pub fn to_color_str_with_newline(&self) -> String {
        self.to_color_str() + "\n"
    }

    pub fn to_color_sampled(&self, samples_per_pixel: i32) -> String {
        let scale = 1.0 / (samples_per_pixel as f64);
        let new_color = *self * scale;

        new_color.to_color_str()
    }

    pub fn to_color_sampled_with_newline(&self, samples_per_pixel: i32) -> String {
        self.to_color_sampled(samples_per_pixel) + "\n"
    }
}