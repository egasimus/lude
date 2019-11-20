mod timeline;

mod document;
pub use document::Document;

mod resource;

mod parser;
pub use parser::parse;

mod engines;
use self::engines::start_jack_engine;

extern crate pest;
#[macro_use] extern crate pest_derive;
#[macro_use] extern crate debug_stub_derive;

use std::env;
use std::process::exit;
use std::fs::read_to_string;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 { exit(1); }
    let filename = &args[1];
    let source = read_to_string(filename).expect("cannot read file");
    let mut document = parse(&source);
    document.load_resources();
    eprintln!("{:#?}", &document);
    start_jack_engine(document);
}
