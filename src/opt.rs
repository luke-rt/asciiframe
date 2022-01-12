use std::path::PathBuf;

use clap::Parser;

use crate::validators;

#[derive(Parser)]
#[clap(about, version, author)]
pub struct Opt {
	#[clap(
        help = "Video filename",
        short,
        long,
        parse(from_os_str),
        validator_os = validators::path_is_readable_file
    )]
	pub file: PathBuf,

	#[clap(
		help = "Output to a script file for sharing",
		short,
		long,
        parse(from_os_str),
        validator_os = validators::cwd_is_writable,
        validator_os = validators::file_does_not_exist,
    )]
	pub output: Option<PathBuf>,

	#[clap(help = "Use color ascii", short, long)]
	pub color: Option<PathBuf>,
}
