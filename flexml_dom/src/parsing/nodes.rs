#[derive(Debug, PartialEq)]
pub struct Style<'a> {
    pub name: &'a str,
    pub value: Option<&'a str>,
}

/// All Flexml node types
#[derive(Debug, PartialEq)]
pub enum Node<'a> {
    Text(&'a str),
    Tag {
        name: &'a str,
    },
    StyleDefinition {
        name: &'a str,
        styles: Vec<Style<'a>>,
    },
    BoxContainer {
        styles: Vec<Style<'a>>,
        children: Vec<Node<'a>>,
    },
}