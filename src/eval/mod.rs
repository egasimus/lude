use std::cmp::min;
use std::cell::RefCell;
use std::collections::{HashMap, BTreeMap};
use std::time::Instant;
use pest::{Parser, iterators::Pair};
use super::sound::{get_duration, get_frame};

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

pub fn render (doc: &Document, begin: u128, end: u128) -> Vec<Vec<f32>> {
    let start = Instant::now();

    let mut channels = Vec::new();

    for frame in begin..(end+1) {
        //eprintln!("rendering frame {}", &frame);
        channels.push(doc.get_frame(frame));
    }

    eprintln!("Rendered in {}usec", start.elapsed().as_micros());

    channels
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
    pub durations: RefCell<HashMap<String, u128>>,
    longest: u128,
}

impl Document {
    pub fn new () -> Document {
        Document {
            length: 0,
            events: BTreeMap::new(),
            durations: RefCell::new(HashMap::new()),
            longest: 0
        }
    }
    pub fn add_event (&mut self, at: u128, event: &str) {
        let duration = self.get_event_duration(event);
        if duration > self.longest {
            self.longest = duration
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
    pub fn get_event_duration (&self, event: &str) -> u128 {
        let mut durations = self.durations.borrow_mut();
        match durations.get(event) {
            Some(duration) => *duration,
            None => {
                let duration = get_duration(&event);
                durations.insert(event.to_string(), duration);
                duration
            }
        }
    }
    pub fn get_frame (&self, frame_index: u128) -> Vec<f32> {
        // nothing if document is empty
        if self.events.len() == 0 { return Vec::new() }

        // determine real bounds of document
        let min = *self.events.keys().next_back().unwrap();
        let max = *self.events.keys().next().unwrap();
        let longest = self.longest;
        let end = max + longest;

        // nothing before the beginning or after the end
        if frame_index < min || frame_index > end { return Vec::new() }

        // maybe something in the middle?
        let mut frame: Vec<f32> = Vec::new();
        let mut event_frames = Vec::new();
        let start = if frame_index < longest { 0 } else { frame_index - longest };
        let event_range = self.events.range(start..frame_index);
        for (event_start, events) in event_range {
            let event_frame_index = frame_index - event_start;
            for event in events {
                match get_frame(event, event_frame_index as i64) {
                    Some(frame) => event_frames.push(frame),
                    _ => {}
                }
            }
        }
        for event_frame in event_frames.iter() {
            for (i, value) in event_frame.iter().enumerate() {
                match frame.get_mut(i) {
                    Some(&mut mut channel) => channel += value,
                    None => frame.push(*value)
                }
            }
        }
        frame
    }
}
