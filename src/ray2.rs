use cgmath::prelude::*;
use cgmath::Vector3;
use rand::Rng;
use crate::sphere::*;
use crate::hitable_list::*;
use crate::hitable::*;

#[derive(Copy, Clone)]
pub struct Ray2 {
	pub origin: Vector3<f32>,
	pub direction: Vector3<f32>
}

impl Ray2 {
	pub fn new(orig: Vector3<f32>, dir: Vector3<f32>) -> Ray2 {
		Ray2 {
			origin: orig,
			direction: dir
		}
	}

	pub fn point_at(&self, t: f32) -> Vector3<f32> {
		self.origin + t * self.direction
	}

	pub fn color(&self, hitable_list: HitableList, depth: u32) -> Vector3<f32> {
		let color: Vector3<f32>;

		match hitable_list.hit(&self, 0.001, std::f32::MAX) {
			Some(hit) => {
				if depth < 50 {
					let material_info = hit.material.unwrap();
					color = material_info.attenuation.mul_element_wise(material_info.scattered.color(hitable_list, depth + 1));
				} else {
					color = Vector3::new(0.0, 0.0, 0.0);
				}
			},
			None => {
				let normalized = self.direction.normalize();
				let t = 0.5 * (normalized.y + 1.0);
				let white = Vector3::new(1.0, 1.0, 1.0);
				let blue = Vector3::new(0.5, 0.7, 1.0);

				color = (1.0 - t) * white + t * blue;
			}
		}

		color
	}

	#[allow(unused_assignments)]
    pub fn random_in_unit_sphere() -> Vector3<f32> {
        let mut random_vec = Vector3::new(0.0, 0.0, 0.0);
        let mut rng = rand::thread_rng();

        loop {
            let x = rng.gen::<f32>();
            let y = rng.gen::<f32>();
            let z = rng.gen::<f32>();
            let unit = Vector3::new(1.0, 1.0, 1.0);
            random_vec = 2.0 * Vector3::new(x, y, z) - unit;

            if random_vec.magnitude2() < 1.0 {
                break;
            }
        }

        random_vec
    }

    #[allow(unused_assignments)]
    pub fn random_in_unit_disk() -> Vector3<f32> {
        let mut random_vec = Vector3::new(0.0, 0.0, 0.0);
        let mut rng = rand::thread_rng();

        loop {
            let x = rng.gen::<f32>();
            let y = rng.gen::<f32>();
            let unit = Vector3::new(1.0, 1.0, 0.0);
            random_vec = 2.0 * Vector3::new(x, y, 0.0) - unit;

            if random_vec.magnitude() < 1.0 {
                break;
            }
        }

        random_vec
    }
}