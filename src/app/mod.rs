use clap::Parser;

mod converter;
mod opt;
mod renderer;
mod validators;

pub fn main() -> Result<(), String> {
	let opts = opt::Opt::parse();

	// verify that the current dir is writeable if script file is desired
	if opts.output {
		validators::cwd_is_writable(&std::env::current_dir().unwrap())?;
	}

	match renderer::render(&opts.file, opts.output, converter::ASCII) {
		Err(e) => {
			return Err(format!("OpenCV error code {}:\n{}", e.code, e.message))
		}
		Ok(()) => return Ok(()),
	}
}
