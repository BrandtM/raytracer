use crate::ray::Ray;
use crate::ray_hit::RayHit;

pub trait Hitable {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, hit: &mut RayHit) -> bool;
}