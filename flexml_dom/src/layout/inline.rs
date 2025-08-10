use crate::layout::tree::{LayoutNodeKind, LayoutTree};
use crate::styles::context::{FontStyle, StyleContext, TextDecoration, TextTransform, WhiteSpace};
use parley::{Alignment, AlignmentOptions, FontWeight, InlineBox, LineHeight, StyleProperty};
use std::ops::Range;
use taffy::{LayoutInput, LayoutOutput, LayoutPartialTree, NodeId, Point, Size};
use unicode_segmentation::UnicodeSegmentation;

fn parley_style<'a>(style: &StyleContext) -> Vec<StyleProperty<'a, [u8; 4]>> {
    let em = style.resolved_font_size();
    let rem = style.resolved_root_font_size();
    let dpi = style.dpi();

    let font_style = match style.font_style() {
        FontStyle::Normal => parley::FontStyle::Normal,
        FontStyle::Italic => parley::FontStyle::Italic,
        FontStyle::Oblique => parley::FontStyle::Oblique(None)
    };

    let color = style.color();

    vec![
        StyleProperty::FontSize(style.resolved_font_size()),
        StyleProperty::LineHeight(LineHeight::Absolute(style.line_height().as_pixels(em, rem, em, dpi))),
        StyleProperty::LetterSpacing(style.letter_spacing().as_pixels(em, rem, em, dpi)),
        StyleProperty::FontWeight(FontWeight::new(style.font_weight() as f32)),
        StyleProperty::FontStyle(font_style),
        StyleProperty::Brush([color.0, color.1, color.2, color.3]),
        StyleProperty::WordSpacing(style.word_spacing().as_pixels(em, rem, em, dpi)),
        StyleProperty::Underline(matches!(style.text_decoration(), TextDecoration::Underline)),
        StyleProperty::Strikethrough(matches!(style.text_decoration(), TextDecoration::LineThrough)),
    ]
}

enum InlineItemBuilder<'a> {
    Text{range: Range<usize>, styles: Vec<StyleProperty<'a, [u8; 4]>>},
    Inline{id: NodeId, index: usize, width: f32, height: f32},
}

fn transform_with_ws(
    text: &str,
    preserve_whitespace: WhiteSpace,
    text_transform: TextTransform,
    allow_pre_ws: bool,
) -> (String, bool) {
    let transformed = match text_transform {
        TextTransform::None => text.to_string(),
        TextTransform::Uppercase => text.to_uppercase(),
        TextTransform::Lowercase => text.to_lowercase(),
        TextTransform::Capitalize => {
            text.split_word_bounds()
                .map(|word| {
                    let mut c = word.chars();
                    match c.next() {
                        None => "".to_string(),
                        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                    }
                })
                .collect::<String>()
        }
    };

    let mut result = String::new();
    let mut trailing_ws = false;

    match preserve_whitespace {
        WhiteSpace::Normal | WhiteSpace::NoWrap => {
            let words: Vec<&str> = transformed
                .split_whitespace()
                .collect();

            if words.is_empty() {
                return (String::new(), false);
            }

            result = words.join(" ");

            trailing_ws = {
                let trimmed_len = text.trim_end().len();
                trimmed_len < text.len()
            };
        }
        WhiteSpace::Pre | WhiteSpace::PreWrap | WhiteSpace::PreLine => {
            if !allow_pre_ws {
                result = transformed.trim_start().to_string();
            } else {
                result = transformed;
            }

            trailing_ws = {
                let trimmed_len = result.trim_end_matches(|c: char| c.is_whitespace()).len();
                trimmed_len < result.len()
            };
        }
    }

    (result, trailing_ws)
}

/// Layout an inline container.
/// We use the tree to compute inline blocks
pub(super) fn compute_inline_layout (tree: &mut LayoutTree, node_id: NodeId, inputs: LayoutInput) -> LayoutOutput {
    let mut i_text = String::new();
    let mut i_items: Vec<InlineItemBuilder> = Vec::new();

    let node = tree.node_from_id(node_id);
    let ws = node.style_context.white_space();
    let transform = node.style_context.text_transform();
    let mut trailing_ws = false;

    for child_id in tree.node_from_id(node_id).children.clone() {
        let child_node = tree.node_from_id(child_id);
        match child_node.kind {
            LayoutNodeKind::Text => {
                if let Some(text) = &child_node.text {
                    let (transformed, has_trailing_ws) = transform_with_ws(text, ws, transform, !trailing_ws);
                    trailing_ws = has_trailing_ws;

                    let start = i_text.len();
                    i_text.push_str(&transformed);
                    let end = i_text.len();
                    i_items.push(InlineItemBuilder::Text { range: start..end, styles: parley_style(&child_node.style_context) })
                }
            }
            // Containers directly in an inline layout are always treated as inline block
            LayoutNodeKind::Container => {
                let inline_index = i_text.len();

                let layout = tree.compute_child_layout(child_id, inputs);
                let width = layout.content_size.width;
                let height = layout.content_size.height;

                i_items.push(InlineItemBuilder::Inline { id: child_id, index: inline_index, width, height });
            }

            // Inline content should only contain Containers and Text
            // Any other layout nodes are dropped
            _ => {  }
        }
    }

    let mut builder = tree.context.parley_layout_context
        .ranged_builder(&mut tree.context.parley_font_context, &i_text, tree.context.parley_display_scale, true);

    for i_item in i_items {
        match i_item {
            InlineItemBuilder::Inline { id, index, width, height } => {
                builder.push_inline_box(InlineBox{
                    id: id.into(), width, height, index
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