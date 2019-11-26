use super::{read, eval};

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
/// a document containing a single event
fn test_single_event () {
    let doc = eval(read("./test.wav"));
    assert_eq!(doc.events.len(), 1);
    assert_eq!(doc.length, 100);
    println!("{:#?}", &doc);
}
