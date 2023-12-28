use crate::error::Result;
use opencv::prelude::*;

pub const CHARS: [char; 11] = [' ', ' ', '.', ':', '!', '+', '*', 'e', '$', '@', '8'];

pub fn convert_frame(frame: &Mat) -> Result<String> {
	let mut res = String::default();

	for i in 0..frame.rows() {
		for j in 0..frame.cols() {
			let bgr: opencv::core::Vec3b = *frame.at_2d::<opencv::core::Vec3b>(i, j)?;
			res.push(convert_pxl(bgr));
		}
		res.push('\n');
	}

	Ok(res)
}

fn convert_pxl(bgr: opencv::core::Vec3b) -> char {
	let b = *bgr.get(0).expect("Should be in bound of 3");
	let g = *bgr.get(1).expect("Should be in bound of 3");
	let r = *bgr.get(2).expect("Should be in bound of 3");

	to_ascii(r, g, b)
}

// conversion strategies
fn to_ascii(r: u8, g: u8, b: u8) -> char {
	let brightness = 0.2126 * f32::from(r) + 0.7152 * f32::from(g) + 0.0722 * f32::from(b);

	*CHARS
		.get((10.0 * brightness / 255.0) as usize)
		.expect("RGB values should not exceed 255")
}
