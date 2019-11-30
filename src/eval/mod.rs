use std::cell::RefCell;
use std::collections::{HashMap, BTreeMap};
use std::time::Instant;
use std::mem::replace;
use pest::{Parser, iterators::Pair};
use super::sound::SoundMap;

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

// these should be polymorphic:
type Sample = f32;
type Frame = Vec<Sample>;
type FrameTime = u128;

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
                            eprintln!("Rule::Time={}", time.as_str());
                            let time = FrameTime::from_str_radix(
                                time.as_str().to_string().trim(),
                                10
                            ).unwrap();
                            cursor = time;
                        },
                        Rule::RelTime => {
                            eprintln!("Rule::RelTime={}", time.as_str());
                            let mut time = time.into_inner();
                            let rel = time.next().unwrap().as_str();
                            let dur = FrameTime::from_str_radix(
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
    media: SoundMap,
    pub length: FrameTime,
    pub events: BTreeMap<FrameTime, Vec<String>>,
    pub durations: RefCell<HashMap<String, FrameTime>>,
    longest: FrameTime,
}

impl Document {
    pub fn new () -> Document {
        Document {
            media: SoundMap::new(),
            length: 0,
            events: BTreeMap::new(),
            durations: RefCell::new(HashMap::new()),
            longest: 0
        }
    }
    pub fn add_event (&mut self, at: FrameTime, event: &str) {
        eprintln!("add_event {} {}", &at, &event);
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
    pub fn get_event_duration (&mut self, event: &str) -> FrameTime {
        let mut durations = self.durations.borrow_mut();
        match durations.get(event) {
            Some(duration) => *duration,
            None => {
                let duration = self.media.get_duration(&event);
                durations.insert(event.to_string(), duration);
                duration
            }
        }
    }
    pub fn get_frame (&self, frame_index: u128) -> Option<Frame> {
        // nothing if document is empty
        if self.events.len() == 0 { return None }

        // determine real bounds of document
        let min = *self.events.keys().next_back().unwrap();
        let max = *self.events.keys().next().unwrap();
        let longest = self.longest;
        let end = max + longest;

        // nothing before the beginning or after the end
        if frame_index < min || frame_index > end { return None }

        // maybe something in the middle?
        let mut event_frames = Vec::new();
        let start = if frame_index < longest { 0 } else { frame_index - longest };
        let event_range = self.events.range(start..frame_index+1);
        for (event_start, events) in event_range {
            let event_frame_index = frame_index - event_start;
            for event in events {
                match self.media.get_frame(event, event_frame_index as i64) {
                    Some(frame) => event_frames.push(frame),
                    _ => {}
                }
            }
        }
        sum_event_frames(event_frames)
    }
}

pub fn render (
    doc: &Document,
    begin: FrameTime,
    end: FrameTime
) -> Vec<Option<Frame>> {
    let start = Instant::now();

    let mut frames = Vec::new();

    for index in begin..(end+1) {
        let frame = doc.get_frame(index);
        frames.push(frame);
    }

    eprintln!("Rendered {}..{} in {}usec",
        &begin, &end, start.elapsed().as_micros());

    frames
}


fn sum_event_frames (event_frames: Vec<Frame>) -> Option<Frame> {
    print!("sum_event_frames {:?}", &event_frames);
    let mut frame: Frame = Vec::new();
    for event_frame in event_frames.iter() {
        for (i, value) in event_frame.iter().enumerate() {
            print!("<-{}:{:?}::{:?}", &i, &value, &frame.get(i));
            match frame.get_mut(i) {
                Some(&mut current) => {frame[i] = current + value;},
                None => frame.push(*value)
            }
        }
    }
    println!("={:?}", &frame);
    if frame.len() > 0 {
        Some(frame)
    } else {
        None
    }
}
