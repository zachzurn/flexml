use crate::document::{Context, Element, ParseError};

// Returns a page break element
pub(crate) fn parse(src: &str, macro_element: &Element, context: &mut Context) -> Result<Option<Element>, ParseError>{
    Ok(Some(Element::page_break_variant()))
}