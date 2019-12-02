use crate::document::Document;
use crate::types::FrameTime;
use std::cell::RefCell;
use std::collections::HashMap;
use std::time::Instant;
use pest::{Parser, iterators::Pair};

#[derive(Parser)]
#[grammar = "./grammar.pest"]
struct DefaultParser;

/// Parses a string using Pest according to the grammar.
pub fn read (source: &str) -> Pair<Rule> {
    let start = Instant::now();
    let parsed = DefaultParser::parse(Rule::Doc, source)
        .unwrap_or_else(|e| panic!("{}", e)).next().unwrap();
    eprintln!("parsed in {}usec ", start.elapsed().as_micros());
    parsed
}

/// Evaluates a parse result into a document.
pub fn eval (parsed: Pair<Rule>) -> Document {
    let start = Instant::now();
    let evaluator = Eval::new(parsed);
    let doc = evaluator.run();
    eprintln!("evaluated in {}usec ", start.elapsed().as_micros());
    doc
}

enum SliceType {
    Full,
    Abs,
    Fwd,
    Rew
}

/// Handles evaluation state.
// 'i lifetime marker is required by Pest.
struct Eval<'i> {
    parsed:  RefCell<Option<Pair<'i, Rule>>>,
    doc:     RefCell<Document>,
    cursor:  RefCell<FrameTime>,
    source:  RefCell<String>,
    markers: RefCell<HashMap<String, FrameTime>>
}

impl Eval<'_> {
    pub fn new (parsed: Pair<Rule>) -> Eval {
        Eval {
            parsed:  RefCell::new(Some(parsed)),
            doc:     RefCell::new(Document::new()),
            cursor:  RefCell::new(0),
            source:  RefCell::new(String::new()),
            markers: RefCell::new(HashMap::new())
        }
    }
    pub fn run (&self) -> Document {
        let parsed = self.parsed.replace(None).unwrap();
        for statement in parsed.into_inner() {
            for inner in statement.into_inner() {
                match inner.as_rule() {
                    Rule::Comment => {},
                    Rule::Jump => self.jump(inner.into_inner().next().unwrap()),
                    Rule::Skip => self.skip(inner.into_inner().next().unwrap()),
                    Rule::Back => self.back(inner.into_inner().next().unwrap()),
                    Rule::Sync => self.sync(inner.into_inner().next().unwrap()),
                    Rule::Source => self.source(inner),
                    Rule::Slice => self.slice(inner),
                    Rule::Assign => self.assign(inner),
                    Rule::Alias => self.alias(inner),
                    _ => unreachable!(),
                };
            }
        }
        let mut doc = self.doc.replace(Document::new()); // how do i drop
        doc.length = *self.cursor.borrow();
        doc
    }
    fn jump (&self, time: Pair<Rule>) {
        let time = FrameTime::from_str_radix(
            time.as_str().to_string().trim(),
            10
        ).unwrap();
        self.cursor.replace(time);
    }
    fn skip (&self, time: Pair<Rule>) {
        let time = FrameTime::from_str_radix(
            time.as_str().to_string().trim(),
            10
        ).unwrap();
        self.cursor.replace_with(|cursor| *cursor + time);
    }
    fn back (&self, time: Pair<Rule>) {
        let time = FrameTime::from_str_radix(
            time.as_str().to_string().trim(),
            10
        ).unwrap();
        self.cursor.replace_with(|cursor| *cursor - time);
    }
    fn sync (&self, name: Pair<Rule>) {
        let name = name.as_str().to_string();
        self.markers.borrow_mut().insert(name, *self.cursor.borrow());
    }
    fn source (&self, path: Pair<Rule>) {
        let path = path.as_str().to_string();
        self.source.replace(path);
    }
    fn slice (&self, slice: Pair<Rule>) {
        let mut slice_type  = SliceType::Full;
        let mut slice_start = None;
        let mut slice_end   = None;
        for pair in slice.into_inner().flatten() {
            match pair.as_rule() {
                Rule::SliceStart => slice_start = pair_to_frame_time(pair),
                Rule::SliceEnd => slice_end = pair_to_frame_time(pair),
                Rule::SliceAbs => slice_type = SliceType::Abs,
                Rule::SliceFwd => slice_type = SliceType::Fwd,
                Rule::SliceRew => slice_type = SliceType::Rew,
                _ => unreachable!()
            }
        }
        let cursor = *self.cursor.borrow();
        let advance = self.doc.borrow_mut().add_event(
            cursor, &self.source.borrow(), slice_start, slice_end
        );
        self.cursor.replace_with(|cursor| *cursor + advance);
    }
    fn assign (&self, _assign: Pair<Rule>) {
        panic!("not implemented")
    }
    fn alias (&self, _name: Pair<Rule>) {
        panic!("not implemented")
    }
    fn event (&self, event: Pair<Rule>) {
        let mut name = "";
        let mut slice_start = None;
        let mut slice_end = None;
        for pair in event.into_inner().flatten() {
            match pair.as_rule() {
                Rule::Path => name = pair.as_str(),
                Rule::SliceStart => slice_start = Some(
                    FrameTime::from_str_radix(pair.as_str(), 10).unwrap()
                ),
                Rule::SliceEnd => slice_end = Some(
                    FrameTime::from_str_radix(pair.as_str(), 10).unwrap()
                ),
                _ => unreachable!()
            }
        }
        let cursor = *self.cursor.borrow();
        let advance = self.doc.borrow_mut().add_event(
            cursor, &name, slice_start, slice_end
        );
        self.cursor.replace(cursor + advance);
    }
}

fn pair_to_frame_time (pair: Pair<Rule>) -> Option<FrameTime> {
    Some(FrameTime::from_str_radix(pair.as_str(), 10).unwrap())
}
