use std::fs::File;
use std::path::Path;
use std::result::Result;

pub fn path_readable_video<P: AsRef<Path> + ?Sized>(p: &P)
        -> Result<(), String> {
    // TODO: figure out why OsString leads to a compile time err
    let path = p.as_ref();

    if path.is_dir() {
        return Err(format!("{}: Input path must be a video file, not a directory", path.display()));
    }

    // TODO: verify that the filetype is MP4, MPEG, GIF, etc

    File::open(path)
        .map(|_| ())
        .map_err(|e| format!("{}: {}", path.display(), e))
}
