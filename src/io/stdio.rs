use crate::types::Frame;
use crate::media::sndfile::{SndFile, OpenMode};

pub fn write_to_stdout (frames: Vec<Frame>) {
    let start = Instant::now();

    let sndfile = SndFile::new_with_fd(1, OpenMode::Write, true).unwrap();
    let output = frames.map(|f| match f {
        Some(f) => f.get(0).unwrap(),
        None => 0
    }).collect()
    sndfile.write_sync();

    eprintln!("wrote {} frames in {}usec ",
        &frames.len(), start.elapsed().as_micros());
}
