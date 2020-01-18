mod camera;
mod hitable;
mod hitable_list;
mod image;
mod material;
mod model_loader;
mod plane;
mod ray;
mod ray_hit;
mod sphere;
mod triangle;

use camera::*;
use cgmath::prelude::*;
use cgmath::Vector3;
use hitable_list::*;
use image::*;
use rand::Rng;
use rayon::prelude::*;
use std::path::Path;

fn main() {
    let x_resolution = 1280;
    let y_resolution = 720;
    let samples_per_pixel = 20;

    let look_from = Vector3::new(0.0, 0.0, 5.4);
    let look_at = Vector3::new(0.0, 0.0, -1.0);

    let camera = Camera::new(
        look_from,
        look_at,
        Vector3::new(0.0, 1.0, 0.0),
        60.0,
        x_resolution as f32 / y_resolution as f32,
        0.02,
        (look_from - look_at).magnitude(),
    );

    env_logger::init();

    let hitable_list = HitableList {
        list: model_loader::load(&Path::new("models/monkey.obj")),
    };

    let pixels: Vec<Vec<Pixel>> = (0..y_resolution)
        .into_par_iter()
        .map(|y| {
            (0..x_resolution)
                .into_par_iter()
                .map(|x| {
                    let mut color = (0..samples_per_pixel)
                        .into_par_iter()
                        .map(|_| {
                            let mut rng = rand::thread_rng();

                            let u = (x as f32 + rng.gen::<f32>()) / x_resolution as f32;
                            let v = (y as f32 + rng.gen::<f32>()) / y_resolution as f32;

                            let ray = camera.get_ray(u, v);
                            ray.color(hitable_list.clone(), 0)
                        })
                        .reduce(
                            || Vector3::new(0.0, 0.0, 0.0),
                            |col, ray_color| col + ray_color,
                        );

                    color /= samples_per_pixel as f32;

                    Pixel {
                        red: (255_f32 * color.x.sqrt()) as u8,
                        green: (255_f32 * color.y.sqrt()) as u8,
                        blue: (255_f32 * color.z.sqrt()) as u8,
                    }
                })
                .collect()
        })
        .collect();

    let img = Image {
        width: x_resolution,
        height: y_resolution,
        pixels,
    };

    img.save("image.ppm").unwrap();
}
