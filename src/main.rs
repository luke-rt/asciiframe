#![forbid(unsafe_code)]
#![warn(warnings, rust_2018_idioms)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::float_arithmetic, clippy::implicit_return, clippy::needless_return)]
// #![warn(clippy::missing_docs_in_private_items)]

mod app;

fn main() {
	app::main();
}
