
extern crate cgmath;

mod hitable;
mod hitable_list;
mod sphere;
mod ray_hit;
mod ray;
mod camera;
mod material;

use std::rc::Rc;
use std::fs::File;
use std::io::prelude::*;
use cgmath::Vector3;
use cgmath::prelude::*;
use rand::Rng;

use sphere::Sphere;
use hitable_list::HitableList;
use camera::Camera;
use material::*;

fn main() {
    let mut file = File::create("image.ppm").unwrap();

    let wx = 200;
    let wy = 100;
    let samples_per_pixel = 100;

    file.write(format!("P3\n{} {}\n255\n", wx, wy).as_bytes()).unwrap();

    let camera = Camera::new(
        Vector3::new(-2.0, 2.0, 1.0),
        Vector3::new(0.0, 0.0, -1.0),
        Vector3::new(0.0, 1.0, 0.0),
        45.0, 
        wx as f32 / wy as f32
    );

    let mat1 = Lambertian {
        albedo: Vector3::new(0.1, 0.2, 0.5),
    };

    let mat2 = Lambertian {
        albedo: Vector3::new(0.8, 0.8, 0.0),
    };

    let mat3 = Metal {
        albedo: Vector3::new(0.8, 0.6, 0.2),
        fuzz: 0.3,
    };

    let mat4 = Metal {
        albedo: Vector3::new(0.8, 0.6, 0.6),
        fuzz: 1.0,
    };

    let mat5 = Dielectric {
        refraction_index: 1.5,
    };

    let sphere1 = Sphere {
        center: Vector3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::new(mat1),
    };

    let sphere2 = Sphere {
        center: Vector3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Rc::new(mat2),
    };

    let sphere3 = Sphere {
        center: Vector3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::new(mat3),
    };

    let sphere4 = Sphere {
        center: Vector3::new(-1.0, 0.0, -1.0),
        radius: -0.45,
        material: Rc::new(mat5),
    };

    let sphere5 = Sphere {
        center: Vector3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::new(mat5),
    };

    let hitable_list = HitableList {
        list: vec![
            Box::new(sphere1), 
            Box::new(sphere2),
            Box::new(sphere3),
            Box::new(sphere4),
            Box::new(sphere5),
        ],
    };

    let mut rng = rand::thread_rng();

    for y in (0..wy).rev() {
        for x in 0..wx {
            let mut color = Vector3::new(0.0, 0.0, 0.0);

            for _sample in 0..samples_per_pixel {
                let u = (x as f32 + rng.gen::<f32>()) / wx as f32;
                let v = (y as f32 + rng.gen::<f32>()) / wy as f32;
                let ray = camera.get_ray(u, v);
                color += ray.color(&hitable_list, 0);
            }
            
            color /= samples_per_pixel as f32;

            let r = (255.99 * color.x.sqrt()) as u8;
            let g = (255.99 * color.y.sqrt()) as u8;
            let b = (255.99 * color.z.sqrt()) as u8;

            file.write(format!("{} {} {}\n", r, g, b).as_bytes()).unwrap();
        }
    }
}
