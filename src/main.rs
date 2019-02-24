
extern crate cgmath;

use std::fs::File;
use std::io::prelude::*;
use cgmath::Vector3;

pub mod hitable;
pub mod hitable_list;
pub mod sphere;
pub mod ray_hit;
pub mod ray;

use ray::Ray;
use sphere::Sphere;
use hitable_list::HitableList;

fn main() {
    let mut file = File::create("image.ppm").unwrap();

    let wx = 200;
    let wy = 100;

    file.write(format!("P3\n{} {}\n255\n", wx, wy).as_bytes()).unwrap();

    let bottom_left = Vector3::new(-2.0, -1.0, -1.0);
    let horizontal = Vector3::new(4.0, 0.0, 0.0);
    let vertical = Vector3::new(0.0, 2.0, 0.0);
    let origin = Vector3::new(0.0, 0.0, 0.0);

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

    for y in (0..wy).rev() {
        for x in 0..wx {
            let u = x as f32 / wx as f32;
            let v = y as f32 / wy as f32;
            let ray = Ray {
                origin,
                direction: bottom_left + u * horizontal + v * vertical
            };

            let color = ray.color(&hitable_list);
            let r = (255.0 * color.x) as u8;
            let g = (255.0 * color.y) as u8;
            let b = (255.0 * color.z) as u8;
            file.write(format!("{} {} {}\n", r, g, b).as_bytes()).unwrap();
        }
    }
}
