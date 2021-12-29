use std::path::PathBuf;

use clap::Parser;

use crate::app::validators;

#[derive(Parser)]
#[clap(about, version, author)]
pub struct Opt {
	#[clap(help = "Print everything", short, long)]
	pub verbose: bool,

	#[clap(help = "Use colors", short, long)]
	pub color: bool,

	#[clap(help = "Output to a script file for sharing", short, long = "output-file")]
	pub output: bool,

    #[clap(help = "Video filename", short, long, parse(from_os_str), validator_os = validators::path_readable_video)]
	pub file: PathBuf,
}
