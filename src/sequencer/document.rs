use std::collections::HashMap;
use super::command::{Command, Commands};
use crate::timeline::Timeline;

pub type Sequence = Timeline<String>;

#[derive(Debug)]
pub struct Document {
    pub definitions: HashMap<String, Command>,
    pub sequences: HashMap<String, Sequence>,
}

impl Document {
    pub fn new () -> Document {
        let definitions = HashMap::new();
        let sequences = HashMap::new();
        Document { definitions, sequences }
    }

    pub fn define (&mut self, key: &str, val: Command) {
        self.definitions.insert(key.to_string(), val);
    }

    pub fn sequence (&mut self, key: &str, val: Sequence) {
        self.sequences.insert(key.to_string(), val);
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
}

