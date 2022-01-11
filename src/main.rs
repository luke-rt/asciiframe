#![forbid(unsafe_code)]
#![warn(warnings, rust_2018_idioms)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
use std::io::{self, Write};

use ansi_term::Colour::{Green, Red};
use clap::Parser;

mod converter;
mod error;
mod opt;
mod renderer;
mod validators;

pub fn main() {
	// TODO: implement colors opt
	// TODO: replace println! with writeln!(io::stdout.lock())

	let opts = opt::Opt::parse();

	let strategy = converter::ASCII;

	if let Err(e) =
		renderer::render(&opts.file, opts.output.as_deref(), strategy)
	{
		let stderr = io::stderr();
		let mut handle = stderr.lock();
		write!(
			handle,
			"{} {}\n\nFor more information try {}",
			Red.paint("error:"),
			e,
			Green.paint("--help")
		)
		.unwrap();
	}
}
