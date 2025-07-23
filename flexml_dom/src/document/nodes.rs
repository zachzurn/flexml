use crate::styles::style::{AtomicStyle, StyleId};

/// All Flexml node types
#[derive(Debug, PartialEq)]
pub enum Node<'a> {
    //Any text content
    Text(&'a str),

    //Contiguous whitespace
    Whitespace(&'a str),

    // <tag>
    Tag {
        name: &'a str,
    },

    // Define styles
    StyleDefinition(StyleId),

    // Box with children
    BoxContainer {
        styles: Vec<AtomicStyle>,
        children: Vec<Node<'a>>,
    },
}