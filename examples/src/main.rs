use std::path::Path;

fn main() -> asciiframe::Result<()> {
	let strategy = asciiframe::Strategy::Ascii;
	let fin = Path::new("test.mov");
	let fout = Path::new("test.sh");

	asciiframe::render_to_stdout(&fin, strategy)?;
	asciiframe::render_to_file(&fin, &fout, strategy)?;

	Ok(())
}
