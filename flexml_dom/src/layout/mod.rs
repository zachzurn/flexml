mod tree;
mod inline;
mod taffy_style;

use taffy::{AvailableSpace, NodeId, Size};
use crate::document::nodes::Node;
use crate::document::parser::FlexmlDocument;
use crate::layout::tree::{LayoutNode, LayoutNodeKind, LayoutTree};
use crate::styles::context::{Display, StyleContext};
use crate::styles::style::AtomicStyle;
use crate::styles::style_registry::StyleRegistry;


enum PaintableFragment {
    Text {
        
    }
    Box {

    }
}

struct FlexmlPage {
    elements: Vec<PaintableFragment>
}

/// This is produced from a FlexmlDocument
/// It is a renderable layout
struct FlexmlLayout {
    page_width: usize,
    page_height: usize,
    ppi: usize,
    pages: Vec<PaintableFragment>,
}

impl FlexmlLayout {
    fn new(doc: &FlexmlDocument) -> FlexmlLayout {
        let mut layout = FlexmlLayout{
            page_width: doc.page_width(),
            page_height: doc.page_height(),
            ppi: doc.pixels_per_inch(),
            pages: vec!(),
        };

        // TODO link up fonts from doc.style_registry to the parley fonts


        // Top level nodes are each their own LayoutTree
        // This way we can extract paintable fragments and push to pages
        let mut layout_tree = LayoutTree::new();
        let page_space: Size<AvailableSpace> = Size{width: AvailableSpace::from(layout.page_width as f32), height: AvailableSpace::MaxContent};
        let root_layout_nodes = cascade_flexml_document(&mut layout_tree, doc);

        // loop through root_layout_nodes and generate fragments + paginate
        for root_layout_node in root_layout_nodes {
            layout_tree.compute_layout(root_layout_node, page_space,true);
            let node = layout_tree.node_from_id(NodeId::from(root_layout_node));

            //TODO create fragments here and paginate

        }

        layout
    }
}


/// Functions for creating LayoutNodes from the parsed
/// FlexmlDocument
fn cascade_flexml_document<'a>(tree: &mut LayoutTree, doc: &FlexmlDocument<'a>) -> Vec<usize> {
    let mut root_nodes = Vec::new();

    // These are root level text nodes that do not have a container
    let mut text_orphans: Vec<&'a str> = vec![];

    for node in &doc.nodes {
        match node {
            Node::BoxContainer {styles, children} => {
                if let Some(id) = maybe_add_root_text(tree, &mut text_orphans, doc.root_style) {
                    root_nodes.push(id);
                }

                root_nodes.push(cascade_box_container(tree, &doc.style_registry, &doc.root_style, children, styles));
            },
            Node::Text(text) | Node::Whitespace(text) => {
                text_orphans.push(text);
            },
            Node::Tag{..} => {
                todo!("Expand tags into containers")
            },
            _ => {}
        }
    }

    if let Some(id) = maybe_add_root_text(tree, &mut text_orphans, doc.root_style) {
        root_nodes.push(id);
    }

    root_nodes
}

/// Utility function to drain top level raw str into a LayoutNode::Container
fn maybe_add_root_text(tree: &mut LayoutTree, text: &mut Vec<&str>, style: StyleContext) -> Option<usize> {
    if text.is_empty() { return None };

    let combined = text.drain(..).collect::<String>();
    let text_node_id = tree.add_node(LayoutNode::new_text(style, combined));

    Some(tree.add_node(LayoutNode::new_container(LayoutNodeKind::InlineContent, style, vec![text_node_id])))
}



/// Core cascade recurse
fn cascade_box_container(tree: &mut LayoutTree, style_registry: &StyleRegistry, parent_style: &StyleContext, box_children: &Vec<Node>, box_styles: &Vec<AtomicStyle>) -> usize {
    let layout_style = style_registry.resolve_style(parent_style, box_styles);

    let mut layout_children: Vec<usize> = Vec::new();

    for box_child in box_children {
        match box_child {
            Node::BoxContainer {styles, children} => {
                layout_children.push(
                    cascade_box_container(tree, style_registry, &layout_style, children, styles)
                )
            }
            Node::Text(text) | Node::Whitespace(text) => {
                layout_children.push(tree.add_node(LayoutNode::new_text(layout_style, text.to_string())))
            }
            _ => {}
        }
    }

    let kind = if let Display::Inline = layout_style.display {
        LayoutNodeKind::InlineContent
    } else {
        LayoutNodeKind::Container
    };

    tree.add_node(LayoutNode::new_container(kind, layout_style, layout_children))
}











