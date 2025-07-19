use crate::parsing::parser::Parser;

pub mod parsing;
mod styles;

fn main() {
    let input = "[bold+italic this is some text \r\n and some more on a new line ]";

    let mut parser = Parser::new(input);

    while let Some(node) = parser.parse_next() {
        println!("{:#?}", node);
    }
}