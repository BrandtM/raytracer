mod image;
mod ray2;
mod sphere;
mod hitable;
mod ray_hit;
mod hitable_list;
mod camera;
mod material;

use cgmath::prelude::*;
use cgmath::Vector3;
use rayon::prelude::*;
use rand::Rng;
use image::*;
use ray2::*;
use sphere::*;
use hitable_list::*;
use camera::*;
use material::*;

fn main() {
	let x_resolution = 200;
	let y_resolution = 100;
	let samples_per_pixel = 100;

	let main_sphere = Sphere {
		center: Vector3::new(0.0, 0.0, -1.0), 
		radius: 0.5,
		material: Box::new(Lambertian {
			albedo: Vector3::new(0.8, 0.3, 0.3)
		})
	};

	let ground_sphere = Sphere {
		center: Vector3::new(0.0, -100.5, -1.0), 
		radius: 100.0,
		material: Box::new(Lambertian {
			albedo: Vector3::new(0.8, 0.8, 0.0)
		})
	};

	let camera = Camera::new(
		Vector3::new(0.0, 0.0, 2.0), 
		Vector3::new(0.0, 0.0, -1.0), 
		Vector3::new(0.0, 1.0, 0.0),
		45.0, 
		x_resolution as f32 / y_resolution as f32, 
		0.00, 
		100.0
	);

	let hitable_list = HitableList {
		list: vec![
			Box::new(main_sphere),
			Box::new(ground_sphere),
		]
	};
	
	let pixels: Vec<Vec<Pixel>> = (0..y_resolution).into_par_iter().map(|y| {
		(0..x_resolution).into_par_iter().map(|x| {
			let mut color = (0..samples_per_pixel).into_par_iter().map(|_| {
				let mut rng = rand::thread_rng();

				let u = (x as f32 + rng.gen::<f32>()) / x_resolution as f32;
				let v = (y as f32 + rng.gen::<f32>()) / y_resolution as f32;

				let ray = camera.get_ray(u, v);
				ray.color(hitable_list.clone(), 0)
			}).reduce(|| Vector3::new(0.0, 0.0, 0.0), |col, ray_color| col + ray_color);
			
			color /= samples_per_pixel as f32;

			Pixel {
				red: (255_f32 * color.x.sqrt()) as u8,
				green: (255_f32 * color.y.sqrt()) as u8,
				blue: (255_f32 * color.z.sqrt()) as u8
			}
		}).collect()
	}).collect();

	let img = Image {
		width: x_resolution,
		height: y_resolution,
		pixels
	};

	img.save("wow.ppm");
}