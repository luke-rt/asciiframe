use ansi_term::Colour::Red;
use std::fmt::Display;

#[derive(Debug)]

pub enum Error {
	Io(::std::io::Error),
	Opencv(::opencv::Error),
	Msg(String),
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Error::Io(io_error) => write!(f, "IO ERR: {io_error}"),
			Error::Opencv(opencv_error) => write!(f, "{opencv_error}"),
			Error::Msg(error) => write!(f, "ERR: {error}"),
		}
	}
}

impl From<::std::io::Error> for Error {
	fn from(s: ::std::io::Error) -> Self {
		Error::Io(s)
	}
}

impl From<::opencv::Error> for Error {
	fn from(s: ::opencv::Error) -> Self {
		Error::Opencv(s)
	}
}

impl From<&'static str> for Error {
	fn from(s: &'static str) -> Self {
		Error::Msg(s.to_owned())
	}
}

impl From<String> for Error {
	fn from(s: String) -> Self {
		Error::Msg(s)
	}
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

pub fn default_error_handler(handle: &mut dyn std::io::Write, error: &Error) {
	writeln!(
		handle,
		"{} {}\n\nFor more information go to docs.rs/asciiframe",
		Red.paint("error:"),
		error
	)
	.unwrap();
}
