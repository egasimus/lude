use std::collections::HashMap;
use crate::{parser::Document, model::Commands, sndfile::{SndFile, OpenMode}};
use jack::RingBuffer;

const BUFFER_FRAMES: usize = 1 << 14; // 16384

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
                    let sound = Sound::new(&path);
                    self.samples.insert(name.to_string(), sound);
                }
                _ => {}
            }
        }
    }

    pub fn play (&mut self, name: &str) {
        match self.samples.get_mut(name) {
            None => println!("no command {}", &name),
            Some(sound) => sound.play_from_start()
        }
    }
}

#[derive(Debug)]
struct CuePoint {
    position: usize,
    label: Option<String>,
    buffer: Vec<u8>
}

#[derive(DebugStub)]
struct Sound {
    path: String,
    loading_state: LoadingState,
    playback_state: PlaybackState,
    playback_position: usize,
    cues: Vec<CuePoint>,
    next_cue: Option<usize>,
    #[debug_stub="SndFile"]
    streaming_file: SndFile,
    #[debug_stub="SndFile"]
    preloading_file: SndFile,
    #[debug_stub="RingBuffer"]
    playback_buffer: RingBuffer
}

#[derive(Debug)]
pub enum LoadingState {
    Initial,
    Loading,
    Done
}

#[derive(Debug)]
pub enum PlaybackState {
    Stopped,
    Playing
}

impl Sound {
    pub fn new (path: &str) -> Sound {
        Sound {
            path: path.to_string(),

            streaming_file: SndFile::new(path, OpenMode::Read).unwrap(),
            preloading_file: SndFile::new(path, OpenMode::Read).unwrap(),
            loading_state: LoadingState::Initial,

            cues: Vec::new(),
            next_cue: None,

            playback_state: PlaybackState::Stopped,
            playback_position: 0,
            playback_buffer: RingBuffer::new(BUFFER_FRAMES).unwrap_or_else(
                |_| panic!("failed to allocate ringbuffer: {}"))
        }
    }

    pub fn cue_set (&mut self, index: usize, position: usize) {
        let cue = CuePoint { position, label: None, buffer: Vec::new() };
        self.cues[index] = cue;
    }

    pub fn cue_jump (&mut self, cue: usize) {
        self.next_cue = Some(cue);
        self.playback_position = self.cues.get(cue).unwrap().position;
    }

    pub fn play_from_start (&mut self) {
        self.playback_position = 0;
        self.playback_state = PlaybackState::Playing;
    }

    pub fn play_from_cue (&mut self, cue: usize) {
        self.cue_jump(cue);
        self.playback_state = PlaybackState::Playing;
    }

    pub fn stop (&mut self) {
        self.playback_state = PlaybackState::Stopped;
    }
}

/*
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
        println!("load {}", path.to_string());
        self.source = Some(path.to_string());
        self.sound  = Some(Sound::new(path).expect("failed to load sound"));
    }

    pub fn play (&mut self, from: &str) {
        println!("play from {}", from.to_string());
        let mut sound = self.sound.as_mut().expect("wat");
        sound.play()
    }
}

use std::collections::HashMap;
pub type DeckMap = Vec<Deck>;
*/
