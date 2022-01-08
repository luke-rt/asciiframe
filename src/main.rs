#![forbid(unsafe_code)]
#![warn(warnings, rust_2018_idioms)]
#![warn(clippy::all, clippy::pedantic)]
// #![warn(clippy::missing_docs_in_private_items)]
#![allow(
	clippy::float_arithmetic,
	clippy::implicit_return,
	clippy::needless_return
)]

use clap::Parser;

mod converter;
mod error;
mod opt;
mod renderer;
mod validators;

pub fn main() {
	let opts = opt::Opt::parse();

	let strategy = converter::ASCII;
	// TODO: implement colors opt

	if let Err(e) =
		renderer::render(&opts.file, opts.output.as_deref(), strategy)
	{
		println!("{}", e);
	}
}
