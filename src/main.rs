use std::env;
use std::process::exit;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashMap, BTreeMap};
use std::cmp::min;
use std::time::Instant;
use indexmap::IndexMap;

mod model;
use crate::model::{load, Sequence};

#[derive(Debug)]
struct Player {
    grid:             u128,
    sequences:        HashMap<String, Sequence>,
    active_sequences: Vec<String>,
}

impl Player {
    pub fn new (grid: u128, sequences: HashMap<String, Sequence>) -> Player {
        Player { grid, sequences, active_sequences: vec![] }
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
    let (grid, sequences) = load(reader);
    let mut player = Player::new(grid, sequences);
    println!("{:?}", player);
    player.play();
}
