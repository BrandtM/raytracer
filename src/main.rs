
extern crate cgmath;
extern crate num_cpus;

mod hitable;
mod hitable_list;
mod sphere;
mod ray_hit;
mod ray;
mod camera;
mod material;
mod image;

use std::rc::Rc;
use cgmath::Vector3;
use cgmath::prelude::*;
use rand::Rng;
use std::sync::{Arc, RwLock};

use sphere::Sphere;
use hitable_list::HitableList;
use camera::Camera;
use material::*;
use std::thread;
use image::*;

fn main() {
    let wx = 512_u32;
    let wy = 512_u32;
    let samples_per_pixel = 100;


    let look_from = Vector3::new(3.0, 3.0, 2.0);
    let look_at = Vector3::new(0.0, 0.0, -1.0);

    let camera = Camera::new(
        look_from,
        look_at,
        Vector3::new(0.0, 1.0, 0.0),
        20.0, 
        wx as f32 / wy as f32,
        5.0,
        (look_from - look_at).magnitude()
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
        material: Arc::new(RwLock::new(mat1)),
    };

    let sphere2 = Sphere {
        center: Vector3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Arc::new(RwLock::new(mat2)),
    };

    let sphere3 = Sphere {
        center: Vector3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Arc::new(RwLock::new(mat3)),
    };

    let sphere4 = Sphere {
        center: Vector3::new(-1.0, 0.0, -1.0),
        radius: -0.45,
        material: Arc::new(RwLock::new(mat5)),
    };

    let sphere5 = Sphere {
        center: Vector3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Arc::new(RwLock::new(mat5)),
    };

    let hitable_list = HitableList {
        list: vec![
            Arc::new(RwLock::new(sphere1)),
            Arc::new(RwLock::new(sphere2)),
            Arc::new(RwLock::new(sphere3)),
            Arc::new(RwLock::new(sphere4)),
            Arc::new(RwLock::new(sphere5)),
        ],
    };

    let empty_pixel = Pixel {red: 0, green: 0, blue: 0};
    let pixels = Arc::new(RwLock::new(vec![vec![empty_pixel; wx as usize]; wy as usize]));
    let thread_count = num_cpus::get();
    
    let mut handles: Vec<thread::JoinHandle<()>> = Vec::with_capacity(thread_count);

    for t in 0..thread_count {
        let x_range_step = 128;
        let y_range_step = 128;

        let x_range_start = (t * x_range_step) % wx as usize;
        let y_range_start = (t / 4 * y_range_step) % wy as usize;
        let world = hitable_list.clone();
        let pixels = pixels.clone();
        
        let handle = thread::spawn(move || {
            for y in (y_range_start..y_range_start+y_range_step).rev() {
                for x in x_range_start..x_range_start+x_range_step {
                    let mut rng = rand::thread_rng();

                    let x: usize = x as usize;
                    let y: usize = y as usize;
                    
                    let mut color = Vector3::new(0.0, 0.0, 0.0);

                    for _sample in 0..samples_per_pixel {
                        let u = (x as f32 + rng.gen::<f32>()) / wx as f32;
                        let v = (y as f32 + rng.gen::<f32>()) / wy as f32;
                        let ray = camera.get_ray(u, v);
                        color += ray.color(&world, 0);
                    }
                    
                    color /= samples_per_pixel as f32;
                    
                    let r = (255.99 * color.x.sqrt()) as u8;
                    let g = (255.99 * color.y.sqrt()) as u8;
                    let b = (255.99 * color.z.sqrt()) as u8;

                    pixels.write().unwrap()[y][x] = Pixel {
                        red: r,
                        green: g,
                        blue: b
                    };
                }
            }
        });

        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    let img = Image {
        width: wx,
        height: wy,
        pixels: pixels.read().unwrap().to_vec()
    };

    img.save("image.ppm").unwrap();
}
