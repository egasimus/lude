use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use indexmap::IndexMap;

pub type Duration = u128;

#[derive(Debug)]
pub struct Event {
    pub name: String
}

impl Event {
    pub fn new (name: String) -> Event {
        Event { name }
    }
}

#[derive(Debug)]
pub struct Sequence {
    pub events: IndexMap<u128, Vec<Event>>,
    pub length: u128
}

impl Sequence {
    pub fn new () -> Sequence {
        Sequence { events: IndexMap::new(), length: 0 }
    }

    pub fn add (&mut self, index: u128, event: &str) {
        match self.events.get_mut(&index) {
            Some(events) => {
                events.push(Event::new(event.to_string()))
            }
            None => {
                let mut events = Vec::new();
                events.push(Event::new(event.to_string()));
                self.events.insert(index, events);
            }
        }
    }
}
