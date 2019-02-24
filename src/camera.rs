use cgmath::Vector3;
use crate::ray::Ray;

pub struct Camera {
	pub origin: Vector3<f32>,
	pub bottom_left: Vector3<f32>,
	pub horizontal: Vector3<f32>,
	pub vertical: Vector3<f32>,
}

impl Camera {
	pub fn new(origin: Vector3<f32>, bottom_left: Vector3<f32>, horizontal: Vector3<f32>, vertical: Vector3<f32>) -> Camera {
		Camera {
			origin,
			bottom_left,
			horizontal,
			vertical,
		}
	}

	pub fn get_ray(&self, u: f32, v: f32) -> Ray {
		Ray::new(self.origin, self.bottom_left + u * self.horizontal + v * self.vertical - self.origin)
	}
}