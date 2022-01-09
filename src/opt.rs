use clap::Parser;
use std::path::PathBuf;

use crate::validators;

#[derive(Parser)]
#[clap(about, version, author)]
pub struct Opt {
	#[clap(help = "Print everything", short, long)]
	pub verbose: bool,

	#[clap(
		help = "Output to a script file for sharing",
		short,
		long = "output",
        parse(from_os_str),
        validator_os = validators::cwd_is_writable,
        validator_os = validators::file_does_not_exist,
    )]
	pub output: Option<PathBuf>,

	#[clap(
        help = "Video filename",
        short,
        long,
        parse(from_os_str),
        validator_os = validators::path_is_readable_file
    )]
	pub file: PathBuf,
}
