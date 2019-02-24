use cgmath::prelude::*;
use cgmath::Vector3;
use rand::Rng;
use crate::hitable::Hitable;
use crate::ray_hit::RayHit;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Ray {
        Ray {
            origin,
            direction,
        }
    }

    pub fn point_at(&self, t: f32) -> Vector3<f32> {
        return self.origin + t * self.direction;
    }

    pub fn color(&self, world: &Hitable) -> Vector3<f32> {
        let mut hit = RayHit::new(0.0, Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));
        
        if world.hit(*self, 0.001, std::f32::MAX, &mut hit) {
            let target = hit.point + hit.normal + Ray::random_in_unit_sphere();
            let new_ray = Ray {
                origin: hit.point,
                direction: target - hit.point
            };

            return 0.5 * Ray::cast(new_ray, world);
        }

        let t = 0.5 * self.direction.normalize().y + 1.0;
        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }

    pub fn cast(ray: Ray, world: &Hitable) -> Vector3<f32> {
        ray.color(world)
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
}