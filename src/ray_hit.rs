use std::rc::Rc;
use cgmath::Vector3;
use std::sync::{Arc, RwLock};
// use crate::material::Material;

#[derive(Clone)]
pub struct RayHit {
    pub t: f32,
    pub point: Vector3<f32>,
    pub normal: Vector3<f32>,
	// pub material: Arc<RwLock<Material + Send + Sync>>,
}

impl RayHit {
    pub fn new(t: f32, point: Vector3<f32>, normal: Vector3<f32>) -> RayHit {
        RayHit {
            t,
            point,
            normal
        }
    }

	// Create an empty RayHit. Reduces duplicate code
	pub fn init() -> RayHit {
		RayHit::new(
            0.0,
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 0.0),
        )
	}
}