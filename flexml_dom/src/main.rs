use crate::parsing::parser::Parser;

pub mod parsing;
pub mod document;

fn main() {
    let input = "[bold+italic this is some text ]";

    let mut parser = Parser::new(input);

    while let Some(node) = parser.parse_next() {
        println!("{:#?}", node);
    }
}