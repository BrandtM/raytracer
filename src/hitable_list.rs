use crate::hitable::Hitable;
use crate::material::*;
use crate::ray::Ray;
use crate::ray_hit::RayHit;
use cgmath::Vector3;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct HitableList {
    pub list: Vec<Arc<RwLock<Hitable + Send + Sync>>>,
}

impl Hitable for HitableList {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, hit: &mut RayHit) -> bool {
        let mut tmp_hit = RayHit::new(
            0.0,
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 0.0),
            Arc::new(RwLock::new(EmptyMaterial {})),
        );
        let mut hit_anything = false;
        let mut closest = t_max;

        for hitable in self.list.iter() {
            if hitable.read().unwrap().hit(ray, t_min, closest, &mut tmp_hit) {
                let tmp = tmp_hit.clone();

                hit_anything = true;
                closest = tmp.t;
                *hit = tmp;
            }
        }

        hit_anything
    }
}
