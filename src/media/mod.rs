pub mod sndfile;
mod sndfile_ffi;

use std::cell::RefCell;
use std::collections::HashMap;
use sndfile::{SndFile, OpenMode, SeekMode};
use crate::types::{Frame, FrameTime};

#[derive(Debug)]
pub struct SoundMap {
    sounds: RefCell<HashMap<String, SndFile>>,
    durations: RefCell<HashMap<String, FrameTime>>,
}

impl SoundMap {
    pub fn new () -> SoundMap {
        SoundMap {
            sounds: RefCell::new(HashMap::new()),
            durations: RefCell::new(HashMap::new())
        }
    }
    fn get_sound (&self, path: &str) -> SndFile {
        let mut sounds = self.sounds.borrow_mut();
        let sound = sounds.get(path);
        match sound {
            Some(sound) => sound.clone(),
            None => {
                let sound = SndFile::new(path, OpenMode::Read).unwrap();
                sounds.insert(path.to_string(), sound.clone());
                sound
            }
        }.clone()
    }
    pub fn get_duration (&self, path: &str) -> FrameTime {
        let sound = self.get_sound(path);
        let info = sound.get_sndinfo();
        info.frames as FrameTime
    }
    pub fn get_frame (&self, path: &str, frame: i64) -> Option<Frame> {
        if frame < 0 { return None }
        let mut sound = self.get_sound(path);
        let info = sound.get_sndinfo();
        let channels = info.channels;
        let mut frames = vec![0; channels as usize];
        sound.seek(frame, SeekMode::SeekSet);
        sound.readf_i16(frames.as_mut_slice(), 1);
        Some(frames)
    }
    pub fn get_source_length (&self, event: &str) -> FrameTime {
        let mut durations = self.durations.borrow_mut();
        match durations.get(event) {
            Some(duration) => *duration,
            None => {
                let duration = self.get_duration(&event);
                durations.insert(event.to_string(), duration);
                duration
            }
        }
    }
}
