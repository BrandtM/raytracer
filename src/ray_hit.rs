use crate::material::*;
use cgmath::Vector3;

#[derive(Clone)]
pub struct RayHit {
    pub t: f32,
    pub point: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub material: Option<MaterialHitInfo>,
}

impl RayHit {
    pub fn new(
        t: f32,
        point: Vector3<f32>,
        normal: Vector3<f32>,
        material: Option<MaterialHitInfo>,
    ) -> RayHit {
        RayHit {
            t,
            point,
            normal,
            material,
        }
    }

    pub fn init() -> RayHit {
        RayHit::new(
            0.0,
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 0.0),
            None,
        )
    }
}
