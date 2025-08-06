mod tree;
mod inline;
mod taffy_style;
pub mod fragments;

use crate::document::nodes::Node;
use crate::document::parser::FlexmlDocument;
use crate::layout::fragments::{collect_fragments, Fragment, FragmentGroup, Radius, Rect};
use crate::layout::tree::{LayoutNode, LayoutNodeKind, LayoutTree};
use crate::styles::context::{Display, StyleContext};
use crate::styles::style::AtomicStyle;
use crate::styles::style_registry::StyleRegistry;
use parley::{FontContext, LayoutContext};
use taffy::{AvailableSpace, NodeId, Size};

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
        let dpi = doc.root_style.dpi();
        let none = 0.0f32;
        let page_width = doc.root_style.width().as_pixels(none, none, none, dpi);
        let page_height = doc.root_style.height().as_pixels(none, none, none, dpi);

        let mut pages = vec!(FlexmlPage{
            fragments: vec![],
        });

        let current_page = pages.last_mut().unwrap();

        // TODO link up fonts from doc.style_registry to the parley fonts
        //layout_context.parley_font_context.collection.register_fonts()

        // This holds references to all layout nodes that are generated
        let mut layout_tree = LayoutTree::new(layout_context);

        //Render page style fragments
        let page_rect = Rect::new(0.0,0.0,page_width, page_height);
        let mut page_fragments = FragmentGroup::new(page_rect);
        page_fragments.fragments.push(Fragment::bg(page_rect, Radius::zero(), doc.root_style.bg_color()));
        current_page.fragments.push(page_fragments);


        // TODO, available space should include padding calculations
        let em = doc.root_style.resolved_font_size();
        let rem = doc.root_style.resolved_root_font_size();
        let dpi = doc.root_style.dpi();

        let page_top = doc.root_style.padding_top().as_pixels(page_width, rem, em, dpi);
        let page_left = doc.root_style.padding_left().as_pixels(page_width, rem, em, dpi);
        let page_bottom = doc.root_style.padding_bottom().as_pixels(page_width, rem, em, dpi);
        let page_right = doc.root_style.padding_right().as_pixels(page_width, rem, em, dpi);
        let page_space: Size<AvailableSpace> = Size{width: AvailableSpace::from(page_width - (page_left + page_right)), height: AvailableSpace::MaxContent};

        //We treat the top level as a box container with its children
        //being root nodes. Each root node is laid out and fragmented/paginated
        //one by one
        let root_layout_id = cascade_container(&mut layout_tree, true, &doc.style_registry, &doc.root_style, &doc.nodes, &[]);
        let root_layout = layout_tree.node_from_id(root_layout_id);
        let root_node_ids = root_layout.children.clone();

        let _current_page_max_y = page_height - page_bottom;
        let mut current_page_y = page_top;
        let current_page_x = page_left;

        // loop through root_layout_nodes and generate fragments + paginate
        for root_layout_node in root_node_ids {
            layout_tree.compute_layout(root_layout_node, page_space,true);

            layout_tree.print_tree(root_layout_node);
            collect_fragments(&layout_tree, root_layout_node, current_page_x, current_page_y, &mut current_page.fragments);

            let root_node = layout_tree.node_from_id(root_layout_node);
            current_page_y += root_node.final_layout.size.height;
        }

        FlexmlLayout{ page_width, page_height, dpi,
            pages,
            context: layout_tree.context
        }
    }
}

/// Core cascade recurse
///
/// Cascades styles and collects LayoutNodes
/// Container, InlineContent, Text
///
/// Container:
/// Raw Containers are Block, Flex containers
/// Flex containers ignore direct descendent whitespace and will wrap text
/// into blocks with InlineContent
///
/// InlineContent:
/// InlineContent holds Text and InlineBlock Containers only
/// InlineContent should not hold child InlineContent, these
/// should be flattened as direct children instead
///
fn cascade_container(
    tree: &mut LayoutTree,
    is_root: bool,
    style_registry: &StyleRegistry,
    parent_style: &StyleContext,
    box_children: &[Node],
    box_styles: &[AtomicStyle],
) -> NodeId {
    let layout_style = if is_root {
        *parent_style
    } else {
        style_registry.resolve_style(parent_style, box_styles)
    };

    let mut layout_children: Vec<NodeId> = Vec::new();
    let mut inline_buffer: Vec<NodeId> = Vec::new();

    for box_child in box_children {
        // process each child
        match box_child {
            Node::BoxContainer { styles, children } => {
                let child_style = style_registry.resolve_style(&layout_style, styles);

                match child_style.display() {
                    // Inline nodes are flattened into the inline buffer
                    Display::Inline => {
                        // Cascade inline. All text is flattened into the inline buffer
                        flush_inline_to_buffer(tree, style_registry, &child_style, children, &mut inline_buffer);
                    }
                    // Inline Blocks are cascaded and live alongside inline content
                    Display::InlineBlock => {
                        let child_node = cascade_container(
                            tree, false, style_registry, &child_style, children, styles,
                        );

                        inline_buffer.push(child_node);
                    }
                    // Every other BoxContainer type is cascaded
                    _ => {
                        let child_node = cascade_container(
                            tree, false, style_registry, &child_style, children, styles,
                        );

                        // flush entire inline buffer before adding block child
                        flush_inline_buffer(tree, &layout_style, &mut inline_buffer, &mut layout_children);
                        layout_children.push(child_node);
                    }
                }
            }
            Node::Text(text) | Node::Whitespace(text) => {
                let text_node = tree.add_node(LayoutNode::new_text(layout_style, text.to_string()));
                inline_buffer.push(text_node);
            }
            _ => {
                flush_inline_buffer(tree, &layout_style, &mut inline_buffer, &mut layout_children);
            }
        }
    }

    // flush inline buffer once at the very end
    flush_inline_buffer(tree, &layout_style, &mut inline_buffer, &mut layout_children);

    // create the container node (Block or Flex)
    tree.add_node(LayoutNode::new_container(
        LayoutNodeKind::Container,
        layout_style,
        layout_children,
    ))
}

/// If a buffer has nodes, wraps them in an InlineContent LayoutNode
/// This is used to clean out a buffer and push the generated LayoutNode
/// into the completed output vec
fn flush_inline_buffer(
    tree: &mut LayoutTree,
    style: &StyleContext,
    buffer: &mut Vec<NodeId>,
    output: &mut Vec<NodeId>,
) {
    if !buffer.is_empty() {
        let inline_content = tree.add_node(LayoutNode::new_container(
            LayoutNodeKind::InlineContent,
            *style,
            std::mem::take(buffer),
        ));
        output.push(inline_content);
    }
}

/// Take a node's children and flush
/// all text to the output vec.
///
/// Styles are still cascaded, but container
/// boxes are dropped since we don't need them.
///
/// This is used to flatten inline containers
fn flush_inline_to_buffer(
    tree: &mut LayoutTree,
    style_registry: &StyleRegistry,
    inherited_style: &StyleContext,
    children: &Vec<Node>,
    output: &mut Vec<NodeId>,
) {
    for node in children {
        match node {
            Node::Text(text) | Node::Whitespace(text) => {
                output.push(tree.add_node(LayoutNode::new_text(*inherited_style, text.to_string())));
            }
            Node::BoxContainer { styles, children } => {
                let container_style = style_registry.resolve_style(inherited_style, styles);
                flush_inline_to_buffer(tree, style_registry, &container_style, children, output );
            }
            _ => {}
        }
    }
}









