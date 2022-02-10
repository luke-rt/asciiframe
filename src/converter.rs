use crate::error::Result;
use opencv::prelude::*;

pub const CHARS: [char; 11] = [' ', ' ', '.', ':', '!', '+', '*', 'e', '$', '@', '8'];

#[derive(Copy, Clone, Debug)]
pub enum Strategy {
	Ascii,
	ColorAscii,
}

pub fn convert_frame(frame: &Mat, strategy: Strategy) -> Result<String> {
	let mut res = String::default();

	for i in 0..frame.rows() {
		for j in 0..frame.cols() {
			let bgr: opencv::core::Vec3b = *frame.at_2d::<opencv::core::Vec3b>(i, j)?;
			res.push_str(&convert_pxl(bgr, strategy));
		}
		res.push('\n');
	}

	Ok(res)
}

fn convert_pxl(bgr: opencv::core::Vec3b, strategy: Strategy) -> String {
	let b = *bgr.get(0).unwrap();
	let g = *bgr.get(1).unwrap();
	let r = *bgr.get(2).unwrap();

	match strategy {
		Strategy::Ascii => to_ascii(r, g, b, strategy).to_string(),
		Strategy::ColorAscii => to_color_ascii(r, g, b),
	}
}

// conversion strategies
fn to_ascii(r: u8, g: u8, b: u8, strategy: Strategy) -> char {
	let brightness: f32;

	match strategy {
		Strategy::Ascii => {
			brightness = 0.2126 * f32::from(r) + 0.7152 * f32::from(g) + 0.0722 * f32::from(b);
		}
		Strategy::ColorAscii => {
			brightness = 0.267 * f32::from(r) + 0.642 * f32::from(g) + 0.091 * f32::from(b);
		}
	}

	CHARS[(10.0 * brightness / 255.0) as usize]
}

fn to_color_ascii(r: u8, g: u8, b: u8) -> String {
	String::from("hello world")
}
