use crate::ray::Ray;
use crate::ray_hit::RayHit;
use cgmath::prelude::*;
use cgmath::Vector3;
use rand::Rng;

pub trait Material: MaterialClone + Sync {
    fn scatter(&self, ray: &Ray, hit: &RayHit) -> MaterialHitInfo;
}

pub trait MaterialClone {
    fn clone_box(&self) -> Box<dyn Material>;
}

impl<T> MaterialClone for T
where
    T: 'static + Material + Clone,
{
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.clone_box()
    }
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    pub albedo: Vector3<f32>,
}

#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: Vector3<f32>,
    pub fuzz: f32,
}

#[derive(Copy, Clone)]
pub struct Dielectric {
    pub refraction_index: f32,
}

#[derive(Copy, Clone)]
pub struct MaterialHitInfo {
    pub attenuation: Vector3<f32>,
    pub scattered: Ray,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &RayHit) -> MaterialHitInfo {
        let target = hit.point + hit.normal + Ray::random_in_unit_sphere();
        let scattered = Ray {
            origin: hit.point,
            direction: target - hit.point,
        };

        let attenuation = self.albedo;

        MaterialHitInfo {
            attenuation,
            scattered,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &RayHit) -> MaterialHitInfo {
        let mut reflection = Ray::reflect(ray.direction, hit.normal);
        reflection += self.fuzz.min(1.0) * Ray::random_in_unit_sphere();

        let scattered = Ray {
            origin: hit.point,
            direction: reflection,
        };

        let attenuation = self.albedo;

        MaterialHitInfo {
            attenuation,
            scattered,
        }
    }
}

impl Material for Dielectric {
    #[allow(unused_assignments)]
    fn scatter(&self, ray: &Ray, hit: &RayHit) -> MaterialHitInfo {
        let mut rng = rand::thread_rng();
        let reflected = Ray::reflect(ray.direction, hit.normal);
        let mut refraction_index = self.refraction_index;
        let attenuation = Vector3::new(1.0, 1.0, 1.0);
        let mut scattered: Ray;

        let mut outward_normal = Vector3::new(0.0, 0.0, 0.0);
        let mut refracted = Vector3::new(0.0, 0.0, 0.0);
        let mut reflection_prob = 0.0_f32;
        let mut cos = 0.0_f32;

        if ray.direction.dot(hit.normal) > 0.0 {
            outward_normal = -hit.normal;
            cos = refraction_index * ray.direction.dot(hit.normal) / ray.direction.magnitude();
        } else {
            outward_normal = hit.normal;
            refraction_index = 1.0 / refraction_index;
            cos = -ray.direction.dot(hit.normal) / ray.direction.magnitude();
        }

        if Ray::refract(
            ray.direction,
            outward_normal,
            refraction_index,
            &mut refracted,
        ) {
            reflection_prob = Ray::schlick(cos, refraction_index);
        } else {
            scattered = Ray {
                origin: hit.point,
                direction: reflected,
            };

            reflection_prob = 1.0;
        }

        if rng.gen::<f32>() < reflection_prob {
            scattered = Ray {
                origin: hit.point,
                direction: reflected,
            };
        } else {
            scattered = Ray {
                origin: hit.point,
                direction: refracted,
            };
        }

        MaterialHitInfo {
            attenuation,
            scattered,
        }
    }
}
