use std::collections::{HashMap, BTreeMap};
use std::time::Instant;
use pest::{Parser, iterators::Pair};
use super::render::sound::get_duration;

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

pub fn eval (parsed: Pair<Rule>) -> Document {
    let start = Instant::now();

    let mut doc = Document::new();

    let mut cursor = 0;

    for statement in parsed.into_inner() {
        for inner in statement.into_inner() {
            match inner.as_rule() {
                Rule::Event => {
                    //eprintln!("Event: {:#?}", &inner.as_str());
                    let event = &inner.as_str();
                    doc.add_event(cursor, event);
                    cursor += doc.get_event_duration(event);
                },
                Rule::Jump => {
                    let time = inner.into_inner().next().unwrap();
                    match time.as_rule() {
                        Rule::Time => {
                            cursor = u128::from_str_radix(
                                time.as_str(),
                                10
                            ).unwrap();
                        },
                        Rule::RelTime => {
                            let mut time = time.into_inner();
                            let rel = time.next().unwrap().as_str();
                            let dur = u128::from_str_radix(
                                time.next().unwrap().as_str(),
                                10
                            ).unwrap();
                            match rel {
                                "+" => { cursor += dur }
                                "-" => { cursor -= dur },
                                _ => panic!("unreachable")
                            }
                        },
                        _ => panic!("unreachable")
                    }
                },
                _ => panic!("not implemented"),
            }
        }
    }

    doc.length = cursor;

    eprintln!("Evaluated in {}usec", start.elapsed().as_micros());

    doc
}

#[derive(Debug)]
pub struct Document {
    pub length: u128,
    pub events: BTreeMap<u128, Vec<String>>,
    pub durations: HashMap<String, u128>,
    max_duration: u128,
}

impl Document {
    pub fn new () -> Document {
        Document {
            length: 0,
            events: BTreeMap::new(),
            durations: HashMap::new(),
            max_duration: 0
        }
    }
    pub fn add_event (&mut self, at: u128, event: &str) {
        let duration = self.get_event_duration(event);
        if duration > self.max_duration {
            self.max_duration = duration
        }
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
        match self.durations.get(event) {
            Some(duration) => *duration,
            None => {
                let duration = get_duration(&event);
                self.durations.insert(event.to_string(), duration);
                duration
            }
        }
    }
    pub fn get_frame (&self, index: u128) -> Vec<f32> {
        let min = *self.events.keys().next_back().unwrap();
        let max = *self.events.keys().next().unwrap();
        if index < min || index > (max + self.max_duration) {
            // nothing before the beginning
            Vec::new()
        } else {
            // maybe something in the middle
            let frame = Vec::new();
            let range = self.events.range(index - self.max_duration..index);
            for (t, events) in range {
                for event in events {
                    let event_end = t + self.get_event_duration(event);
                    if event_end >= index {
                        let u = event_end - index;
                        let event_frame = self.get_event_frame(event, u);
                    }
                }
            }
            frame
        }
    }
    pub fn get_event_frame (&self, event: &str, index: u128) -> Vec<f32> {
        Vec::new()
    }
}
