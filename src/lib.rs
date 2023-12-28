#![forbid(unsafe_code)]
#![warn(warnings)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::cast_sign_loss, clippy::cast_possible_truncation, clippy::get_first)]

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
	width: u16,
	height: u16,
	mut render_frame: impl FnMut(&Frame) -> error::Result<()>,
) -> error::Result<()> {
	let mut capture = videoio::VideoCapture::from_file(fin.to_str().ok_or(error::Error::InvalidPath(fin.to_owned()))?, 0)?;
	let frame_count: u64 = capture.get(videoio::CAP_PROP_FRAME_COUNT)? as u64;

	for i in 0..frame_count {
		let start = SystemTime::now();

		let mut original_frame = Mat::default();
		capture.read(&mut original_frame)?;

		let mut resized_frame = Mat::default();
		imgproc::resize(
			&original_frame,
			&mut resized_frame,
			core::Size {
				width: i32::from(width - 1),
				height: i32::from(height - 1),
			},
			0.0,
			0.0,
			imgproc::INTER_AREA,
		)?;

		render_frame(&Frame {
			data: converter::convert_frame(&resized_frame)?,
			index: i,
			total: frame_count,
			fps: capture.get(videoio::CAP_PROP_FPS)? as u16,
			elapsed: start.elapsed().unwrap_or(Duration::ZERO).as_secs_f32(),
		})?;
	}

	Ok(())
}
