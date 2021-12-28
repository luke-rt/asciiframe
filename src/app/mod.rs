use clap::Parser;

mod opt;
mod render;
mod validators;

pub fn main() {
	let opts = opt::Opt::parse();

	println!("Verbose: {}", opts.verbose);
}
