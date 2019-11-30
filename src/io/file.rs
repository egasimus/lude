/// read/write from/to files
use std::time::Instant;
use crate::types::{Frame, Sample};
use crate::media::sndfile::{SndFile, SndInfo, OpenMode, FormatType};

pub fn write_to_file (frames: Vec<Frame>, path: &str) {
    let start = Instant::now();
    let mut flat_frames: Vec<Sample> = frames.into_iter().flatten().collect();
    let items = flat_frames.len() as i64;
    let mut sndfile = SndFile::new_with_info(
        &path,
        OpenMode::Write,
        Box::new(SndInfo {
            frames: items,
            samplerate: 44100,
            channels: 1,
            format: (FormatType::FormatWav|FormatType::FormatPcm16) as i32,
            sections: 0,
            seekable: 0
        })
    ).unwrap();
    sndfile.write_i16(flat_frames.as_mut_slice(), items);
    eprintln!("wrote {} samples to {} in {}usec ",
        &flat_frames.len(), &path, start.elapsed().as_micros());
}
