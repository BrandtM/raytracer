use crate::hitable::Hitable;
use crate::material::*;
use crate::ray::Ray;
use crate::ray_hit::RayHit;
use cgmath::prelude::*;
use cgmath::Vector3;

#[derive(Clone)]
pub struct Plane {
    pub center: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub width: Vector3<f32>,
    pub height: Vector3<f32>,
    pub material: Box<dyn Material>,
}

impl Hitable for Plane {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<RayHit> {
        let plane_ray = self.center - ray.origin;
        let plane_ray = plane_ray.dot(self.normal);
        let plane_ray_direction = ray.direction.dot(self.normal);
        let t = plane_ray / plane_ray_direction;

        if t > t_min && t < t_max {
            let hitpoint = ray.origin + t * ray.direction;
            let v = hitpoint - self.center;
            let width = self.width.magnitude();
            let height = self.height.magnitude();
            let proj1 = v.dot(self.width) / width;
            let proj2 = v.dot(self.height) / height;

            if (proj1 < width && proj1 > 0.0) && (proj2 < height && proj2 > 0.0) {
                let mut hit = RayHit::new(t, hitpoint, self.normal, None);
                let material_hit = self.material.scatter(ray, &hit);
                hit.material = Some(material_hit);
                return Some(hit);
            }
        }

        None
    }
}
