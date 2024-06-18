use std::collections::HashMap;
use crate::document::{Context, Element, ElementVariant, OptionValueVariant, ParseError};
use crate::parser::util::parse_macro_options;

pub(crate) fn parse(src: &str, macro_element: &Element, _: &mut Context) -> Result<Option<Element>, ParseError>{
    let validators = HashMap::from([
        ("width", OptionValueVariant::Number),
        ("height", OptionValueVariant::Number),
        ("path", OptionValueVariant::String)
    ]);
    
    let result  = parse_macro_options(src, &validators);

    match result {
        Ok(options) => {
            Ok(Some(Element{
                variant: ElementVariant::Image,
                options: Some(options),
                classes: macro_element.classes.clone(),
                content: "".to_string(),
                children: vec![],
            }))
        }
        Err(error) => {
            return Err(error)
        }
    }
}