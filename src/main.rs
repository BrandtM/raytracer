mod image;
mod ray;
mod sphere;
mod hitable;
mod ray_hit;
mod hitable_list;
mod camera;
mod material;
mod plane;

use cgmath::prelude::*;
use cgmath::Vector3;
use rayon::prelude::*;
use rand::Rng;
use image::*;
use sphere::*;
use hitable_list::*;
use camera::*;
use material::*;
use plane::*;

fn main() {
	let x_resolution = 1280;
	let y_resolution = 720;
	let samples_per_pixel = 500;

	let main_sphere = Sphere {
		center: Vector3::new(0.0, 0.0, -1.0), 
		radius: 0.5,
		material: Box::new(Lambertian {
			albedo: Vector3::new(0.8, 0.3, 0.3)
		})
	};

	let sphere_right = Sphere {
		center: Vector3::new(1.0, 0.0, -1.0), 
		radius: 0.5,
		material: Box::new(Metal {
			albedo: Vector3::new(0.8, 0.6, 0.2),
			fuzz: 1.0
		})
	};

	let sphere_left = Sphere {
		center: Vector3::new(-1.0, 0.0, -1.0), 
		radius: 0.5,
		material: Box::new(Dielectric {
			refraction_index: 1.5
		})
	};

	let sphere_front = Sphere {
		center: Vector3::new(0.0, 0.0, 0.0), 
		radius: 0.5,
		material: Box::new(Metal {
			albedo: Vector3::new(0.8, 0.8, 0.8),
			fuzz: 0.3
		})
	};

	let ground_sphere = Sphere {
		center: Vector3::new(0.0, -100.5, -1.0), 
		radius: 100.0,
		material: Box::new(Lambertian {
			albedo: Vector3::new(0.8, 0.3, 0.0)
		})
	};

	let some_plane = Plane {
		center: Vector3::new(-1.2, -2.2, -3.5),
		normal: Vector3::new(-0.4, -0.3, 1.0),
		width: Vector3::new(3.0, 0.0, 0.0),
		height: Vector3::new(0.0, 4.0, 0.0),
		material: Box::new(Metal {
			albedo: Vector3::new(0.8, 0.8, 0.8),
			fuzz: 0.1
		})
	};

	let look_from = Vector3::new(3.0, 3.0, 2.0);
	let look_at = Vector3::new(0.0, 0.0, -1.0);

	let camera = Camera::new(
		look_from, 
		look_at, 
		Vector3::new(0.0, 1.0, 0.0),
		45.0, 
		x_resolution as f32 / y_resolution as f32, 
		0.2, 
		(look_from - look_at).magnitude()
	);

	let hitable_list = HitableList {
		list: vec![
			Box::new(main_sphere),
			Box::new(sphere_right),
			Box::new(sphere_left),
			Box::new(sphere_front),
			Box::new(ground_sphere),
			Box::new(some_plane),
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

	img.save("image.ppm").unwrap();
}