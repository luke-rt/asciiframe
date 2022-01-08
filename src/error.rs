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
			Error::Io(io_error) => write!(f, "IO ERR: {}", io_error),
			Error::Opencv(opencv_error) => write!(
				f,
				"OPENCV ERR: CODE: {}\n{}",
				opencv_error.code, opencv_error.message
			),
			Error::Msg(error) => write!(f, "ERR: {}", error),
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
