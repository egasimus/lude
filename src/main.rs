mod sequencer;
mod sampler;
mod engine;
mod timeline;

extern crate pest;
#[macro_use] extern crate pest_derive;
#[macro_use] extern crate debug_stub_derive;

use std::env;
use std::process::exit;
use std::fs::read_to_string;

use self::sequencer::parser::parse;
use self::sampler::Sampler;
use self::engine::start_jack_engine;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 { exit(1); }
    let filename = &args[1];
    let source = read_to_string(filename).expect("cannot read file");
    let document = parse(&source);
    println!("{:#?}", &document);
    let mut sampler = Sampler::new();
    for (name, path) in document.get_sounds() {
        sampler.load(&name, &path);
    }
    start_jack_engine(document, sampler);
}
