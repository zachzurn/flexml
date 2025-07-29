use std::ops::Range;
use parley::{Alignment, AlignmentOptions, Brush, FontWeight, InlineBox, PositionedLayoutItem, RangedBuilder, StyleProperty};
use parley::StyleProperty::FontSize;
use taffy::{LayoutInput, LayoutOutput, LayoutPartialTree, NodeId, Size};
use crate::layout::tree::{LayoutNodeKind, LayoutTree};
use crate::styles::context::StyleContext;

fn parley_style<'a>(style: &StyleContext) -> Vec<StyleProperty<'a, [u8; 4]>> {
    let mut styles = vec![];
    //TODO fill out the rest of the styles here
    styles.push(FontSize(style.font_size));
    styles
}

enum InlineItemBuilder<'a> {
    Text{range: Range<usize>, styles: Vec<StyleProperty<'a, [u8; 4]>>},
    Inline{id: u64, index: usize, width: f32, height: f32},
}

/// Layout an inline container.
/// We use the tree to compute inline blocks
pub fn compute_inline_layout (tree: &mut LayoutTree, node_id: NodeId, inputs: LayoutInput) -> LayoutOutput {
    let node = tree.node_from_id(node_id);

    let mut i_text = String::new();
    let mut i_items: Vec<InlineItemBuilder> = Vec::new();

    for child_id in tree.node_from_id(node_id).children.clone() {
        let child_node = tree.node_from_id(NodeId::from(child_id));
        match child_node.kind {
            LayoutNodeKind::Text => {
                if let Some(text) = &child_node.text {
                    let start = i_text.len();
                    i_text.push_str(text);
                    let end = i_text.len();
                    i_items.push(InlineItemBuilder::Text { range: start..end, styles: parley_style(&child_node.style_context) })
                }
            }
            LayoutNodeKind::InlineContent | LayoutNodeKind::Container => {
                let layout = tree.compute_child_layout(NodeId::from(child_id), inputs);
                let width = layout.size.width;
                let height = layout.size.height;
                let index = i_text.len();

                i_items.push(InlineItemBuilder::Inline { id: child_id as u64, index, width, height });
            }
        }
    }

    let mut builder = tree.parley_layout_context.ranged_builder(&mut tree.parley_font_context, &i_text, tree.parley_display_scale, true);

    for i_item in i_items {
        match i_item {
            InlineItemBuilder::Inline { id, index, width, height } => {
                builder.push_inline_box(InlineBox{
                    id, width, height, index
                });
            },
            InlineItemBuilder::Text { range, styles } => {
                for s in styles {
                    builder.push(s, range.clone());
                }
            }
        }
    }

    let mut layout = builder.build(&i_text);

    let available_width = Some(inputs.available_space.width.unwrap_or(f32::INFINITY));
    layout.break_all_lines(available_width);
    layout.align(available_width, Alignment::Start, AlignmentOptions::default());

    let total_width = layout.width();
    let total_height = layout.height();

    let node_mut = tree.node_from_id_mut(node_id);
    node_mut.inline_layout = Some(layout);

    LayoutOutput::from_outer_size(Size{ width: total_width, height: total_height })
}