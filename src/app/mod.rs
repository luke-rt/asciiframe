use clap::Parser;

mod opt;
mod render;
mod validators;

pub fn main() -> Result<(), String> {
	let opts = opt::Opt::parse();

	validators::cwd_is_writable(&opts.file)?;

	Ok(())
}
