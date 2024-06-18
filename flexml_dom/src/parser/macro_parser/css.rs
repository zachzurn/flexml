use crate::document::{Context, Element, ParseError};

//TODO parse css as actuall css instead of just a string
pub(crate) fn parse(src: &str, macro_element: &Element, context: &mut Context) -> Result<Option<Element>, ParseError>{
    context.css = src.to_owned();
    Ok(None)
}