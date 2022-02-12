#![forbid(unsafe_code)]
#![warn(warnings, rust_2018_idioms)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
use clap::Parser;
use std::process;

mod cli;
mod validators;

fn run(strategy: asciiframe::Strategy) -> asciiframe::Result<()> {
	let opts = cli::Opts::parse();

	if let Some(p) = opts.output {
		asciiframe::render_to_file(&opts.file, &p, strategy)?;
	} else {
		asciiframe::render_to_stdout(&opts.file, strategy)?;
	}

	Ok(())
}

pub fn main() {
	// TODO: implement colors opt
	let strategy = asciiframe::Strategy::Ascii;

	let result = run(strategy);

	match result {
		Err(error) => {
			let stderr = std::io::stderr();
			asciiframe::default_error_handler(&mut stderr.lock(), &error);
			process::exit(1);
		}
		Ok(()) => {
			process::exit(0);
		}
	}
}
