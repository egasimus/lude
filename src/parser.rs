use std::time::Instant;
use std::collections::HashMap;
use pest::{Parser, RuleType, iterators::Pair};
use crate::model::{Sequence, Duration};

#[derive(Parser)]
#[grammar = "../grammars/default/grammar.pest"]
struct DefaultParser;

#[derive(Debug)]
pub struct Document {
    pub definitions: HashMap<String, String>,
    pub sequences: HashMap<String, Sequence>
}

impl Document {
    pub fn new () -> Document {
        let definitions = HashMap::new();
        let sequences = HashMap::new();
        Document { definitions, sequences }
    }

    pub fn definition (&mut self, key: &str, val: &str) {
        self.definitions.insert(key.to_string(), val.to_string());
    }

    pub fn sequence (&mut self, key: &str, val: Sequence) {
        self.sequences.insert(key.to_string(), val);
    }
}

pub fn parse (document: &str) -> Document {
    let parse_headed = Instant::now();
    let mut doc = Document::new();
    let parsed = DefaultParser::parse(Rule::Document, document)
        .unwrap_or_else(|e| panic!("{}", e)).next().unwrap();
    for declaration in parsed.into_inner() {
        for inner in declaration.into_inner() {
            match inner.as_rule() {
                Rule::Definition => {
                    let (key, val) = parse_definition(inner);
                    doc.definition(key, val);
                },
                Rule::NamedSeq => {
                    let (name, parsed) = parse_named_seq::<Rule>(inner);
                    let (seq, dur, rep, div) = parsed;
                    doc.definition(&name, &format!("(play {})", &name));
                    doc.sequence(&name, seq);
                },
                Rule::Seq => {
                    let (seq, dur, rep, div) = parse_seq::<Rule>(inner);
                    doc.sequence(&"<main>".to_string(), seq);
                },
                Rule::EOI => {},
                _ => unreachable!()
            }
        }
    };
    println!("Parsed in {}usec", parse_headed.elapsed().as_micros());
    doc
}

fn parse_definition<T: RuleType> (pair: Pair<T>) -> (&str, &str) {
    let mut definition = pair.into_inner();
    //println!("Assign {:#?}", definition);
    let key = definition.next().unwrap().as_str().trim();
    let val = definition.next().unwrap().as_str().trim();
    (key, val)
}

type ParsedSeq = (Sequence, Duration, u128, u128);

fn parse_named_seq<T: RuleType> (pair: Pair<Rule>) -> (String, ParsedSeq) {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str();
    let next = inner.next().unwrap();
    println!("{:#?}", &next);
    let seq = parse_seq::<Rule>(next);
    (name.to_string(), seq)
}

fn parse_seq<T: RuleType> (pair: Pair<Rule>) -> ParsedSeq {
    let mut inner = pair.into_inner();
    let dur = parse_seq_head::<Rule>(inner.next().unwrap());
    let seq = parse_seq_body::<Rule>(inner.next().unwrap());
    let (rep, div) = parse_seq_tail::<Rule>(inner.next().unwrap());
    (seq, dur, rep, div)
}

fn parse_seq_head<T: RuleType> (head: Pair<Rule>) -> Duration {
    //println!("[ {:#?}", &start);
    for item in head.into_inner() {
        println!("[[ {:#?}", &item);
    }
    0
}

fn parse_seq_tail<T: RuleType> (tail: Pair<Rule>) -> (u128, u128) {
    let mut rep = 0;
    let mut div = 0;
    for item in tail.into_inner() {
        println!("[[ {:#?}", &item);
    }
    (rep, div)
}

fn parse_seq_body<T: RuleType> (body: Pair<Rule>) -> Sequence {
    let mut seq = Sequence::new();
    let mut next = 0;
    //println!("~ {:#?}", body);
    for step in body.into_inner() {
        let step_inner = step.into_inner().next().unwrap();
        match step_inner.as_rule() {
            Rule::Rest => {},
            Rule::Hit => {
                for hit in step_inner.into_inner() {
                    seq.add(next, &hit.as_str());
                }
            }
            _ => unreachable!()
        }
        next += 1;
    }
    seq.length = next;
    seq
}

fn parse_step<T: RuleType> (pair: Pair<Rule>) {
}
