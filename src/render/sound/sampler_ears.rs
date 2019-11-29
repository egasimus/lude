use std::collections::HashMap;
use crate::{parser::Document, model::Commands};
use ears::{Sound as EarsSound, AudioController};

#[derive(DebugStub)]
struct Sound {
    #[debug_stub="SomeSamples"]
    sound: EarsSound
}

#[derive(Debug)]
pub struct Sampler {
    samples: HashMap<String, Sound>
}

impl Sampler {
    pub fn new () -> Sampler {
        Sampler { samples: HashMap::new() }
    }

    pub fn load_from_document (&mut self, doc: &Document) {
        for (name, command) in doc.definitions.iter() {
            match command.name {
                Commands::Sound => {
                    let path = command.args.get(0).unwrap();
                    let sound = EarsSound::new(path).expect("oops");
                    self.samples.insert(name.to_string(), Sound { sound });
                }
                _ => {}
            }
        }
    }

    pub fn play (&mut self, name: &str) {
        match self.samples.get_mut(name) {
            None => eprintln!("no command {}", &name),
            Some(sound) => sound.sound.play()
        }
    }
}
