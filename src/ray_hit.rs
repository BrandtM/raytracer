use cgmath::Vector3;

#[derive(Copy, Clone)]
pub struct RayHit {
    pub t: f32,
    pub point: Vector3<f32>,
    pub normal: Vector3<f32>,
}

impl RayHit {
    pub fn new(t: f32, point: Vector3<f32>, normal: Vector3<f32>) -> RayHit {
        RayHit {
            t,
            point,
            normal
        }
    }
}