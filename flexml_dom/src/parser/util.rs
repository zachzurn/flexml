use std::collections::HashMap;
use crate::document::{Element, OptionValue, OptionValueVariant, ParseError, ParseErrorVariant};
use crate::parser::json;

pub(crate) struct ElementTreeBuilder {
    elements: Vec<Element>,
    valid_state: bool,
    text_buffer: String,
    buffer_filled: bool
}

impl ElementTreeBuilder {
    pub(crate) fn new() -> Self {
        Self {
            elements: vec![Element::container_variant()],
            valid_state: true,
            text_buffer: String::new(),
            buffer_filled: false
        }
    }

    //Adds a new current parent element
    pub(crate) fn push(&mut self, element: Element){
        self.ensure_valid();
        self.flush_text_buffer();
        self.elements.push(element);
    }

    pub(crate) fn push_text(&mut self, text: &str){
        if text.is_empty() { return };
        self.text_buffer.push_str(text);
        self.buffer_filled = true;
    }

    pub(crate) fn push_child(&mut self, element: Element){
        self.flush_text_buffer();
        self.current().children.push(element);
    }

    pub(crate) fn current(&mut self) -> &mut Element {
        self.ensure_valid();
        if self.elements.len() < 1 { panic!("Invalid attempt to pop root element.") };
        self.elements.last_mut().unwrap()
    }

    pub(crate) fn pop(&mut self) -> Element {
        self.ensure_valid();
        if self.elements.len() < 1 { panic!("Invalid attempt to pop root element.") };
        self.elements.pop().unwrap()
    }

    pub(crate) fn can_fold(&mut self) -> bool {
        self.ensure_valid();
        if self.elements.len() < 1 { false } else { true }
    }

    //Removes the current parent element and adds as a child of the previous parent
    pub(crate) fn fold(&mut self){
        self.ensure_valid();
        if self.elements.len() < 1 { panic!("Invalid attempt to pop root element.") };
        self.flush_text_buffer();
        let child = self.elements.pop().unwrap();
        self.elements.last_mut().unwrap().children.push(child);
    }

    //Removes the root element. Any other use of the
    //ElementStack after this will result in a panic
    pub(crate) fn root(&mut self) -> Element{
        self.ensure_valid();
        self.elements.pop().unwrap()
    }

    pub(crate) fn fold_text(&mut self) {
        self.ensure_valid();
        self.flush_text_buffer();
    }

    fn flush_text_buffer(&mut self){
        if self.buffer_filled {
            let text = self.text_buffer.trim().to_owned();
            self.current().children.push(Element::text_variant(text));
            self.text_buffer.clear();
            self.buffer_filled = false;
        };
    }

    fn ensure_valid(&self){
        if !self.valid_state { panic!("Element stack is being used after being emptied.") };
    }
}


pub(crate) fn validate_macro_option(options: &HashMap<String, OptionValue>, rule: (&&str, &OptionValueVariant)) -> Option<ParseError> {
    let valid = match options.get(*rule.0) {
        None => { false }
        Some(value) => {
            rule.1.validate_value(value)
        }
    };

    if valid {
        None
    } else {
        Some(ParseError{
            line: 0,
            col: 0,
            variant: ParseErrorVariant::Macro,
            description: format!("{} is required and should be {}.",rule.0, rule.1.describe()),
        })
    }

}

pub(crate) fn parse_macro_options(src: &str, validators: &HashMap<&str, OptionValueVariant>) -> Result<HashMap<String, OptionValue>, ParseError> {
    let result = json::parse(src);

    return match result {
        Ok(value) => {
            //Value must be an object and pass validation
            match value {
                OptionValue::Object(object) => {
                    for validator in validators {
                        let err = validate_macro_option(&object, validator);
                        if err.is_some() { return Err(err.unwrap()) }
                    }

                    Ok(object)
                }
                _ => {
                    Err(ParseError {
                        line: 0,
                        col: 0,
                        variant: ParseErrorVariant::Macro,
                        description: "IMG Should have the following format: IMG: { width: 300, height: 200, path: \"./test.png\" }".to_string(),
                    })
                }
            }
        }
        Err(error) => {
            Err(ParseError {
                line: 0,
                col: 0,
                variant: ParseErrorVariant::Macro,
                description: error.0,
            })
        }
    }
}