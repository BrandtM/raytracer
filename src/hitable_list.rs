use crate::hitable::Hitable;
// use crate::material::*;
use crate::ray2::Ray2;
use crate::ray_hit::RayHit;
use cgmath::Vector3;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct HitableList {
    pub list: Vec<Box<dyn Hitable>>,
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray2, t_min: f32, t_max: f32) -> Option<RayHit> {
        let mut tmp_hit = None;
        let mut hit_anything = false;
        let mut closest = t_max;

		self.list.iter().for_each(|hitable| {
			match hitable.hit(ray, t_min, closest) {
				Some(hit) => {
					closest = hit.t;
					tmp_hit = Some(hit);
				},
				_ => ()
			}
		});

        tmp_hit
    }
}
