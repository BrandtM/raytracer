use cgmath::prelude::*;
use cgmath::Vector3;
use crate::hitable::Hitable;
use crate::ray_hit::RayHit;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn point_at(&self, t: f32) -> Vector3<f32> {
        return self.origin + t * self.direction;
    }

    pub fn color(&self, world: &Hitable) -> Vector3<f32> {
        let mut hit = RayHit::new(0.0, Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));
        
        if world.hit(*self, 0.0, std::f32::MAX, &mut hit) {
            return 0.5 * Vector3::new(hit.normal.x + 1.0, hit.normal.y + 1.0, hit.normal.z + 1.0);
        }

        let t = 0.5 * self.direction.normalize().y + 1.0;
        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }
}