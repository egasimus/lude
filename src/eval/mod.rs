#[cfg(test)]
mod tests;

use std::collections::{HashMap, BTreeMap};
use std::time::Instant;
use pest::{Parser, iterators::{Pair, Pairs}};

#[derive(Parser)]
#[grammar = "./eval/grammar.pest"]
struct DefaultParser;

pub fn read (source: &str) -> Pair<Rule> {
    let start = Instant::now();

    let parsed = DefaultParser::parse(Rule::Doc, source)
        .unwrap_or_else(|e| panic!("{}", e)).next().unwrap();

    eprintln!("Parsed in {}usec", start.elapsed().as_micros());

    parsed
}

#[derive(Debug)]
pub struct Document {
    pub length: u128,
    pub events: BTreeMap<u128, Vec<String>>,
    pub durations: HashMap<String, u128>,
}

impl Document {
    pub fn new () -> Document {
        Document {
            length: 0,
            events: BTreeMap::new(),
            durations: HashMap::new()
        }
    }
    pub fn add_event (&mut self, at: u128, event: &str) {
        let event = event.to_string();
        match self.events.get_mut(&at) {
            Some(events) => events.push(event),
            None => {
                let mut events = Vec::new();
                events.push(event);
                self.events.insert(at, events);
            }
        }
    }
    pub fn get_event_duration (&mut self, event: &str) -> u128 {
        100
    }
}

pub fn eval (parsed: Pair<Rule>) -> Document {
    let start = Instant::now();

    let mut doc = Document::new();

    let mut playhead = 0;

    for statement in parsed.into_inner() {
        for inner in statement.into_inner() {
            match inner.as_rule() {
                Rule::Event => {
                    println!("Event: {:#?}", &inner.as_str());
                    let event = &inner.as_str();
                    doc.add_event(playhead, event);
                    playhead += doc.get_event_duration(event);
                },
                _ => panic!("not implemented"),
            }
        }
    }

    doc.length = playhead;

    eprintln!("Evaluated in {}usec", start.elapsed().as_micros());

    doc
}
