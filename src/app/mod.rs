use clap::Parser;

mod converter;
mod opt;
mod renderer;
mod validators;

pub fn main() -> Result<(), String> {
	let opts = opt::Opt::parse();

	let strategy = converter::ASCII;
	// TODO: implement colors opt

	match renderer::render(&opts.file, opts.output.as_deref(), strategy) {
		Err(e) => {
			return Err(format!("OpenCV error code {}:\n{}", e.code, e.message))
		}
		Ok(()) => return Ok(()),
	}
}
