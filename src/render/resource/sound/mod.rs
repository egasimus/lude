mod sndfile;
mod sndfile_ffi;

use sndfile::{SndFile, OpenMode};

pub fn get_duration (path: &str) -> u128 {
    let sound = SndFile::new(path, OpenMode::Read).unwrap();
    let info = sound.get_sndinfo();
    info.frames as u128
}
