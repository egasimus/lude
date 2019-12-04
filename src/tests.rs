use crate::eval::{read, eval};
use crate::render::{render, to_channels, to_frames};
use crate::document::Document;
use crate::types::{FrameTime, Chunk};

fn assert_some<T> (v: &Vec<Option<T>>, i: usize) {
    match v.get(i).unwrap() {
        Some(_) => {},
        None => panic!("#{} should exist", &i),
    }
}

fn assert_none<T: std::fmt::Debug> (v: &Vec<Option<T>>, i: usize) {
    match v.get(i).unwrap() {
        Some(x) => panic!("#{} should not exist, was {:?}", &i, &x),
        None => {}
    }
}

fn eval_expect_len (src: &str, len: FrameTime, elen: usize) -> Document {
    let doc = eval(read(src));
    assert_eq!(doc.length, len);
    assert_eq!(doc.events.len(), elen);
    doc
}

fn render_expect_len (
    doc: &Document, from: FrameTime, to: FrameTime, len: usize
) -> Chunk {
    let out = render(&doc, from, to);
    assert_eq!(out.len(), len);
    out
}

#[test]
fn test_0_empty () {
    for (i, src) in vec!["", " ", "\n"].iter().enumerate() {
        eprintln!("--- test 0.{} ---", &i);
        let doc = eval_expect_len(src, 0, 0);
        render_expect_len(&doc, 0,   0,   1);
        render_expect_len(&doc, 100, 300, 201);
    }
}

#[test]
fn test_1_source_slice () {
    for (i, (src, events, samples)) in vec![
        ("./test/100ms.wav", 0, 0),
        // ("./test/100ms.wav|", 0, 0), TODO test error
        ("./test/100ms.wav||", 1, 4410),
        ("./test/100ms.wav|||", 2, 8820),
        ("./test/100ms.wav||||", 3, 13230),
        ("./test/100ms.wav|||:2205|", 3, 11025),
        ("./test/100ms.wav|||2205:|./test/100ms.wav", 3, 11025),
        ("./test/100ms.wav|||2205:|./test/100ms_inverted.wav||", 4, 15435),
    ].iter().enumerate() {
        eprintln!("--- test 1.{} --- {}", &i, &src);
        let doc = eval(read(src));
        assert_eq!(doc.events.len(), *events);
        assert_eq!(doc.length, *samples);
    }
}

#[test]
fn test_2_jumps () {
    for (i, (src, events, samples, ranges)) in vec![
        (
            "@10 ./test/100ms.wav|:10| @30 ./test/100ms.wav|:10|:10|",
            0, 50, vec![
                (true,  0, 0),
                (false, 1, 1)
            ]
        )
    ].iter().enumerate() {
        eprintln!("--- test 1.{} --- {}", &i, &src);
        let doc = eval(read(src));
        let out = render_expect_len(&doc, 0, *samples, *samples+1);
        for (exists, start, end) in ranges {
            expect_range(&out, *exists, *start, *end);
        }
    }
}

fn expect_range (out: &Chunk, exists: bool, start: FrameTime, end: FrameTime) {
    for i in start..end {
        match exists {
            true  => assert_some(&out, i),
            false => assert_none(&out, i)
        }
    }
}
