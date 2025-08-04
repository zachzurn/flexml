mod tree;
mod inline;
mod taffy_style;
pub mod fragments;

use parley::{FontContext, LayoutContext, PositionedLayoutItem};
use taffy::{AvailableSpace, NodeId, Size};
use crate::document::nodes::Node;
use crate::document::parser::FlexmlDocument;
use crate::layout::fragments::{collect_fragments, Fragment, FragmentGroup, FragmentKind, GlyphRunFragment, Radius, Rect};
use crate::layout::fragments::FragmentKind::ColorBackground;
use crate::layout::tree::{LayoutNode, LayoutNodeKind, LayoutTree};
use crate::styles::context::{Color, Display, StyleContext};
use crate::styles::style::AtomicStyle;
use crate::styles::style_registry::StyleRegistry;

pub struct FlexmlPage {
    pub fragments: Vec<FragmentGroup>,
}

/// This is produced from a FlexmlDocument
/// It is a renderable layout
pub struct FlexmlLayout {
    pub page_width: f32,
    pub page_height: f32,
    pub dpi: f32,
    pub pages: Vec<FlexmlPage>,
    pub context: FlexmlLayoutContext,

}

pub struct FlexmlLayoutContext {
    pub(super) parley_font_context: FontContext,
    pub(super) parley_layout_context: LayoutContext,
    pub(super) parley_display_scale: f32
}

impl Default for FlexmlLayoutContext {
    fn default() -> Self {
        Self{
            parley_font_context: FontContext::new(),
            parley_layout_context: LayoutContext::new(),
            parley_display_scale: 1.0f32
        }
    }
}

impl FlexmlLayout {
    pub fn new(doc: &FlexmlDocument, layout_context: FlexmlLayoutContext) -> FlexmlLayout {
        let dpi = doc.root_style.dpi;
        let none = 0.0f32;
        let page_width = doc.root_style.width.to_pixels(none, none, none, dpi);
        let page_height = doc.root_style.height.to_pixels(none, none, none, dpi);

        let mut pages = vec!(FlexmlPage{
            fragments: vec![],
        });

        let current_page = pages.last_mut().unwrap();

        // TODO link up fonts from doc.style_registry to the parley fonts
        //layout_context.parley_font_context.collection.register_fonts()

        println!("doc {:?}", doc.nodes);

        let mut layout_tree = LayoutTree::new(layout_context);

        let page_space: Size<AvailableSpace> = Size{width: AvailableSpace::from(page_width), height: AvailableSpace::MaxContent};
        let root_layout_nodes = cascade_flexml_document(&mut layout_tree, doc);

        // TODO implement padding into these
        let mut page_y = 0.0;
        let page_x = 0.0;
        let page_max_y = 0.0;

        //Render page style fragments
        let page_rect = Rect::new(0.0,0.0,page_width, page_height);
        let mut page_fragments = FragmentGroup::new(page_rect);
        page_fragments.fragments.push(Fragment::bg(page_rect, Radius::zero(), doc.root_style.bg_color));
        current_page.fragments.push(page_fragments);

        // loop through root_layout_nodes and generate fragments + paginate
        for root_layout_node in root_layout_nodes {
            layout_tree.compute_layout(root_layout_node, page_space,true);

            layout_tree.print_tree(root_layout_node);
            collect_fragments(&layout_tree, root_layout_node, page_x, page_y, &mut current_page.fragments);

            let root_node = layout_tree.node_from_id(NodeId::from(root_layout_node));
            page_y += root_node.final_layout.size.height;
        }

        FlexmlLayout{ page_width, page_height, dpi,
            pages,
            context: layout_tree.context
        }
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

    let inline_content_id= tree.add_node(LayoutNode::new_container(LayoutNodeKind::InlineContent, style, vec![text_node_id]));

    Some(tree.add_node(LayoutNode::new_container(LayoutNodeKind::Container, style, vec![inline_content_id])))
}



/// Core cascade recurse
fn flush_inline_buffer(
    tree: &mut LayoutTree,
    style: &StyleContext,
    buffer: &mut Vec<usize>,
    output: &mut Vec<usize>,
) {
    if !buffer.is_empty() {
        let inline_node = tree.add_node(LayoutNode::new_container(
            LayoutNodeKind::InlineContent,
            style.clone(),
            std::mem::take(buffer),
        ));
        output.push(inline_node);
    }
}

fn cascade_box_container(
    tree: &mut LayoutTree,
    style_registry: &StyleRegistry,
    parent_style: &StyleContext,
    box_children: &[Node],
    box_styles: &[AtomicStyle],
) -> usize {
    let layout_style = style_registry.resolve_style(parent_style, box_styles);
    let mut layout_children: Vec<usize> = Vec::new();
    let mut inline_buffer: Vec<usize> = Vec::new();

    for box_child in box_children {
        match box_child {
            Node::BoxContainer { styles, children } => {
                let child_style = style_registry.resolve_style(&layout_style, styles);

                match child_style.display {
                    Display::Inline => {
                        let inline_child = cascade_box_container(
                            tree,
                            style_registry,
                            &layout_style,
                            children,
                            styles,
                        );
                        inline_buffer.push(inline_child);
                    }
                    _ => {
                        flush_inline_buffer(tree, &layout_style, &mut inline_buffer, &mut layout_children);

                        let block_child = cascade_box_container(
                            tree,
                            style_registry,
                            &layout_style,
                            children,
                            styles,
                        );
                        layout_children.push(block_child);
                    }
                }
            }

            Node::Text(text) | Node::Whitespace(text) => {
                let text_node =
                    tree.add_node(LayoutNode::new_text(layout_style, text.to_string()));
                inline_buffer.push(text_node);
            }

            _ => {
                flush_inline_buffer(tree, &layout_style, &mut inline_buffer, &mut layout_children);
            }
        }
    }

    flush_inline_buffer(tree, &layout_style, &mut inline_buffer, &mut layout_children);

    tree.add_node(LayoutNode::new_container(
        LayoutNodeKind::Container,
        layout_style,
        layout_children,
    ))
}













