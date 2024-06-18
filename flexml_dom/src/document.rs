use std::collections::HashMap;
use logos::Source;

#[derive(Debug)]
pub(crate) struct Macro {
    name: String,
    content: String
}

#[derive(Debug)]
pub(crate) struct Context {
    pub(crate) css: String,
    pub(crate) errors: Vec<ParseError>,
}

#[derive(Debug)]
pub(crate) struct Document {
    pub(crate) context: Context,
    pub(crate) root: Option<Element>
}

impl Document {
    pub(crate) fn new() -> Self {
        Self {
            context: Context {
                css : "".to_owned(),
                errors: Vec::new(),
            },
            root: None,

        }
    }
}

#[derive(Debug)]
pub(crate) enum ElementVariant {
    Root,
    Break,
    Container,
    Text,
    Comment,
    Image,
    Macro(String)
}

#[derive(Debug)]
pub(crate) struct Element {
    pub(crate) variant: ElementVariant,
    pub(crate) options: Option<HashMap<String, OptionValue>>,
    pub(crate) classes: Vec<String>,
    pub(crate) content: String,
    pub(crate) children: Vec<Element>
}

impl Element {
    pub(crate) fn page_break_variant() -> Element {
        Element {
            classes: vec![],
            content: String::new(),
            children: vec![],
            variant: ElementVariant::Break,
            options: None
        }
    }

    pub(crate) fn root_variant() -> Element {
        Element {
            classes: vec![],
            content: "".to_owned(),
            children: vec![],
            variant: ElementVariant::Root,
            options: None
        }
    }

    pub(crate) fn from_raw_name(raw_name: &str) -> Element {
        let classes = raw_name.slice(0..raw_name.len()-1).unwrap_or("");
        Element::with_classes(classes)
    }

    pub(crate) fn from_raw_macro_name(raw_name: &str) -> Element {
        let macro_name = raw_name.slice(0..3).unwrap_or("RAW");
        let classes = raw_name.slice(4..raw_name.len()-1).unwrap_or("");
        let mut element = Element::with_classes(classes);
        element.variant = ElementVariant::Macro(macro_name.to_owned());
        element
    }

    pub(crate) fn comment_variant(comment: &str) -> Element {
        let content = if comment.len() < 2 {
            "".to_owned()
        } else {
            comment
                .slice(2..comment.len())
                .unwrap_or("")
                .trim()
                .to_owned()
        };

        Element {
            classes: vec![],
            content,
            children: vec![],
            variant: ElementVariant::Comment,
            options: None
        }
    }

    pub(crate) fn text_variant(content: String) -> Element {
        Element {
            classes: vec![],
            content: content.to_owned(),
            children: vec![],
            variant: ElementVariant::Text,
            options: None
        }
    }

    pub(crate) fn container_variant() -> Self {
        Element {
            classes: vec![],
            content: "".to_owned(),
            children: vec![],
            variant: ElementVariant::Container,
            options: None
        }
    }

    pub(crate) fn with_classes(raw_classes: &str) -> Self {
        let mut classes = vec![];
        let content = "".to_owned();

        if raw_classes.len() != 0 {
            let class_strings = raw_classes.split(".");

            for class_string in class_strings {
                classes.push(class_string.to_owned());
            }
        }

        Element {
            classes,
            content,
            children: vec![],
            variant: ElementVariant::Container,
            options: None
        }
    }
}




#[derive(Debug)]
pub(crate) enum OptionValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<OptionValue>),
    Object(HashMap<String, OptionValue>),
}

pub(crate) enum OptionValueVariant {
    Null,
    String,
    Number,
    Array,
    Object,
    Bool
}

impl OptionValueVariant {
    pub fn describe(&self) -> &str{
        match self {
            OptionValueVariant::String => { "a string" }
            OptionValueVariant::Number => { "a number" }
            OptionValueVariant::Array => { "an array" }
            OptionValueVariant::Object => { "an object" }
            OptionValueVariant::Bool => { "a boolean value" }
            OptionValueVariant::Null => { "a null value" }
        }
    }

    pub fn validate_value(&self, value: &OptionValue) -> bool{
        match self {
            OptionValueVariant::String => { value.is_string() }
            OptionValueVariant::Number => { value.is_number() }
            OptionValueVariant::Array => { value.is_array() }
            OptionValueVariant::Object => { value.is_object() }
            OptionValueVariant::Bool => { value.is_bool() }
            OptionValueVariant::Null => { value.is_null() }
        }
    }
}

impl OptionValue {
    pub fn is_null(&self) -> bool {
        return match self {
            OptionValue::Null => { true }
            _ => { false }
        }
    }

    pub fn is_number(&self) -> bool {
        return match self {
            OptionValue::Number(_) => { true }
            _ => { false }
        }
    }

    pub fn is_array(&self) -> bool {
        return match self {
            OptionValue::Array(_) => { true }
            _ => { false }
        }
    }

    pub fn is_object(&self) -> bool {
        return match self {
            OptionValue::Object(_) => { true }
            _ => { false }
        }
    }

    pub fn is_string(&self) -> bool {
        return match self {
            OptionValue::String(_) => { true }
            _ => { false }
        }
    }

    pub fn is_bool(&self) -> bool {
        return match self {
            OptionValue::Bool(_) => { true }
            _ => { false }
        }
    }
}

#[derive(Debug)]
pub(crate) enum ParseErrorVariant {
    Unexpected,
    UnclosedElement,
    Macro
}

#[derive(Debug)]
pub(crate) struct ParseError {
    pub(crate) line: usize,
    pub(crate) col: usize,
    pub(crate) variant: ParseErrorVariant,
    pub(crate) description: String,
}

impl ParseError {
    pub(crate) fn unclosed_element(line: usize, col: usize) -> ParseError {
        Self {
            line,
            col,
            variant: ParseErrorVariant::UnclosedElement,
            description: "An element was not closed properly.".to_string(),
        }
    }
}