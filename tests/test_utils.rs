#[cfg(test)]
pub mod utils {
	use std::path::Path;
	use std::{fs::File, io::Write};

	pub fn render_stdout(frame: &asciiframe::Frame) -> asciiframe::Result<()> {
		println!("{esc}c", esc = 27 as char);
		println!("{}", frame.data);
		Ok(())
	}

	pub fn render_file(frame: &asciiframe::Frame, fout: &Path) -> asciiframe::Result<()> {
		let mut fout = File::create(fout)?;
		fout.write_all(format!("echo '{}'\necho '\u{001b}[0;0H' \n", frame.data).as_bytes())?;
		Ok(())
	}
}
pub use utils::*;
