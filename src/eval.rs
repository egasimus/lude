use crate::document::Document;
use crate::types::{FrameTime, SliceType};
use std::cell::RefCell;
use std::collections::HashMap;
use std::time::Instant;
use pest::{Parser, iterators::Pair};

#[derive(Parser)]
#[grammar = "./grammar.pest"]
struct DefaultParser;

/// The **source code** of a `Document` is **parsed** by
/// [Pest](https://pest.rs), returning a collection of **statements**.
/// A document contains zero or more **statements**, separated by **whitespace**.
/// * **TODO** Make whitespace less significant.
///
/// ### Writing comments.
/// Things between `(` and `)` are ignored.
/// You can use this to describe things for humans.
pub fn read (source: &str) -> Pair<Rule> {
    let start = Instant::now();
    let parsed = DefaultParser::parse(Rule::Doc, source)
        .unwrap_or_else(|e| panic!("{}", e)).next().unwrap();
    eprintln!("parsed in {}usec ", start.elapsed().as_micros());
    parsed
}

/// A collection of **statements** is **evaluated**, returning a `Document` -
/// a full, unambiguous description of what **slices** should be
/// **rendered** to the **output**.
///
/// ### Time
/// During evaluation, a **cursor** points to the current **time**.
/// Time is measured in **frames**, represented by an **unsigned integer**.
/// Frames correspond to the **output sample rate** (currently hardcoded
/// at 44100 Hz)
///
/// * **TODO** Measure time in [flicks](https://en.wikipedia.org/wiki/Flick_(time)).
/// * **TODO** Index time from 1 instead of 0
/// * **TODO** Allow custom units of time to be defined.
/// * **TODO** Allow output sample rate to be set.
///
/// Stating one of the following commands moves the cursor:
///
/// * The **jump** command (`@NUMBER`) sets the cursor to `NUMBER`.
/// * The **skip** command (`@+NUMBER`) moves the cursor forward by `NUMBER`.
/// * The **back** command (`@-NUMBER`) mobes the cursor back by `NUMBER`.
/// * The **sync** command gives a **name** to the current value of the cursor,
///   so that you can reference a point in time by a name rather than a number.
///   It is equivalent ot an alias (see below)
/// * **TODO** use `/` and `*` for speeding up/slowing down
/// * **TODO** add something for repetition
///
/// ### Source
/// Stating a **path** to a **source** makes that source **active**.
///
/// * **TODO** Paths are evaluated relative to the location of the source file.
///
///
/// ### Name
/// Assignment is of the form `NAME = [CONTENT]`.
/// Afterwards, writing `NAME` is equivalent to writing `CONTENT`.
///
/// ### Command
/// **TODO** use `!` for commands to the renderer (such as setting sample rate,
/// mixing algorithm, etc)
///
/// ### Alter
/// **TODO**
pub fn eval (parsed: Pair<Rule>) -> Document {
    let start = Instant::now();
    let evaluator = Eval::new(parsed);
    let doc = evaluator.run();
    eprintln!("evaluated in {}usec ", start.elapsed().as_micros());
    doc
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
            match statement.as_rule() {
                Rule::Comment => {},
                Rule::Jump   => self.jump(statement.into_inner().next().unwrap()),
                Rule::Skip   => self.skip(statement.into_inner().next().unwrap()),
                Rule::Back   => self.back(statement.into_inner().next().unwrap()),
                Rule::Sync   => self.sync(statement.into_inner().next().unwrap()),
                Rule::Source => self.source(statement),
                Rule::Slice  => self.slice(statement),
                Rule::Assign => self.assign(statement),
                Rule::Alias  => self.alias(statement),
                _ => unreachable!(),
            };
        }
        let mut doc = self.doc.replace(Document::new()); // how do i drop
        doc.length = *self.cursor.borrow();
        doc
    }
    fn jump (&self, time: Pair<Rule>) {
        let time = pair_to_frame_time(time);
        self.cursor.replace(time);
    }
    fn skip (&self, time: Pair<Rule>) {
        let time = pair_to_frame_time(time);
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
                Rule::SliceStart => slice_start = Some(pair_to_frame_time(pair)),
                Rule::SliceEnd => slice_end = Some(pair_to_frame_time(pair)),
                Rule::SliceAbs => slice_type = SliceType::Abs,
                Rule::SliceFwd => slice_type = SliceType::Fwd,
                Rule::SliceRew => slice_type = SliceType::Rew,
                _ => unreachable!()
            }
        }
        let cursor = *self.cursor.borrow();
        let advance = self.doc.borrow_mut().write(
            cursor, &self.source.borrow(), slice_type, slice_start, slice_end
        );
        self.cursor.replace_with(|cursor| *cursor + advance);
    }
    fn assign (&self, _assign: Pair<Rule>) {
        panic!("not implemented")
    }
    fn alias (&self, _name: Pair<Rule>) {
        panic!("not implemented")
    }
}

fn pair_to_frame_time (pair: Pair<Rule>) -> FrameTime {
    FrameTime::from_str_radix(pair.as_str(), 10).unwrap()
}
