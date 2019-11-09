use std::env;
use std::process::exit;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashMap, BTreeMap};
use std::cmp::min;
use std::time::Instant;
use indexmap::IndexMap;

#[derive(Debug)]
struct Event {
    name: String
}

impl Event {
    pub fn new (name: String) -> Event {
        Event { name }
    }
}

#[derive(Debug)]
struct Sequence {
    events: IndexMap<u128, Event>
}

impl Sequence {
    pub fn new () -> Sequence {
        Sequence { events: IndexMap::new() }
    }
}

struct Player {
    grid:             u128,
    sequences:        HashMap<String, Sequence>,
    active_sequences: Vec<String>,
    instrument:       Option<String>
}

impl Player {
    pub fn new () -> Player {
        Player {
            grid: 234,
            sequences: HashMap::new(),
            active_sequences: vec![],
            instrument: None
        }
    }

    pub fn load (&mut self, reader: BufReader<File>) {
        for (_index, result) in reader.lines().enumerate() {
            let raw = result.expect("could not read line");
            let line = raw.trim_end();
            match &self.instrument {
                None => {
                    match line.chars().next() {
                        None => continue,
                        Some(char) => {
                            if char == ' ' {
                                panic!("E001")
                            } else {
                                self.instrument = Some(line.to_string())
                            }
                        }
                    }
                },
                Some(_) => {
                    match line.chars().next() {
                        None => self.instrument = None,
                        Some(char) => {
                            if char == ' ' {
                                let inst = self.instrument.as_ref().unwrap();
                                let (name, seq) = self.parse_line(&line.trim_start());
                                self.sequences.insert(format!("{}.{}", inst, name), seq);
                            } else {
                                self.instrument = Some(line.to_string())
                            }
                        }
                    }
                }
            }
        }
        println!("{:?}", &self.sequences)
    }

    fn parse_line (&self, line: &str) -> (String, Sequence) {
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
                    j += self.grid;
                }
            }
        }
        (name.to_string(), seq)
    }

    pub fn play (&mut self) {}
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        exit(1);
    }
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut player = Player::new();
    player.load(reader);
    player.play();
}
