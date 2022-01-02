// load video file with OpenCV
// get frames
// loop:
//     for each frame apply converter::convert(frame)
//     spit out ascii to either stdout or bash script
//     await the required amt of time before next frame to comply with fps
use std::path::Path;
use std::result::Result;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

use opencv::prelude::*;
use opencv::{core, imgproc, videoio, Error};
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
	let time_d: f32 = (1.0 / capture.get(videoio::CAP_PROP_FPS)?) as f32;
	let term = termsize::get().unwrap();

	for _i in 0..frame_count {
		let start = SystemTime::now();
		print!("{esc}c", esc = 27 as char);

		let mut frame = Mat::default();
		// CV_8UC3
		capture.read(&mut frame)?;

		let mut resized = Mat::default();
		imgproc::resize(
			&frame,
			&mut resized,
			core::Size {
				width: term.cols as i32,
				height: term.rows as i32,
			},
			0.0,
			0.0,
			imgproc::INTER_AREA,
		)?;

		frame.release()?;

		render_frame(&resized, strategy, output)?;
		resized.release()?;

		let elapsed = start.elapsed().unwrap().as_secs_f32();
		if elapsed < time_d {
			sleep(Duration::from_millis(((time_d - elapsed) * 1000.0) as u64));
		}
	}

	Ok(())
}

fn render_frame(frame: &Mat, strategy: u8, output: bool) -> Result<(), Error> {
	let ascii = converter::convert_frame(frame, strategy)?;

	println!("{}", ascii);

	Ok(())
}
