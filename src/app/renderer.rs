// load video file with OpenCV
// get frames
// loop:
//     for each frame apply converter::convert(frame)
//     spit out ascii to either stdout or bash script
//     await the required amt of time before next frame to comply with fps
use std::path::Path;
use std::result::Result;
use opencv::prelude::*;
use opencv::{
    videoio,
    Error,
};


struct Video {
    width: f64,
    height: f64,
    frame_count: f64,
    fps: f64,
}


pub fn render(filename: &Path, output: bool) -> Result<(), Error> {
    let video = load(filename)?;

    println!("{}, {}, {}, {}", video.width, video.height, video.frame_count, video.fps);

    Ok(())
}

fn load(path: &Path) -> Result<Video, Error>{
    let capture = videoio::VideoCapture::from_file(path.to_str().unwrap(), 0)?;

    Ok(Video {
        width: capture.get(videoio::CAP_PROP_FRAME_WIDTH)?,
        height: capture.get(videoio:: CAP_PROP_FRAME_HEIGHT)?,
        frame_count: capture.get(videoio::CAP_PROP_FRAME_COUNT)?,
        fps: capture.get(videoio::CAP_PROP_FPS)?,
    })
}
