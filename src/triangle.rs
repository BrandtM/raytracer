use crate::hitable::Hitable;
use crate::material::*;
use crate::ray::Ray;
use crate::ray_hit::RayHit;
use cgmath::prelude::*;
use cgmath::Vector3;

#[derive(Clone)]
pub struct Triangle {
    pub vertices: [Vector3<f32>; 3],
    pub normal: Vector3<f32>,
    pub material: Box<dyn Material>,
}

impl Hitable for Triangle {
    /// https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
    fn hit(&self, ray: &Ray, t_min: f32, _t_max: f32) -> Option<RayHit> {
        let (v0, v1, v2) = (self.vertices[0], self.vertices[1], self.vertices[2]);
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let h = ray.direction.cross(edge2);
        let a = edge1.dot(h);
        let normal = -edge1.cross(edge2).normalize();

        if a < t_min {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin - v0;
        let u = f * s.dot(h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(edge1);
        let v = f * ray.direction.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(q);

        if t > t_min && t < 1.0 {
            let hitpoint = ray.origin + ray.direction * t;
            let mut hit = RayHit::new(t, hitpoint, normal, None);
            let material_hit = self.material.scatter(ray, &hit);
            hit.material = Some(material_hit);
            return Some(hit);
        }

        None
    }
}
