use crate::hitable::Hitable;
use crate::material::*;
use crate::ray2::Ray2;
use crate::ray_hit::RayHit;
use cgmath::prelude::*;
use cgmath::Vector3;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray2, t_min: f32, t_max: f32) -> Option<RayHit> {
        let origin_center = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = origin_center.dot(ray.direction);
        let c = origin_center.dot(origin_center) - self.radius.powi(2);
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut tmp = (-b - discriminant.sqrt()) / a;

            if tmp < t_max && tmp > t_min {
				let hitpoint = ray.point_at(tmp);
				let mut hit = RayHit::new(tmp, hitpoint, (hitpoint - self.center) / self.radius, None);
				let material_hit = self.material.scatter(ray, &hit);
				hit.material = Some(material_hit);
				return Some(hit);
            }

            tmp = (-b + discriminant.sqrt()) / a;

            if tmp < t_max && tmp > t_min {
				let hitpoint = ray.point_at(tmp);
				let mut hit = RayHit::new(tmp, hitpoint, (hitpoint - self.center) / self.radius, None);
				let material_hit = self.material.scatter(ray, &hit);
				hit.material = Some(material_hit);
				return Some(hit);
            }
        }

        None
    }
}
