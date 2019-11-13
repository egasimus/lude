mod model;
mod parser;
mod sequencer;
mod sampler;

extern crate pest;
#[macro_use] extern crate pest_derive;
#[macro_use] extern crate debug_stub_derive;

use std::env;
use std::process::exit;
use std::fs::read_to_string;

use crate::sequencer::Sequencer;
use crate::sampler::Sampler;
use crate::parser::parse;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 { exit(1); }
    let filename = &args[1];
    let source = read_to_string(filename).expect("cannot read file");
    let parsed = parse(&source);
    println!("{:#?}", &parsed);
    let mut sampler = Sampler::new();
    sampler.load_from_document(&parsed);
    let mut sequencer = Sequencer::new(parsed, sampler);
    sequencer.play();
    /*let file = File::open(filename}.unwrap();
    let reader = BufReader::new(file);
    let (grid, sequences, playing) = load(reader);
    let grid_usec = grid * 1000;
    let mut player = Player { grid_usec, sequences, playing };
    println!("{:?}", player);
    player.play();*/
}
