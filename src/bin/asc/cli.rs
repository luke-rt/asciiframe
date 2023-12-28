use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(about, version, author)]
pub struct Args {
	#[arg(help = "Video filename", short, long, value_name = "INPUT FILE")]
	pub file: PathBuf,

	#[arg(
		help = "Output to a script file for sharing",
		short,
		long,
		value_name = "OUTPUT FILE"
	)]
	pub output: Option<PathBuf>,

	#[clap(help = "Use color ascii", short, long)]
	pub color: bool,
}
