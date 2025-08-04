use std::ops::Range;
use parley::{Alignment, AlignmentOptions, FontWeight, InlineBox, LineHeight, StyleProperty};
use taffy::{compute_leaf_layout, AvailableSpace, LayoutInput, LayoutOutput, LayoutPartialTree, NodeId, Point, Size, SizingMode};
use crate::layout::tree::{LayoutNodeKind, LayoutTree};
use crate::styles::context::{FontStyle, StyleContext};

fn parley_style<'a>(style: &StyleContext) -> Vec<StyleProperty<'a, [u8; 4]>> {
    let em = style.resolved_font_size;
    let rem = style.resolved_root_font_size;
    let dpi = style.dpi;

    let font_style = match style.font_style {
        FontStyle::Normal => parley::FontStyle::Normal,
        FontStyle::Italic => parley::FontStyle::Italic,
        FontStyle::Oblique => parley::FontStyle::Oblique(None)
    };

    vec![
        StyleProperty::FontSize(style.resolved_font_size),
        StyleProperty::LineHeight(LineHeight::Absolute(style.line_height.to_pixels(em, rem, em, dpi))),
        StyleProperty::LetterSpacing(style.letter_spacing.to_pixels(em, rem, em, dpi)),
        StyleProperty::FontWeight(FontWeight::new(style.font_weight as f32)),
        StyleProperty::FontStyle(font_style),
        StyleProperty::Brush([style.color.0, style.color.1, style.color.2, style.color.3]),
        // TODO add other styles
    ]
}

enum InlineItemBuilder<'a> {
    Text{range: Range<usize>, styles: Vec<StyleProperty<'a, [u8; 4]>>},
    Inline{id: u64, index: usize, width: f32, height: f32},
}

/// Layout an inline container.
/// We use the tree to compute inline blocks
pub(super) fn compute_inline_layout (tree: &mut LayoutTree, node_id: NodeId, inputs: LayoutInput) -> LayoutOutput {
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
            // Containers directly in an inline layout are considered inline block
            LayoutNodeKind::Container => {
                let inline_index = i_text.len();
                let mut inline_block = inputs; //Copy
                inline_block.sizing_mode = SizingMode::ContentSize;
                inline_block.available_space.height = AvailableSpace::MaxContent;
                inline_block.available_space.width = AvailableSpace::MinContent;

                let layout = tree.compute_child_layout(NodeId::from(child_id), inline_block);
                //tree.compute_layout(child_id, inline_block.available_space, true);
                //let layout = tree.node_from_id(NodeId::from(child_id)).final_layout;
                let width = layout.content_size.width;
                let height = layout.content_size.height;

                i_items.push(InlineItemBuilder::Inline { id: child_id as u64, index: inline_index, width, height });
            }

            // Inline content should only contain Containers and Text
            // Other InlineContent is ignored as is considered an error
            _ => {  }
        }
    }

    let mut builder = tree.context.parley_layout_context.ranged_builder(&mut tree.context.parley_font_context, &i_text, tree.context.parley_display_scale, true);

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

    let baseline_y = layout.lines()
        .next()
        .map(|line| line.metrics().baseline)
        .unwrap_or(0.0);

    let first_baselines = Point { x: None, y: Some(baseline_y) };

    let node_mut = tree.node_from_id_mut(node_id);
    node_mut.inline_layout = Some(layout);

    let size = Size { width: total_width, height: total_height };
    let content_size = size;

    LayoutOutput::from_sizes_and_baselines(size, content_size, first_baselines)
}