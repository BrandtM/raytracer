use crate::ray2::Ray2;
use crate::ray_hit::RayHit;

pub trait Hitable: HitableClone + Sync {
    fn hit(&self, ray: &Ray2, t_min: f32, t_max: f32) -> Option<RayHit>;
}

pub trait HitableClone {
	fn clone_box(&self) -> Box<dyn Hitable>;
}

impl<T> HitableClone for T where T: 'static + Hitable + Clone {
	fn clone_box(&self) -> Box<dyn Hitable> {
		Box::new(self.clone())
	}
}

impl Clone for Box<dyn Hitable> {
	fn clone(&self) -> Box<dyn Hitable> {
		self.clone_box()
	}
}