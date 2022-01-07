#![forbid(unsafe_code)]
#![warn(warnings, rust_2018_idioms)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(
	clippy::float_arithmetic,
	clippy::implicit_return,
	clippy::needless_return
)]
// #![warn(clippy::missing_docs_in_private_items)]

use clap::Parser;

mod error;
mod opt;
mod converter;
mod renderer;
mod validators;

pub fn main() {
	let opts = opt::Opt::parse();

	let strategy = converter::ASCII;
	// TODO: implement colors opt

    if let Err(e) = renderer::render(&opts.file, opts.output.as_deref(), strategy) {
        println!("{}", e);
    }
}

