use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Pixel {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
}

#[derive(Clone)]
pub struct Image {
	pub width: u32,
	pub height: u32,
	pub pixels: Vec<Vec<Pixel>>,
}

impl Image {
	pub fn save(&self, file_name: &str) -> std::io::Result<usize> {
		let mut file = File::create(file_name)?;
		let mut pixel_str = String::new();
		file.write(format!("P3\n{} {}\n255\n", self.width, self.height).as_bytes())?;

		self.pixels.iter().rev().for_each(|pixel_row| {
			pixel_row.iter().for_each(|pixel| {
				pixel_str = format!("{}{} {} {}\n", pixel_str, pixel.red, pixel.green, pixel.blue);
			});
		});

		file.write(pixel_str.as_bytes())
	}
}