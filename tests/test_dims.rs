use std::path::Path;

mod test_utils;
use crate::test_utils::*;

#[test]
fn test_zero_dimensions() {
	let fin = Path::new("tests/sources/test.mov");

	let actual = asciiframe::render(&fin, 0, 0, render_stdout).unwrap_err();
	let expected = asciiframe::Error::InvalidBounds(0, 0);

	assert_eq!(actual.to_string(), expected.to_string());
}

/*
#[test]
fn test_dimensions_too_large() {
	let fin = Path::new("tests/sources/test.mov");
	let fout1 = Path::new("tests/artifacts/test1.sh");
	let fout2 = Path::new("tests/artifacts/test2.sh");

	asciiframe::render(&fin, 20000, 20000, |frame| { render_file(frame, fout1) }).unwrap();
	asciiframe::render(&fin, 10000, 10000, |frame| { render_file(frame, fout2) }).unwrap();

	let result = std::fs::read_to_string(fout1).unwrap();
	let expected = std::fs::read_to_string(fout2).unwrap();

	assert_eq!(result, expected);
}
*/
