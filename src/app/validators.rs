use faccess::PathExt;
use std::fs::File;
use std::path::Path;
use std::result::Result;

pub fn path_is_readable_video<P: AsRef<Path> + ?Sized>(
	p: &P,
) -> Result<(), String> {
	let path = p.as_ref();

	if path.is_dir() {
		return Err(format!(
			"{}: Input path must be a video file, not a directory",
			path.display()
		));
	}

	// TODO: verify that the filetype is MP4, MPEG, GIF, etc

	File::open(path)
		.map(|_| ())
		.map_err(|e| format!("{}: {}", path.display(), e))
}

pub fn cwd_is_writable<P: AsRef<Path> + ?Sized>(p: &P) -> Result<(), String> {
	let path = p.as_ref();

	if !path.is_dir() {
		return Err(format!("Not a directory: {}", path.display()));
	}
	if path.writable() {
		// TODO: implement writability check independently(maybe by attempting to write to the dir and returning the Result)
		return Ok(());
	}

	Err(format!(
		"Would be unable to write to destination directory: {}",
		path.display()
	))
}

pub fn file_does_not_exist<P: AsRef<Path> + ?Sized>(
	p: &P,
) -> Result<(), String> {
	let path = p.as_ref();

	if path.exists() {
		return Err(format!("File already exists: {}", path.display()));
	}

	Ok(())
}
