use std::collections::HashMap;
use crate::{timeline::Timeline, resource::{Sampler, Sequencer}};

#[derive(Debug)]
pub struct Document {
    sampler: Sampler,
    sequencer: Sequencer,
    definitions: Definitions,
    sequences: Sequences,
}

impl Document {
    pub fn new () -> Document {
        Document {
            sampler: Sampler::new(),
            resources: Resources::new(),
            definitions: Definitions::new(),
            sequences: Sequences::new()
        }
    }

    pub fn add_resource (&mut self, key: &str, val: Resource) {}

    pub fn add_definition (&mut self, key: &str, val: Command) {
        self.definitions.insert(key.to_string(), val);
    }

    pub fn add_sequence (&mut self, key: &str, val: Sequence) {
        self.sequences.insert(key.to_string(), val);
    }
    
    pub fn load_resources (&mut self) {
        for (name, path) in self.get_sounds() {
            self.sampler.load(&name, &path);
        }
    }

    pub fn get_sounds (&self) -> HashMap<String, String> {
        let sounds = self.definitions.iter().filter_map(|(name, command)| {
            match command.name {
                _ => None,
                Commands::Sound => Some((
                    name.to_string(),
                    command.args.get(0).unwrap().to_string()
                ))
            }
        });
        let sounds_map: HashMap<_, _> = sounds.collect();
        sounds_map
    }

    pub fn get_frame (&self, start: u32, size: u32) -> (Vec<Vec<(String, usize)>>, bool) {
        // ok so how will this work...
        // each sequence is split into a number of equal steps
        // that have length of at least 1 sample
        // so for each sample of the requested frame
        // this function should return a list of tuples (sound, offset)
        // based on the currently active sequences
        // and if the offset is past of the end of the sound
        // simply play nothing (or don't return that tuple at all?)
        // some coupling between document and sampler might be in order?
        // or not - just pass the list of what to play to the sampler
        // and it ignores offsets past sample ends...
        //
        // start with main sequence and recursively descend into
        // child sequences/sounds ... a sound is just a sequence of samples
        // how will sounds be resampled?
        (Vec::new(), false)
    }
}

/*
use std::time::Instant;
use self::parser::Document;
use crate::sampler::Sampler;

#[derive(Debug)]
pub struct Sequencer {
    grid_usec: u128,
    document: Document,
    sampler: Sampler,
    started: Option<Instant>,
    last_step: Option<u128>
}

impl Sequencer {

    pub fn new (document: Document, sampler: Sampler) -> Sequencer {
        Sequencer {
            grid_usec: 234000,
            started:   None,
            last_step: None,
            document,
            sampler,
        }
    }

    pub fn play (&mut self) {
        self.started = Some(Instant::now());
        loop {
            match self.started {
                Some(_) => self.tick(),
                None => break
            }
        }
    }

    pub fn stop (&mut self) {
        self.started = None
    }

    fn get_step (&self) -> (u128, u128) {
        let elapsed = self.started.unwrap().elapsed().as_micros();
        let step    = elapsed / self.grid_usec;
        let jitter  = elapsed % self.grid_usec;
        (step, jitter)
    }

    fn tick (&mut self) {
        let (step, jitter) = self.get_step();
        match self.last_step {
            None => self.step(step, jitter),
            Some(last_step) => if step > last_step {
                self.step(step, jitter)
            }
        }
    }

    fn step (&mut self, step: u128, jitter: u128) {
        self.last_step = Some(step);
        let main_sequence = self.document.sequences.get("<main>").unwrap();
        print!("{}+{}us", &step, &jitter);
        match main_sequence.get(step) {
            None => {},
            Some(s) => {
                for event in s {
                    self.sampler.play(event.name.as_str(), None);
                    print!("{:#?}", &event.name);
                }
            }
        }
        eprintln!("");
    }

}
*/
