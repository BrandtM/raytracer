use crate::hitable::Hitable;
use crate::material::*;
use crate::ray::Ray;
use crate::ray_hit::RayHit;
use cgmath::prelude::*;
use cgmath::Vector3;
use std::rc::Rc;

pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub material: Rc<Material>,
}

impl Hitable for Sphere {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, hit: &mut RayHit) -> bool {
        let origin_center = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = origin_center.dot(ray.direction);
        let c = origin_center.dot(origin_center) - self.radius.powi(2);
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut tmp = (-b - discriminant.sqrt()) / a;

            if tmp < t_max && tmp > t_min {
                hit.t = tmp;
                hit.point = ray.point_at(tmp);
                hit.normal = (hit.point - self.center) / self.radius;
                hit.material = self.material.clone();
                return true;
            }

            tmp = (-b + discriminant.sqrt()) / a;

            if tmp < t_max && tmp > t_min {
                hit.t = tmp;
                hit.point = ray.point_at(tmp);
                hit.normal = (hit.point - self.center) / self.radius;
                hit.material = self.material.clone();
                return true;
            }
        }

        false
    }
}
