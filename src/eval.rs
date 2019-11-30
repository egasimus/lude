use crate::document::Document;
use crate::types::FrameTime;
use std::cell::RefCell;
use std::time::Instant;
use pest::{Parser, iterators::Pair};

#[derive(Parser)]
#[grammar = "./grammar.pest"]
struct DefaultParser;

pub fn read (source: &str) -> Pair<Rule> {
    let start = Instant::now();
    let parsed = DefaultParser::parse(Rule::Doc, source)
        .unwrap_or_else(|e| panic!("{}", e)).next().unwrap();
    eprintln!("parsed in {}usec ", start.elapsed().as_micros());
    parsed
}

pub fn eval (parsed: Pair<Rule>) -> Document {
    let start = Instant::now();
    let evaluator = Eval::new(parsed);
    let doc = evaluator.run();
    eprintln!("evaluated in {}usec ", start.elapsed().as_micros());
    doc
}

// lifetime is enforced by Pest
struct Eval<'i> {
    parsed: RefCell<Option<Pair<'i, Rule>>>,
    doc: RefCell<Document>,
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
    fn jump (&self, jump: Pair<Rule>) {
        let time = jump.into_inner().next().unwrap();
        match time.as_rule() {
            Rule::Time => self.jump_abs(time),
            Rule::RelTime => self.jump_rel(time),
            _ => unreachable!()
        }
    }
    fn jump_abs (&self, time: Pair<Rule>) {
        let time = FrameTime::from_str_radix(
            time.as_str().to_string().trim(),
            10
        ).unwrap();
        self.cursor.replace(time);
    }
    fn jump_rel (&self, time: Pair<Rule>) {
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
