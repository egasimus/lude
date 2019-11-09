extern crate pest;
#[macro_use] extern crate pest_derive;

use std::env;
use std::process::exit;
use std::fs::{File, read_to_string};
use std::io::BufReader;

mod model;
mod player;
mod parser;
use crate::player::Player;
use crate::parser::parse;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 { exit(1); }
    let filename = &args[1];
    let source = read_to_string(filename).expect("cannot read file");
    let parsed = parse(&source);
    /*let file = File::open(filename}.unwrap();
    let reader = BufReader::new(file);
    let (grid, sequences, playing) = load(reader);
    let grid_usec = grid * 1000;
    let mut player = Player { grid_usec, sequences, playing };
    println!("{:?}", player);
    player.play();*/
}
