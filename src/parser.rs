use std::time::Instant;
use pest::{Parser, RuleType, iterators::Pair};
use crate::timeline::Moment;
use super::Document;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
struct DefaultParser;

pub fn parse (document: &str) -> Document {
    let parse_start = Instant::now();
    let mut doc = Document::new();
    let parsed = DefaultParser::parse(Rule::Document, document)
        .unwrap_or_else(|e| panic!("{}", e)).next().unwrap();
    //eprintln!("{:#?}", &parsed);
    for statement in parsed.into_inner() {
        for inner in statement.into_inner() {
            match inner.as_rule() {
                Rule::Definition => {
                    let (key, val) = parse_definition::<Rule>(inner);
                    doc.add_definition(key, val);
                },
                Rule::NamedSeq => {
                    let (name, parsed) = parse_named_seq::<Rule>(inner);
                    let (seq, _start, _dur, _rep, _div) = parsed;
                    doc.add_sequence(&name, seq);
                    doc.add_definition(
                        &name,
                        Command::new(Commands::Sequence, vec![name.to_string()])
                    );
                },
                Rule::Seq => {
                    let (seq, _start, _dur, _rep, _div) = parse_seq::<Rule>(inner);
                    doc.add_sequence(&"<main>".to_string(), seq);
                },
                Rule::EOI => {},
                _ => unreachable!()
            }
        }
    };
    eprintln!("Parsed in {}usec", parse_start.elapsed().as_micros());
    doc
}

fn parse_definition<T: RuleType> (pair: Pair<Rule>) -> (&str, Command) {
    let mut definition = pair.into_inner();
    //eprintln!("Assign {:#?}", definition);
    let key = definition.next().unwrap().as_str().trim();
    let val_body = definition.next().unwrap();
    let val = match &val_body.as_rule() {
        Rule::Expression => {
            let mut val_tokens = val_body.into_inner();
            let val_head = val_tokens.next().unwrap();
            let val_rule = &val_head.as_rule();
            //eprintln!("->{:#?} {:#?}", &val_head, &val_rule);
            match val_head.as_str() {
                "sound" => Command::new(
                    Commands::Sound,
                    val_tokens.map(|t| t.as_str().to_string()).collect()
                ),
                _ => Command::nop()
            }
        },
        _ => Command::nop(),
    };
    (key, val)
}

type ParsedSeq = (Sequence, Moment, Moment, u128, u128);

fn parse_named_seq<T: RuleType> (pair: Pair<Rule>) -> (String, ParsedSeq) {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str();
    let next = inner.next().unwrap();
    //eprintln!("{:#?}", &next);
    let seq = parse_seq::<Rule>(next);
    (name.to_string(), seq)
}

fn parse_seq<T: RuleType> (pair: Pair<Rule>) -> ParsedSeq {
    let mut inner = pair.into_inner();
    let start = 0;
    let end = parse_seq_head::<Rule>(inner.next().unwrap());
    let seq = parse_seq_body::<Rule>(inner.next().unwrap());
    let (rep, div) = parse_seq_tail::<Rule>(inner.next().unwrap());
    (seq, start, end, rep, div)
}

fn parse_seq_head<T: RuleType> (_head: Pair<Rule>) -> Moment {
    //eprintln!("[ {:#?}", &start);
    //for item in head.into_inner() {
    //}
    0
}

fn parse_seq_tail<T: RuleType> (tail: Pair<Rule>) -> (u128, u128) {
    let mut rep = 0;
    let mut div = 0;
    for _item in tail.into_inner() {
        rep = 0;
        div = 0;
    }
    (rep, div)
}

fn parse_seq_body<T: RuleType> (body: Pair<Rule>) -> Sequence {
    let mut seq = Sequence::new();
    let mut next = 0;
    //eprintln!("~ {:#?}", body);
    for step in body.into_inner() {
        let step_inner = step.into_inner().next().unwrap();
        match step_inner.as_rule() {
            Rule::Rest => {},
            Rule::Hit => {
                for hit in step_inner.into_inner() {
                    seq.add(next, hit.as_str().to_string());
                }
            }
            _ => unreachable!()
        }
        next += 1;
    }
    seq.end = next;
    seq
}

fn parse_step<T: RuleType> (_pair: Pair<Rule>) {
}
