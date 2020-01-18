use crate::hitable::Hitable;
use crate::ray::Ray;
use crate::ray_hit::RayHit;
use crate::triangle::*;

#[derive(Clone)]
pub struct HitableList {
    pub list: Vec<Box<Triangle>>,
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<RayHit> {
        let mut tmp_hit = None;
        let mut closest = t_max;

        self.list
            .iter()
            .for_each(|hitable| match hitable.hit(ray, t_min, closest) {
                Some(hit) => {
                    closest = hit.t;
                    tmp_hit = Some(hit);
                }
                _ => (),
            });

        tmp_hit
    }
}
