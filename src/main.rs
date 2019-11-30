mod types;
mod eval;
mod document;
mod render;
mod media;

pub use eval::{read, eval};
pub use render::render;

#[cfg(test)]
mod tests;

extern crate pest;
#[macro_use] extern crate pest_derive;
//#[macro_use] extern crate lazy_static;
//#[macro_use] extern crate debug_stub_derive;

use std::env;
use std::process::exit;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 { exit(1); }
    let filename = &args[1];
    let source = read_to_string(filename).expect("cannot read file");
    eprintln!("{:#?}", &source);
    let parsed = read(&source);
    eprintln!("{:#?}", &parsed);
    let document = eval(parsed);
    eprintln!("{:#?}", &document);
    //render(&document);
}
