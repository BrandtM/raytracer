use cgmath::Vector3;
use cgmath::prelude::*;
use crate::ray::Ray;

pub struct Camera {
	pub origin: Vector3<f32>,
	pub bottom_left: Vector3<f32>,
	pub horizontal: Vector3<f32>,
	pub vertical: Vector3<f32>,
	pub lens_radius: f32,
}

impl Camera {
	pub fn new(look_from: Vector3<f32>, look_at: Vector3<f32>, view_up: Vector3<f32>, vfov: f32, aspect: f32, aperture: f32, focus_distance: f32) -> Camera {
		let mut u = Vector3::new(0.0, 0.0, 0.0); // camera x
		let mut v = Vector3::new(0.0, 0.0, 0.0); // camera y
		let mut w = Vector3::new(0.0, 0.0, 0.0); // camera z

		let theta = vfov * (std::f32::consts::PI / 180.0);
		let half_height = (theta / 2.0).tan();
		let half_width = aspect * half_height;

		w = (look_from - look_at).normalize();
		u = view_up.cross(w).normalize();
		v = w.cross(u);

		let bottom_left = look_from - half_width * focus_distance * u - half_height * focus_distance * v - focus_distance * w;
		let horizontal = 2.0 * half_width * focus_distance * u;
		let vertical = 2.0 * half_height * focus_distance * v;
		let origin = look_from;

		Camera {
			origin,
			bottom_left,
			horizontal,
			vertical,
			lens_radius: aperture / 2.0,
		}
	}

	pub fn get_ray(&self, u: f32, v: f32) -> Ray {
		let rd = self.lens_radius * Ray::random_in_unit_disk();
		let offset = u * rd.x + v * rd.y;
		let offset_vec = Vector3::new(offset, offset, offset);

		Ray::new(self.origin + offset_vec, self.bottom_left + u * self.horizontal + v * self.vertical - self.origin - offset_vec)
	}
}