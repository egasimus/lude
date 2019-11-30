mod sndfile;
mod sndfile_ffi;

use std::cell::RefCell;
use std::collections::HashMap;
use sndfile::{SndFile, OpenMode, SeekMode};

#[derive(Debug)]
pub struct SoundMap {
    sounds: RefCell<HashMap<String, SndFile>>
}

impl SoundMap {
    pub fn new () -> SoundMap {
        SoundMap { sounds: RefCell::new(HashMap::new()) }
    }

    fn get_sound (&mut self, path: &str) -> SndFile {
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

    pub fn get_duration (&mut self, path: &str) -> u128 {
        let mut sound = self.get_sound(path);
        let info = sound.get_sndinfo();
        info.frames as u128
    }

    pub fn get_frame (&mut self, path: &str, frame: i64) -> Option<Vec<f32>> {
        if frame < 0 { return None }
        let mut sound = self.get_sound(path);
        let info = sound.get_sndinfo();
        let channels = info.channels;
        let mut frames = vec![0.0; channels as usize];
        sound.seek(frame + 1, SeekMode::SeekSet);
        sound.readf_f32(frames.as_mut_slice(), 1);
        Some(frames)
    }
}
