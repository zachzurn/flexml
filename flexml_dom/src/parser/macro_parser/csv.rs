use crate::document::{Context, Element, ParseError};

//Breaks apart strings by comma delimited value and returns
//An element with rows and columns
//TODO
pub(crate) fn parse(src: &str, macro_element: &Element, context: &mut Context) -> Result<Option<Element>, ParseError>{
    Ok(None)
}