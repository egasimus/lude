use std::sync::{Arc, Mutex};
use std::thread::{Builder as ThreadBuilder, JoinHandle};
use jack::RingBuffer;

use super::{
    N_CUE_BUFFER_FRAMES,
    cues::CuePointManager, sndfile::{SndFile, OpenMode}
};

type Shared<T> = Arc<Mutex<T>>;

#[derive(DebugStub)]
pub struct Sound {
    path: String,
    cues: CuePointManager,
    next_cue: Option<usize>,

    #[debug_stub="SndFile"]
    playback_file: SndFile,
    playback_thread: JoinHandle<()>,
    loading_state: LoadingState,
    playback_state: PlaybackState,
    playback_position: usize,
    #[debug_stub="RingBuffer"]
    playback_buffer: Shared<RingBuffer>
}

impl Sound {
    pub fn new (path: &str) -> Sound {

        let playback_file = SndFile::new(path, OpenMode::Read).unwrap();
        let playback_thread = ThreadBuilder::new()
            .name("playback reader".into())
            .spawn(|| {}).unwrap();
        let playback_buffer = Arc::new(Mutex::new(
            RingBuffer::new(N_CUE_BUFFER_FRAMES).unwrap_or_else(
                |_| panic!("failed to allocate ringbuffer: {}"))));

        Sound {
            path: path.to_string(),
            loading_state: LoadingState::Initial,

            cues: CuePointManager::new(path),
            next_cue: None,

            playback_file,
            playback_thread,
            playback_buffer,
            playback_state: PlaybackState::Stopped,
            playback_position: 0,
        }
    }

    pub fn cue_set (&mut self, label: &str, position: usize) {
        self.cues.set(label, position);
    }

    pub fn play_from_start (&mut self) {
        self.playback_position = 0;
        self.playback_state = PlaybackState::Playing;
    }

    pub fn play_from_cue (&mut self, cue: &str) {
    }

    pub fn read_playback (&mut self) {
        // okay let's see what's going on here
        // C source was too tangled, what with all the threads
        // still need to figure out the multithreaded control flow
        // current model seems to be N(cues)+1 OS threads per sound
        // let's not reader everything into memory
        // ears also has a streaming reader (struct Music)?
        // maybe we can implement preloaded cue points into that?
    }

    pub fn stop (&mut self) {
        self.playback_state = PlaybackState::Stopped;
    }

    pub fn is_playing (&mut self) -> bool {
        match self.playback_state {
            PlaybackState::Playing => true,
            _ => false
        }
    }

    pub fn read_advance (&mut self, frame_index: u32) -> f32 { 0.0 }
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
