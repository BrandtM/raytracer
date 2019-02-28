use cgmath::Vector3;
use cgmath::prelude::*;
use crate::ray::Ray;

pub struct Camera {
	pub origin: Vector3<f32>,
	pub bottom_left: Vector3<f32>,
	pub horizontal: Vector3<f32>,
	pub vertical: Vector3<f32>,
}

impl Camera {
	pub fn new(look_from: Vector3<f32>, look_at: Vector3<f32>, view_up: Vector3<f32>, vfov: f32, aspect: f32) -> Camera {
		let mut u = Vector3::new(0.0, 0.0, 0.0); // camera x
		let mut v = Vector3::new(0.0, 0.0, 0.0); // camera y
		let mut w = Vector3::new(0.0, 0.0, 0.0); // camera z

		let theta = vfov * (std::f32::consts::PI / 180.0);
		let half_height = (theta / 2.0).tan();
		let half_width = aspect * half_height;

		w = (look_from - look_at).normalize();
		u = view_up.cross(w).normalize();
		v = w.cross(u);

		let bottom_left = Vector3::new(-half_width, -half_height, -1.0);
		let bottom_left = look_from - half_width * u - half_height * v - w;
		let horizontal = 2.0 * half_width * u;
		let vertical = 2.0 * half_height * v;
		let origin = look_from;

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