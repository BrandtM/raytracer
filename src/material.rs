use cgmath::prelude::*;
use cgmath::Vector3;
use rand::Rng;
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
	pub fuzz: f32,
}

#[derive(Copy, Clone)]
pub struct Dielectric {
	pub refraction_index: f32,
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
		let mut reflection = Ray::reflect(ray.direction, hit.normal);
		reflection += self.fuzz * Ray::random_in_unit_sphere();

		*scattered = Ray {
			origin: hit.point,
			direction: reflection,
		};
		
		*attenuation = self.albedo;
		
		scattered.direction.dot(hit.normal) > 0.0
	}
}

impl Material for Dielectric {
	#[allow(unused_assignments)]
	fn scatter(&self, ray: Ray, hit: &RayHit, attenuation: &mut Vector3<f32>, scattered: &mut Ray) -> bool {
		let mut rng = rand::thread_rng();
		let reflected = Ray::reflect(ray.direction, hit.normal);
		let mut refraction_index = self.refraction_index;
		*attenuation = Vector3::new(1.0, 1.0, 1.0);

		let mut outward_normal = Vector3::new(0.0, 0.0, 0.0);
		let mut refracted = Vector3::new(0.0, 0.0, 0.0);
		let mut reflection_prob = 0.0_f32;
		let mut cos = 0.0_f32;

		if ray.direction.dot(hit.normal) > 0.0 {
			outward_normal = -hit.normal;
			cos = refraction_index * ray.direction.dot(hit.normal) / ray.direction.magnitude();
		} else {
			outward_normal = hit.normal;
			refraction_index = 1.0 / refraction_index;
			cos = -ray.direction.dot(hit.normal) / ray.direction.magnitude();
		}

		if Ray::refract(ray.direction, outward_normal, refraction_index, &mut refracted) {
			reflection_prob = Ray::schlick(cos, refraction_index);
		} else {
			*scattered = Ray {
				origin: hit.point,
				direction: reflected,
			};

			reflection_prob = 1.0;
		}

		if rng.gen::<f32>() < reflection_prob {
			*scattered = Ray {
				origin: hit.point,
				direction: reflected,
			}
		} else {
			*scattered = Ray {
				origin: hit.point,
				direction: refracted,
			}
		}

		true
	}
}