// load video file with OpenCV
// get frames
// loop:
//     for each frame apply converter::convert(frame)
//     spit out ascii to either stdout or bash script
//     await the required amt of time before next frame to comply with fps
use opencv::prelude::*;
use opencv::{videoio, Error};
use std::path::Path;
use std::result::Result;
use termsize;

use crate::app::converter;

pub fn render(
	filename: &Path,
	output: bool,
	strategy: u8,
) -> Result<(), Error> {
	let mut capture =
		videoio::VideoCapture::from_file(filename.to_str().unwrap(), 0)?;
	let frame_count: u64 = capture.get(videoio::CAP_PROP_FRAME_COUNT)? as u64;
	let time_d: f64 = 1.0 / capture.get(videoio::CAP_PROP_FPS)?;
	let term = termsize::get().unwrap();

	for _i in 0..frame_count {
		let mut frame = Mat::default();
		capture.read(&mut frame)?;

		render_frame(&frame, strategy, output)?;
	}

	Ok(())
}

fn render_frame(frame: &Mat, strategy: u8, output: bool) -> Result<(), Error> {
	let ascii = converter::convert(frame, strategy);

    Ok(())
}
