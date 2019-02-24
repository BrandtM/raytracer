
extern crate cgmath;

pub mod hitable;
pub mod hitable_list;
pub mod sphere;
pub mod ray_hit;
pub mod ray;
pub mod camera;

use std::fs::File;
use std::io::prelude::*;
use cgmath::Vector3;
use rand::Rng;

use sphere::Sphere;
use hitable_list::HitableList;
use camera::Camera;

fn main() {
    let mut file = File::create("image.ppm").unwrap();

    let wx = 200;
    let wy = 100;
    let samples_per_pixel = 100;

    file.write(format!("P3\n{} {}\n255\n", wx, wy).as_bytes()).unwrap();

    let camera = Camera::new(
        Vector3::new(0.0, 0.0, 0.0), 
        Vector3::new(-2.0, -1.0, -1.0), 
        Vector3::new(4.0, 0.0, 0.0), 
        Vector3::new(0.0, 2.0, 0.0)
    );

    let sphere1 = Sphere {
        center: Vector3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };

    let sphere2 = Sphere {
        center: Vector3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    };

    let hitable_list = HitableList {
        list: vec![Box::new(sphere1), Box::new(sphere2)],
    };

    let mut rng = rand::thread_rng();

    for y in (0..wy).rev() {
        for x in 0..wx {
            let mut color = Vector3::new(0.0, 0.0, 0.0);

            for _sample in 0..samples_per_pixel {
                let u = (x as f32 + rng.gen::<f32>()) / wx as f32;
                let v = (y as f32 + rng.gen::<f32>()) / wy as f32;
                let ray = camera.get_ray(u, v);
                color += ray.color(&hitable_list);
            }

            color /= samples_per_pixel as f32;

            let r = (255.0 * color.x) as u8;
            let g = (255.0 * color.y) as u8;
            let b = (255.0 * color.z) as u8;
            file.write(format!("{} {} {}\n", r, g, b).as_bytes()).unwrap();
        }
    }
}
