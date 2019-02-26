use cgmath::prelude::*;
use cgmath::Vector3;
use crate::ray::Ray;
use crate::ray_hit::RayHit;

pub trait Material {
	fn scatter(&self, ray: Ray, hit: &RayHit, attenuation: &mut Vector3<f32>, scattered: &mut Ray) -> bool;
}

#[derive(Copy, Clone)]
pub struct Lambertian {
	pub albedo: Vector3<f32>,
}

#[derive(Copy, Clone)]
pub struct Metal {
	pub albedo: Vector3<f32>,
}

#[derive(Copy, Clone)]
pub struct EmptyMaterial { }

impl Material for EmptyMaterial {
	fn scatter(&self, _ray: Ray, hit: &RayHit, _attenuation: &mut Vector3<f32>, scattered: &mut Ray) -> bool {
		*scattered = Ray {
			origin: hit.point,
			direction: hit.normal
		};

		true
	}
}

impl Material for Lambertian {
	fn scatter(&self, _ray: Ray, hit: &RayHit, attenuation: &mut Vector3<f32>, scattered: &mut Ray) -> bool {
		let target = hit.point + hit.normal + Ray::random_in_unit_sphere();
		*scattered = Ray {
			origin: hit.point,
			direction: target - hit.point,
		};

		*attenuation = self.albedo;
		true
	}
}

impl Material for Metal {
	fn scatter(&self, ray: Ray, hit: &RayHit, attenuation: &mut Vector3<f32>, scattered: &mut Ray) -> bool {
		let mut reflection = ray.direction.normalize();
		reflection = reflection - 2.0 * reflection.dot(hit.normal) * hit.normal;
		*scattered = Ray {
			origin: hit.point,
			direction: reflection,
		};
		
		*attenuation = self.albedo;
		
		scattered.direction.dot(hit.normal) > 0.0
	}
}