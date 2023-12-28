use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("IO Error")]
	Io(#[from] std::io::Error),
	#[error("OpenCV Error")]
	OpenCV(#[from] opencv::Error),
	#[error("Invalid input file path")]
	InvalidPath(PathBuf),
	#[error("Invalid target bounds")]
	InvalidBounds(i32, i32),
}

pub type Result<T> = std::result::Result<T, Error>;
