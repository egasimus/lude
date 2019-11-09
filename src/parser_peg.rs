peg::parser!( grammar sequence_parser() for str {
    rule document() -> ()
        = {}

})

pub fn parse (source: &str) {
    return sequence_parser::document(str)
}
