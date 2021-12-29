#![forbid(unsafe_code)]
#![warn(warnings, rust_2018_idioms)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(
	clippy::float_arithmetic,
	clippy::implicit_return,
	clippy::needless_return
)]
// #![warn(clippy::missing_docs_in_private_items)]

mod app;

fn main() {
	println!("\n---ASCIIFRAME---\n");

	if let Err(e) = app::main() {
		// TODO: add colors
		println!("error: {}\n", e);
		println!("For more information try --help");
	}
}
