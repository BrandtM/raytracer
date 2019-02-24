use cgmath::Vector3;
use crate::hitable::Hitable;
use crate::ray::Ray;
use crate::ray_hit::RayHit;

pub struct HitableList {
    pub list: Vec<Box<Hitable>>,
}

impl Hitable for HitableList {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, hit: &mut RayHit) -> bool {
        let mut tmp_hit = RayHit::new(0.0, Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));
        let mut hit_anything = false;
        let mut closest = t_max;

        for hitable in self.list.iter() {
            if hitable.hit(ray, t_min, closest, &mut tmp_hit) {
                hit_anything = true;
                closest = tmp_hit.t;
                *hit = tmp_hit;
            }
        }

        hit_anything
    }
}