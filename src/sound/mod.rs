mod sndfile;
mod sndfile_ffi;

use sndfile::{SndFile, OpenMode, SeekMode};

pub fn get_duration (path: &str) -> u128 {
    let sound = SndFile::new(path, OpenMode::Read).unwrap();
    let info = sound.get_sndinfo();
    eprintln!("frames {} {}", &path, &info.frames);
    info.frames as u128
}

pub fn get_frame (path: &str, frame: i64) -> Option<Vec<f32>> {
    eprint!("get_frame {} {}", &path, &frame);
    if frame < 0 { return None }
    let mut sound = SndFile::new(path, OpenMode::Read).unwrap();
    let info = sound.get_sndinfo();
    let channels = info.channels;
    let mut frames = vec![0.0; channels as usize];
    sound.seek(frame, SeekMode::SeekSet);
    let result = sound.readf_f32(frames.as_mut_slice(), 1);
    eprintln!(" ---> {} {} {:?}", &result, &channels, &frames);
    Some(frames)
}
