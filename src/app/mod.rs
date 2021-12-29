use std::ffi::OsString;
use clap::Parser;

mod opt;
mod render;
mod validators;

pub fn main() -> Result<(), OsString> {
	let opts = opt::Opt::parse();

    Ok(())
}
