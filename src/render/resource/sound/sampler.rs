mod cues;
mod sound;
pub mod sndfile;
mod sndfile_ffi;

use std::collections::HashMap;
pub use self::sound::{Sound, PlaybackState};

pub const N_CUE_BUFFER_FRAMES: usize = 1 << 14; // 16384
type SoundMap = HashMap<String, Sound>;

#[derive(Debug)]
pub struct Sampler {
    sounds: SoundMap
}

impl Sampler {

    pub fn new () -> Sampler {
        Sampler { sounds: HashMap::new() }
    }

    pub fn load (&mut self, name: &str, path: &str) {
        self.sounds.insert(name.to_string(), Sound::new(&path));
    }

    pub fn play (&mut self, sound: &str, cue: Option<&str>) {
        match self.sounds.get_mut(sound) {
            None => eprintln!("no command {}", &sound),
            Some(sound) => match cue {
                None => sound.play_from_start(),
                Some(cue) => sound.play_from_cue(cue)
            }
        }
    }

}
