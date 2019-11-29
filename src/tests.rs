use crate::eval::{read, eval};
use crate::render::render;

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
}

#[test]
/// a document containing one or more simple events
fn test_single_event () {
    let doc = eval(read("./test/100ms.wav"));
    assert_eq!(doc.events.len(), 1);
    assert_eq!(doc.length, 4410);

    let doc = eval(read("./test/100ms.wav\n./test/100ms.wav"));
    assert_eq!(doc.events.len(), 2);
    assert_eq!(doc.length, 8820);
}

#[test]
/// a document containing simple events and/or jumps
fn test_jumps () {
    let doc = eval(read("./test/100ms.wav\n@10\n./test/100ms.wav"));
    assert_eq!(doc.events.len(), 2);
    assert_eq!(doc.length, 4420);

    let doc = eval(read("./test/100ms.wav\n@+10\n./test/100ms.wav"));
    assert_eq!(doc.events.len(), 2);
    assert_eq!(doc.length, 8830);

    let doc = eval(read("./test/100ms.wav\n@-4410\n./test/100ms.wav"));
    assert_eq!(doc.events.len(), 1);
    assert_eq!(doc.events.get(&0).unwrap().len(), 2);
    assert_eq!(doc.length, 4410);
}

#[test]
/// render to memory
fn test_render () {
    let doc = eval(read(""));
    let out = render(&doc, 0, 0);
    assert_eq!(out.len(), 1);
    let out = render(&doc, 100, 300);
    assert_eq!(out.len(), 201);
}
