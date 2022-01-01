// given frame use conversion method on frame
// return string of ascii characters for that frame
use opencv::prelude::*;

mod to_ascii;
mod to_color_ascii;

use to_ascii::*;
use to_color_ascii::*;

pub const ASCII: u8 = 0;
pub const COLOR_ASCII: u8 = 1;

pub fn convert(frame: &Mat, strategy: u8) -> Result<String, ()> {
	// change Ok() type

	match strategy {
		ASCII => to_ascii(),
		COLOR_ASCII => to_color_ascii(),
		_ => Err(()),
	}
}
