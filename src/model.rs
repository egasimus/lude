use std::env;
use std::process::exit;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashMap, BTreeMap};
use std::cmp::min;
use std::time::Instant;
use indexmap::IndexMap;

#[derive(Debug)]
pub struct Event {
    name: String
}

impl Event {
    pub fn new (name: String) -> Event {
        Event { name }
    }
}

#[derive(Debug)]
pub struct Sequence {
    pub events: IndexMap<u128, Event>,
    pub length: u128
}

impl Sequence {
    pub fn new () -> Sequence {
        Sequence { events: IndexMap::new(), length: 0 }
    }
}

pub fn load (reader: BufReader<File>) -> (u128, HashMap<String, Sequence>) {
    let grid: u128 = 234;
    let mut instrument: Option<String> = None;
    let mut sequences: HashMap<String, Sequence> = HashMap::new();
    for (_index, result) in reader.lines().enumerate() {
        let raw = result.expect("could not read line");
        let line = raw.trim_end();
        match &instrument {
            None => {
                match line.chars().next() {
                    None => continue,
                    Some(char) => {
                        if char == ' ' {
                            panic!("E001")
                        } else {
                            instrument = Some(line.to_string())
                        }
                    }
                }
            },
            Some(_) => {
                match line.chars().next() {
                    None => instrument = None,
                    Some(char) => {
                        if char == ' ' {
                            let inst = instrument.as_ref().unwrap();
                            let (name, seq) = parse_line(grid, &line.trim_start());
                            sequences.insert(format!("{}.{}", inst, name), seq);
                        } else {
                            instrument = Some(line.to_string())
                        }
                    }
                }
            }
        }
    }
    (grid, sequences)
}

fn parse_line (grid: u128, line: &str) -> (String, Sequence) {
    let mut name = "";
    let mut i: usize = 0;
    let mut chars = line.chars();
    while i < line.len() {
        let c = chars.next();
        match c {
            None => break,
            Some(c) => {
                if c == '=' {
                    name = &line[0..i].trim_end();
                    break
                }
            },
        }
        i += 1;
    }
    let mut j: u128 = 0;
    let mut seq = Sequence::new();
    while i < line.len() {
        let c = chars.next();
        match c {
            None => break,
            Some(c) => if c == ' ' {
                continue
            } else {
                seq.events.insert(j, Event::new(c.to_string()));
                j += grid;
            }
        }
    }
    seq.length = j + grid;
    (name.to_string(), seq)
}

