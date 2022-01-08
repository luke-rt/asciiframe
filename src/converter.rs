// given frame use conversion method on frame
// return string of ascii characters for that frame
use colored::Colorize;
use opencv::prelude::*;

use crate::error::{Error, Result};

pub const CHARS: [char; 11] =
	[' ', ' ', '.', ':', '!', '+', '*', 'e', '$', '@', '8'];

pub const ASCII: u8 = 0;
pub const COLOR_ASCII: u8 = 1;

pub fn convert_frame(frame: &Mat, strategy: u8) -> Result<String> {
	let mut res = String::default();

	for i in 0..frame.rows() {
		for j in 0..frame.cols() {
			let bgr: opencv::core::Vec3b =
				*frame.at_2d::<opencv::core::Vec3b>(i, j)?;
			res.push_str(&convert_pxl(bgr, strategy).unwrap());
		}
		res.push('\n');
	}

	Ok(res)
}

fn convert_pxl(bgr: opencv::core::Vec3b, strategy: u8) -> Result<String> {
	let b = *bgr.get(0).unwrap();
	let g = *bgr.get(1).unwrap();
	let r = *bgr.get(2).unwrap();

	match strategy {
		ASCII => Ok(to_ascii(r, g, b, strategy)?.to_string()),
		COLOR_ASCII => Ok(to_color_ascii(r, g, b, strategy)?),
		_ => Err(Error::from("Invalid strategy code")),
	}
}

// conversion strategies
fn to_ascii(r: u8, g: u8, b: u8, strategy: u8) -> Result<char> {
	let brightness: f32;

	match strategy {
		ASCII => {
			brightness = 0.2126 * f32::from(r)
				+ 0.7152 * f32::from(g)
				+ 0.0722 * f32::from(b);
		}
		COLOR_ASCII => {
			brightness = 0.267 * f32::from(r)
				+ 0.642 * f32::from(g)
				+ 0.091 * f32::from(b);
		}
		_ => {
			return Err(Error::from("Invalid strategy code"));
		}
	}

	Ok(CHARS[(10.0 * brightness / 255.0) as usize])
}

fn to_color_ascii(r: u8, g: u8, b: u8, strategy: u8) -> Result<String> {
	Ok(to_ascii(r, g, b, strategy)?
		.to_string()
		.truecolor(r, g, b)
		.to_string())
}
