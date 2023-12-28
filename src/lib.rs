#![forbid(unsafe_code)]
#![warn(warnings)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(
	clippy::cast_sign_loss,
	clippy::cast_possible_truncation,
	clippy::get_first
)]

use opencv::prelude::*;
use opencv::{core, imgproc, videoio};
use std::path::Path;
use std::time::{Duration, SystemTime};

mod converter;
mod error;

pub use error::*;

pub struct Frame {
	pub data: String,
	pub index: u64,
	pub total: u64,
	pub fps: u16,
	pub elapsed: f32,
}

/// # Errors
/// * [`error::Error::OpenCv`] if opencv returns an error while processing the video or a frame
/// * [`error::Error::Io`] if an `std::io` error occurs
pub fn render(
	fin: &Path,
	width: i32,
	height: i32,
	mut render_frame: impl FnMut(&Frame) -> error::Result<()>,
) -> error::Result<()> {
	if !fin.exists() {
		return Err(error::Error::InvalidPath(fin.to_owned()));
	}
	if width <= 0 || height <= 0 {
		return Err(error::Error::InvalidBounds(width, height));
	}

	let mut capture = videoio::VideoCapture::from_file(
		fin.to_str()
			.ok_or(error::Error::InvalidPath(fin.to_owned()))?,
		0,
	)?;
	let frame_count: u64 = capture.get(videoio::CAP_PROP_FRAME_COUNT)? as u64;

	for i in 0..frame_count {
		let start = SystemTime::now();

		let mut frame = Mat::default();
		capture.read(&mut frame)?;

		if frame.cols() > width || frame.rows() > height {
			let mut resized_frame = Mat::default();
			imgproc::resize(
				&frame,
				&mut resized_frame,
				core::Size {
					width,
					height,
				},
				0.0,
				0.0,
				imgproc::INTER_AREA,
			)?;

			frame = resized_frame;
		}

		render_frame(&Frame {
			data: converter::convert_frame(&frame)?,
			index: i,
			total: frame_count,
			fps: capture.get(videoio::CAP_PROP_FPS)? as u16,
			elapsed: start.elapsed().unwrap_or(Duration::ZERO).as_secs_f32(),
		})?;
	}

	Ok(())
}
