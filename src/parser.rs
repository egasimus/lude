use pest::Parser;

#[derive(Parser)]
#[grammar = "../grammars/default.pest"]
struct DefaultParser;

pub fn parse (document: &str) {
    let document = DefaultParser::parse(Rule::Document, document)
        .unwrap_or_else(|e| panic!("{}", e)).next().unwrap();
    for pair in document.into_inner() {
        println!(">{:?}", pair);
    };
}
