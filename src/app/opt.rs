use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(about, version, author)]
pub struct Opt {
	#[clap(help = "Print everything", short, long)]
	pub verbose: bool,

	#[clap(help = "Use colors", short, long)]
	pub color: bool,

	#[clap(help = "Output to a script file for sharing", short, long)]
	pub output: bool,

	#[clap(help = "Video filename", name = "FILE", parse(from_os_str))]
	pub file: PathBuf,
}
