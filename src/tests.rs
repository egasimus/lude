use crate::eval::{read, eval};
use crate::render::{render, flatten};

#[test]
/// a document can contain zero statements
fn test_empty_document () {
    let doc = eval(read(""));
    assert_eq!(doc.length, 0);
    assert_eq!(doc.events.len(), 0);

    let doc = eval(read(" "));
    assert_eq!(doc.length, 0);
    assert_eq!(doc.events.len(), 0);

    let doc = eval(read("\n"));
    assert_eq!(doc.length, 0);
    assert_eq!(doc.events.len(), 0);

    println!("Render #1A (empty, frame 0)");
    let doc = eval(read(""));
    let out = render(&doc, 0, 0);
    assert_eq!(out.len(), 1);

    println!("Render #1B (empty, frames 100-300)");
    let out = render(&doc, 100, 300);
    assert_eq!(out.len(), 201);
}

#[test]
/// a document containing one or more simple events
fn test_simple_event () {
    let doc = eval(read("./test/100ms.wav"));
    assert_eq!(doc.events.len(), 1);
    assert_eq!(doc.length, 4410);

    let doc = eval(read("./test/100ms.wav ./test/100ms.wav"));
    assert_eq!(doc.events.len(), 2);
    assert_eq!(doc.length, 8820);

    println!("Render #2 (100ms.wav, frames 0-4412)");
    let doc = eval(read("./test/100ms.wav"));
    let out = render(&doc, 0, 4412);
    assert_eq!(out.len(), 4413);
    assert_some(&out, 0);
    assert_some(&out, 4411);
    assert_some(&out, 4412);
}

#[test]
/// a document containing simple events and/or jumps
fn test_jumps () {
    let doc = eval(read("./test/100ms.wav @10 ./test/100ms.wav"));
    assert_eq!(doc.events.len(), 2);
    assert_eq!(doc.length, 4420);

    let doc = eval(read("./test/100ms.wav @+10 ./test/100ms.wav"));
    assert_eq!(doc.events.len(), 2);
    assert_eq!(doc.length, 8830);

    let doc = eval(read("./test/100ms.wav @-4410 ./test/100ms.wav"));
    assert_eq!(doc.events.len(), 1);
    assert_eq!(doc.events.get(&0).unwrap().len(), 2);
    assert_eq!(doc.length, 4410);

    println!("Render #3 (phase cancellation)");
    let doc = eval(read("./test/100ms.wav @0 ./test/100ms_inverted.wav"));
    let out = render(&doc, 0, 100);
    //println!("{:?}", &out);
}

#[test]
/// a document containing a sliced event
fn test_slice () {
    println!("Render #4A (1 frame of slice)");
    let doc = eval(read("./test/100ms.wav[10:30]"));
    assert_eq!(doc.length, 20);
    let out = render(&doc, 0, 0);
    println!("Render #4A = {:?}", &out);
    assert_eq!(out.len(), 1);

    println!("Render #4B (30 frames of slice)");
    let out = render(&doc, 0, 29);
    println!("Render #4B = {:?}", &out);
    assert_eq!(out.len(), 30);
    for i in 0..21 { assert_some(&out, i); }
    for i in 21..30 { assert_none(&out, i); }

    let doc = eval(read("@10 ./test/100ms.wav[10:20] @30 ./test/100ms.wav[0:10]"));
    println!("Render #4C (two slices)");
    let out = render(&doc, 0, 100);
    println!("Render #4C = {:?}", &out);
    for i in 0..10 { assert_none(&out, i); }
    for i in 10..21 { assert_some(&out, i); }
    for i in 21..30 { assert_none(&out, i); }
    for i in 31..41 { assert_some(&out, i); }
    for i in 41..101 { assert_none(&out, i); }
    println!("Flat   #4C = {:?}", flatten(out));

    //println!("Render #4C (overlapping slices)");
}

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
