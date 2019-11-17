mod cues;
mod sound;
pub mod sndfile;
mod sndfile_ffi;

use std::collections::HashMap;
pub use self::sound::Sound;

pub const N_CUE_BUFFER_FRAMES: usize = 1 << 14; // 16384

#[derive(Debug)]
pub struct Sampler {
    samples: HashMap<String, Sound>
}


impl Sampler {

    pub fn new () -> Sampler {
        Sampler { samples: HashMap::new() }
    }

    pub fn load (&mut self, name: &str, path: &str) {
        self.samples.insert(name.to_string(), Sound::new(&path));
    }

    pub fn play (&mut self, sample: &str, cue: Option<&str>) {
        match self.samples.get_mut(sample) {
            None => println!("no command {}", &sample),
            Some(sound) => match cue {
                None => sound.play_from_start(),
                Some(cue) => sound.play_from_cue(cue)
            }
        }
    }

}
