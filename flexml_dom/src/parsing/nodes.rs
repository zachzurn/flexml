#[derive(Debug, PartialEq)]
pub struct Style<'a> {
    pub name: &'a str,
    pub value: Option<&'a str>,
}

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
    StyleDefinition {
        name: &'a str,
        styles: Vec<Style<'a>>,
    },

    // Box with children
    BoxContainer {
        styles: Vec<Style<'a>>,
        children: Vec<Node<'a>>,
    },
}