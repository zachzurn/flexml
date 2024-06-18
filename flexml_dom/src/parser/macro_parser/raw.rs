use crate::document::{Context, Element, ElementVariant, ParseError};

//Returns raw elements separated by newline with whitespace preserved
pub(crate) fn parse(src: &str, _macro_element: &Element, _context: &mut Context) -> Result<Option<Element>, ParseError>{
    let mut lines : Vec<Element> = src.lines().map(| line | Element::text_variant(line.to_owned())).collect();
    
    //If first line is just whitespace, remove it
    if lines.len() > 0 && lines.first().unwrap().content.trim().is_empty() {
        lines.remove(0);
    }

    //If last line is just whitespace, remove it
    if lines.len() > 0 && lines.last().unwrap().content.trim().is_empty() {
        lines.remove(lines.len() - 1);
    }
    
    if lines.is_empty() { return Ok(None) };
    
    Ok(Some(
        Element{
            variant: ElementVariant::Container,
            options: None,
            classes: vec![],
            content: "".to_string(),
            children: lines,
        }    
    ))
}