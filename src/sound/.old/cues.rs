use std::sync::{Arc, Mutex};
use std::thread::{Builder as ThreadBuilder, JoinHandle};
use super::sndfile::{SndFile, OpenMode};
use indexmap::IndexMap;

type Shared<T> = Arc<Mutex<T>>;
type LabelMap = IndexMap<String, usize>;
type CueMap<T> = IndexMap<usize, Vec<T>>;

#[derive(Debug)]
pub struct CuePointManager {
    labels: LabelMap,
    buffers: CueMap<f32>,
    thread: JoinHandle<()>,
}

impl CuePointManager {
    pub fn new (path: &str) -> CuePointManager {
        let mut labels = IndexMap::new();
        let mut buffers = IndexMap::new();
        let mut file = SndFile::new(path, OpenMode::Read).unwrap();
        let thread = ThreadBuilder::new()
            .name("cue reader".into())
            .spawn(move || { /*loop {
                // Thread immediately blocks upon start.
                cues.lock();
                // When the mutex is unlocked from the main thread,
                // this means the list of cues has been updated.
                // Reload cue buffers. TODO: reload only updated ones.
                for cue in cues.get_mut().iter_keys() {
                    file.seek(cue.position as i64, SeekMode::SeekSet);
                    let buffer: &mut[f32] = &mut Vec::with_capacity(N_CUE_BUFFER_FRAMES);
                    file.readf_f32(buffer, N_CUE_BUFFER_FRAMES as i64);
                    cue.buffer = buffer.to_vec();
                }*/
            }).unwrap();
        CuePointManager { labels, buffers, thread }
    }
    
    pub fn set (&mut self, label: &str, position: usize) {
        self.labels.insert(label.to_string(), position);
    }

    pub fn get_position (&self, label: &str) -> usize { 0 }

}
