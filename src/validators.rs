// TODO: verify that this is needed bc error checking in the app might handle it
use ::std::fs::{metadata, File};
use ::std::path::Path;

use crate::error::{Error, Result};

pub fn path_is_readable_file<P: AsRef<Path> + ?Sized>(p: &P) -> Result<()> {
	let path = p.as_ref();

	if path.is_dir() {
		return Err(Error::from(format!(
			"{}: Input path must be a video file, not a directory",
			path.display()
		)));
	}

	File::open(path)
		.map(|_| ())
		.map_err(|e| Error::from(format!("{}: {}", path.display(), e)))
}

pub fn cwd_is_writable<P: AsRef<Path> + ?Sized>(p: &P) -> Result<()> {
	let path = p.as_ref();

	if metadata(path)?.permissions().readonly() {
		return Err(Error::from(format!(
			"Unable to write to destination directory: {}",
			path.display()
		)));
	}

	Ok(())
}

pub fn file_does_not_exist<P: AsRef<Path> + ?Sized>(p: &P) -> Result<()> {
	let path = p.as_ref();

	if path.exists() {
		return Err(Error::from(format!(
			"File already exists: {}",
			path.display()
		)));
	}

	Ok(())
}
