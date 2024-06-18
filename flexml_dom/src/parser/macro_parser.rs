mod img;
mod css;
mod csv;
mod brk;
mod raw;

use crate::document::*;

pub(crate) fn parse(element: &Element, context: &mut Context) -> Result<Option<Element>, ParseError> {
    let result = match &element.variant {
        ElementVariant::Macro(name) => {
            let src = element.content.as_str();
            match name.as_str() {
                "CSS" => { css::parse(src, element, context) }
                "IMG" => { img::parse(src, element, context) }
                "BRK" => { brk::parse(src, element, context) }
                "CSV" => { csv::parse(src, element, context) }
                &_ =>    { raw::parse(src, element, context) }
            }
        }
        _ => {
            //TODO return an error
            Err(ParseError{
                line: 0,
                col: 0,
                variant: ParseErrorVariant::Unexpected,
                description: "Trying to parse a macro_parser that isn't a macro_parser.".to_string(),
            })
        }
    };

    //TODO modify the error col and row based on the relative col row from the macro_parser parse
    result
}
