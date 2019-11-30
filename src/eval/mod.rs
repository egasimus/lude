use std::cell::RefCell;
use std::collections::{HashMap, BTreeMap};
use std::time::Instant;
use pest::{Parser, iterators::{Pair, Pairs}};
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
    let evaluator = Eval::new(parsed);
    let doc = evaluator.run();
    eprintln!("Evaluated in {}usec", start.elapsed().as_micros());
    doc
}

// lifetime is enforced by Pest
struct Eval<'i> {
    parsed: RefCell<Option<Pair<'i, Rule>>>,
    doc:    RefCell<Document>,
    cursor: RefCell<FrameTime>,
}

impl Eval<'_> {
    pub fn new (parsed: Pair<Rule>) -> Eval {
        Eval {
            parsed: RefCell::new(Some(parsed)),
            doc: RefCell::new(Document::new()),
            cursor: RefCell::new(0)
        }
    }
    pub fn run (&self) -> Document {
        let parsed = self.parsed.replace(None).unwrap();
        for statement in parsed.into_inner() {
            for inner in statement.into_inner() {
                match inner.as_rule() {
                    Rule::Event => self.event(inner),
                    Rule::Jump => self.jump(inner),
                    _ => panic!("not implemented"),
                };
            }
        }
        let mut doc = self.doc.replace(Document::new()); // how do i drop
        doc.length = *self.cursor.borrow();
        doc
    }
    fn event (&self, event: Pair<Rule>) {
        eprintln!("!!! Event: {:#?}", &event.as_str());
        let mut event = event.into_inner();
        let name = event.next().unwrap().as_str();
        let slice: Option<(FrameTime, FrameTime)> = match event.next() {
            Some(event) => { println!("-----------slice:{:#?}",&event); None }
            _ => None
        };
        let cursor = *self.cursor.borrow();
        let advance = self.doc.borrow_mut().add_event(cursor, &name);
        self.cursor.replace(cursor + advance);
    }
    fn jump (&self, jump: Pair<Rule>) {
        let time = jump.into_inner().next().unwrap();
        match time.as_rule() {
            Rule::Time => self.jump_abs(time),
            Rule::RelTime => self.jump_rel(time),
            _ => unreachable!()
        }
    }
    fn jump_abs (&self, time: Pair<Rule>) {
        eprintln!("Rule::Time={}", time.as_str());
        let time = FrameTime::from_str_radix(
            time.as_str().to_string().trim(),
            10
        ).unwrap();
        self.cursor.replace(time);
    }
    fn jump_rel (&self, time: Pair<Rule>) {
        eprintln!("Rule::RelTime={}", time.as_str());
        let mut time = time.into_inner();
        let direction = time.next().unwrap().as_str();
        let duration = FrameTime::from_str_radix(
            time.next().unwrap().as_str(),
            10
        ).unwrap();
        match direction {
            "+" => { self.cursor.replace_with(|cursor| *cursor + duration) }
            "-" => { self.cursor.replace_with(|cursor| *cursor - duration) }
            _ => unreachable!()
        };
    }
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
    pub fn add_event (&mut self, at: FrameTime, event: &str) -> FrameTime {
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
        duration
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
    let mut frame: Frame = Vec::new();
    for event_frame in event_frames.iter() {
        for (i, value) in event_frame.iter().enumerate() {
            match frame.get_mut(i) {
                Some(&mut current) => {frame[i] = current + value;},
                None => frame.push(*value)
            }
        }
    }
    if frame.len() > 0 {
        Some(frame)
    } else {
        None
    }
}
