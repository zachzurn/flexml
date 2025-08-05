use crate::styles::style::{AtomicStyle, StyleId};
use crate::styles::style_registry::StyleRegistry;

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

impl<'a> Node<'a> {
    pub fn print_tree(
        &self,
        registry: &StyleRegistry,
        prefix: &str,
        last: bool,
    ) {
        let branch = if last { "└── " } else { "├── " };
        let line_prefix = format!("{}{}", prefix, branch);

        let node_label = match self {
            Node::Text(text) => format!("{:?}", text),
            Node::Whitespace(ws) => format!("Whitespace [{}]", ws),
            Node::Tag { name } => format!("Tag <{}>", name),
            Node::StyleDefinition(style_id) => {
                registry.debug_style_definition(*style_id)
            },
            Node::BoxContainer { styles, .. } => {
                format!("Box [{}]", registry.debug_atomics(styles))
            },
        };

        println!("{}{}", line_prefix, node_label);

        let child_prefix = if last {
            format!("{}    ", prefix)
        } else {
            format!("{}│   ", prefix)
        };

        if let Node::BoxContainer { children, .. } = self {
            let count = children.len();
            for (i, child) in children.iter().enumerate() {
                child.print_tree(registry, &child_prefix, i == count - 1);
            }
        }
    }


}