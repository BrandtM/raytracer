use std::rc::Rc;
use cgmath::Vector3;
use std::sync::{Arc, RwLock};
use crate::material::Material;

#[derive(Clone)]
pub struct RayHit {
    pub t: f32,
    pub point: Vector3<f32>,
    pub normal: Vector3<f32>,
	pub material: Arc<RwLock<Material + Send + Sync>>,
}

impl RayHit {
    pub fn new(t: f32, point: Vector3<f32>, normal: Vector3<f32>, material: Arc<RwLock<Material + Send + Sync>>) -> RayHit {
        RayHit {
            t,
            point,
            normal,
			material,
        }
    }
}