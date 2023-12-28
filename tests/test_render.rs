use std::path::Path;

mod test_utils;
use crate::test_utils::*;

#[test]
fn invalid_input_file() {
	let fin = Path::new("nonexistent.mov");

	let actual = asciiframe::render(fin, 100, 100, render_stdout).unwrap_err();
	let expected = asciiframe::Error::InvalidPath(fin.to_owned());

	assert_eq!(actual.to_string(), expected.to_string());
}

#[test]
fn render_image() {
	let fin = Path::new("tests/sources/test.png");

	asciiframe::render(&fin, 100, 100, render_stdout).unwrap();
}
