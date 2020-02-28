//! This is Lude, a language for *painting with time*.
//! A Lude program describes how a **cursor**
//! moves in time to places **slices** of media files.
//! The output of a Lude program is a rendered media composition.
//! 
//! ## TODO
//! * **TODO** Render output to `stdout`.
//! * **TODO** Render waveform/spectrogram/loundess curve alongside output.
//! * **TODO** Render the results of the parse and evaluate stages in a GUI,
//! which allows the source file to be manipulated in a simple, semantics-aware
//! manner. See [Iced](https://github.com/hecrj/iced).

#![warn(missing_doc_code_examples)]
#![warn(missing_docs)]

mod types;
mod eval;
mod document;
mod render;
mod media;
mod io;

#[cfg(test)]
mod tests;

extern crate pest;
#[macro_use] extern crate pest_derive;
//#[macro_use] extern crate lazy_static;
//#[macro_use] extern crate debug_stub_derive;

use std::env;
use std::process::exit;
use std::fs::read_to_string;

pub use types::*;
pub use eval::{read, eval};
pub use render::{render, to_channels, to_frames};
pub use document::Document;
use io::file::write_to_file;

/// Takes a source file and renders it to a file.
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: lude SOURCE_FILE");
        exit(1);
    }
    let filename = &args[1];
    let source = read_to_string(filename).expect("cannot read file");
    //eprintln!("{:#?}", &source);
    let parsed = read(&source);
    //eprintln!("{:#?}", &parsed);
    let document = eval(parsed);
    //eprintln!("{:#?}", &document);
    let (_, max, longest) = document.bounds();
    let rendered = render(&document, 0, max + longest);
    let output = to_frames(to_channels(rendered));
    //eprintln!("{:#?}", &output);
    write_to_file(output, "output.wav");
}
