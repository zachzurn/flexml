mod tree;
mod inline;
mod taffy_style;

use parley::PositionedLayoutItem;
use parley::PositionedLayoutItem::GlyphRun;
use taffy::{AvailableSpace, NodeId, Size};
use crate::document::nodes::Node;
use crate::document::parser::FlexmlDocument;
use crate::layout::tree::{LayoutNode, LayoutNodeKind, LayoutTree};
use crate::styles::context::{Color, Display, StyleContext};
use crate::styles::style::AtomicStyle;
use crate::styles::style_registry::StyleRegistry;


struct OwnedGlyphRun {

}

impl<'a> From<parley::GlyphRun<'a, [u8; 4]>> for OwnedGlyphRun {
    fn from(run: parley::GlyphRun<'a, [u8; 4]>) -> Self {
        run.glyphs().collect()
        

        Self {
            glyphs: run.glyphs().to_vec(),
            positions: run.positions().to_vec(),
            color: run.color(),
        }
    }
}

#[derive(Clone)]
struct PaintableRect {
    x: f32, y: f32, w: f32, h: f32
}

enum PaintableFragmentKind {
    Text(parley::GlyphRun<'static, [u8;4]>),
    Rect{color: Color, border_radius: PaintableRect, border_color: Color, border_weight: f32},
}

struct PaintableFragment {
    rect: PaintableRect,
    kind: PaintableFragmentKind
}


struct FlexmlPage {
    elements: Vec<PaintableFragment>
}

/// This is produced from a FlexmlDocument
/// It is a renderable layout
pub struct FlexmlLayout {
    page_width: f32,
    page_height: f32,
    dpi: f32,
    pages: Vec<PaintableFragment>,
}

impl FlexmlLayout {
    pub(crate) fn new<'a>(doc: &FlexmlDocument) -> FlexmlLayout {
        let dpi = doc.root_style.dpi;
        let none = 0.0f32;
        let page_width = doc.root_style.width.to_pixels(none, none, none, dpi);
        let page_height = doc.root_style.height.to_pixels(none, none, none, dpi);

        let mut layout = FlexmlLayout{ page_width, page_height, dpi,
            pages: vec!(),
        };

        // TODO link up fonts from doc.style_registry to the parley fonts


        // Top level nodes are each their own LayoutTree
        // This way we can extract paintable fragments and push to pages
        let mut layout_tree = LayoutTree::new();
        let page_space: Size<AvailableSpace> = Size{width: AvailableSpace::from(layout.page_width), height: AvailableSpace::MaxContent};
        let root_layout_nodes = cascade_flexml_document(&mut layout_tree, doc);

        // loop through root_layout_nodes and generate fragments + paginate
        for root_layout_node in root_layout_nodes {
            layout_tree.compute_layout(root_layout_node, page_space,true);
            let node = layout_tree.node_from_id(NodeId::from(root_layout_node));

            //TODO create fragments here and paginate
            layout_tree.print_tree(root_layout_node);
        }

        layout
    }
}

fn fragment_from_container(container: &LayoutNode) -> Option<Vec<PaintableFragment>> {
    let em = container.style_context.resolved_font_size;
    let rem = container.style_context.resolved_root_font_size;
    let dpi = container.style_context.dpi;

    let has_bg = container.style_context.bg_color.3 != 0;
    let has_border = container.style_context.border_color.3 != 0;

    //No fragments to render
    if !has_bg && !has_border { return None }

    let mut fragments = vec![];
    let layout = &container.final_layout;
    let style = &container.style_context;


    let rect = PaintableRect {
        x: layout.location.x,
        y: layout.location.y,
        w: layout.size.width,
        h: layout.size.height,
    };

    let border_radius = PaintableRect {
        x: container.style_context.border_top_left_radius.to_pixels(layout.size.width, rem, em, dpi),
        y: container.style_context.border_top_right_radius.to_pixels(layout.size.width, rem, em, dpi),
        w: container.style_context.border_bottom_left_radius.to_pixels(layout.size.width, rem, em, dpi),
        h: container.style_context.border_bottom_right_radius.to_pixels(layout.size.width, rem, em, dpi)
    };

    let color = container.style_context.bg_color;
    let border_color = container.style_context.border_color;
    let border_weight = container.style_context.border_width.to_pixels(layout.size.width, rem, em, dpi);

    let kind = PaintableFragmentKind::Rect { color, border_color, border_radius, border_weight };

    fragments.push(PaintableFragment { rect, kind });

    Some(fragments)
}


fn collect_fragments(tree: &LayoutTree, node_id: usize, fragments: &mut Vec<PaintableFragment>) {
    let node = tree.node_from_id(NodeId::from(node_id));
    let node_children = node.children.clone();

    match node.kind {
        LayoutNodeKind::Container => {
            if let Some(some_fragments) = fragment_from_container(node) {
                fragments.extend(some_fragments);
            }
        }
        LayoutNodeKind::InlineContent => {
            if let Some(inline_layout) = &node.inline_layout {
                let mut x = node.final_layout.location.x;
                let mut y = node.final_layout.location.y;

                for line in inline_layout.lines() {
                    // Iterate over GlyphRun's within each line
                    for item in line.items() {
                        match item {
                            PositionedLayoutItem::GlyphRun(glyph_run) => {
                                fragments.push(PaintableFragment{
                                    rect: PaintableRect{ x: x, y: y, w: x, h: x },
                                    kind: PaintableFragmentKind::Text(glyph_run.clone())
                                })
                            }
                            PositionedLayoutItem::InlineBox(inline_box) => {
                                let inline_node = tree.node_from_id(NodeId::from(inline_box.id));
                                if let Some(some_fragments) = fragment_from_container(inline_node) {
                                    fragments.extend(some_fragments);
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        // Ignore text
        _ => {}
    }

    for child in node_children {
        collect_fragments(tree, child, fragments);
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











