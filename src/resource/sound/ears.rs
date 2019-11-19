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

/*
pub enum LoadState {
    Initial,
    Loading,
    Done
}

pub enum PlayState {
    Stopped,
    Playing
}

pub struct CuePoint {
    position: u64,
}

pub struct Deck {
    pub label:  String,
    pub sound:  Option<Sound>,
    pub source: Option<String>,
    /*load_state: LoadState,
    play_state: PlayState,
    playhead:   u64,
    cues:       Vec<CuePoint>,*/
}

impl Deck {
    pub fn new (label: &str) -> Deck {
        Deck {
            label:       label.to_string(),
            sound:       None,
            source:      None,
            /*load_state:  LoadState::Initial,
            play_state:  PlayState::Stopped,
            position:    0,
            cues:        Vec::new()*/
        }
    }

    pub fn load (&mut self, path: &str) {
        eprintln!("load {}", path.to_string());
        self.source = Some(path.to_string());
        self.sound  = Some(Sound::new(path).expect("failed to load sound"));
    }

    pub fn play (&mut self, from: &str) {
        eprintln!("play from {}", from.to_string());
        let mut sound = self.sound.as_mut().expect("wat");
        sound.play()
    }
}

use std::collections::HashMap;
pub type DeckMap = Vec<Deck>;
*/
