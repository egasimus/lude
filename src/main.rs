mod sequencer;
mod sampler;
mod engine;

extern crate pest;
#[macro_use] extern crate pest_derive;
#[macro_use] extern crate debug_stub_derive;

use std::env;
use std::process::exit;
use std::fs::read_to_string;

use self::sequencer::{Commands, Sequencer, parser::parse};
use self::sampler::Sampler;
use self::engine::start_engine;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 { exit(1); }
    let filename = &args[1];
    let source = read_to_string(filename).expect("cannot read file");
    let doc = parse(&source);
    println!("{:#?}", &doc);
    let mut sampler = Sampler::new();
    for (name, path) in doc.get_sounds() {
        sampler.load(&name, &path);
    }
    let mut sequencer = Sequencer::new(doc, sampler);
    start_engine(sampler, sequencer);
}
