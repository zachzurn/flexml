use crate::styles::context::{AlignContent, AlignItems, Dimension, Display, FlexDirection, FlexWrap, JustifyContent, Length, StyleContext};
use std::default::Default;

fn to_taffy_dimension(dim: Dimension) -> taffy::style::Dimension {
    match dim {
        Dimension::Px(px) => taffy::style::Dimension::length(px as f32),
        Dimension::Percent(pct) => taffy::style::Dimension::percent(pct),
    }
}

fn to_taffy_lpa(dim: Dimension) -> taffy::style::LengthPercentageAuto {
    match dim {
        Dimension::Px(px) => taffy::style::LengthPercentageAuto::length(px as f32),
        Dimension::Percent(pct) => taffy::style::LengthPercentageAuto::percent(pct),
    }
}

fn to_taffy_lp(dim: Dimension) -> taffy::style::LengthPercentage {
    match dim {
        Dimension::Px(px) => taffy::style::LengthPercentage::length(px as f32),
        Dimension::Percent(pct) => taffy::style::LengthPercentage::percent(pct),
    }
}

fn to_taffy_size(w: Dimension, h: Dimension) -> taffy::Size<taffy::style::Dimension> {
    taffy::Size{
        width: to_taffy_dimension(w),
        height: to_taffy_dimension(h),
    }
}

fn to_taffy_length(length: Length) -> taffy::style::Dimension {
    match length {
        Length::Auto => taffy::style::Dimension::auto(),
        Length::Content => taffy::style::Dimension::auto(),
        Length::Px(px) => taffy::style::Dimension::length(px as f32),
        Length::Percent(pct) => taffy::style::Dimension::percent(pct),
    }
}

fn to_taffy_display(display: Display) -> taffy::style::Display {
    match display {
        Display::Block => taffy::style::Display::Block,
        Display::Inline => taffy::style::Display::Block, // We handle inline separately
        Display::InlineBlock => taffy::style::Display::Block, // We handle inline separately
        Display::Flex => taffy::style::Display::Flex,
        Display::Table => taffy::style::Display::None,
    }
}

fn to_taffy_flex_direction(fd: FlexDirection) -> taffy::style::FlexDirection {
    match fd {
        FlexDirection::Row => taffy::style::FlexDirection::Row,
        FlexDirection::RowReverse => taffy::style::FlexDirection::RowReverse,
        FlexDirection::Column => taffy::style::FlexDirection::Column,
        FlexDirection::ColumnReverse => taffy::style::FlexDirection::ColumnReverse,
    }
}

fn to_taffy_flex_wrap(fw: FlexWrap) -> taffy::style::FlexWrap {
    match fw {
        FlexWrap::NoWrap => taffy::style::FlexWrap::NoWrap,
        FlexWrap::Wrap => taffy::style::FlexWrap::Wrap,
        FlexWrap::WrapReverse => taffy::style::FlexWrap::WrapReverse,
    }
}

fn to_taffy_justify_content(jc: JustifyContent) -> Option<taffy::style::JustifyContent> {
    Some(match jc {
        JustifyContent::FlexStart => taffy::style::JustifyContent::FlexStart,
        JustifyContent::FlexEnd => taffy::style::JustifyContent::FlexEnd,
        JustifyContent::Center => taffy::style::JustifyContent::Center,
        JustifyContent::SpaceBetween => taffy::style::JustifyContent::SpaceBetween,
        JustifyContent::SpaceAround => taffy::style::JustifyContent::SpaceAround,
        JustifyContent::SpaceEvenly => taffy::style::JustifyContent::SpaceEvenly,
    })
}

fn to_taffy_align_items(ai: AlignItems) -> Option<taffy::style::AlignItems> {
    Some(match ai {
        AlignItems::FlexStart => taffy::style::AlignItems::FlexStart,
        AlignItems::FlexEnd => taffy::style::AlignItems::FlexEnd,
        AlignItems::Center => taffy::style::AlignItems::Center,
        AlignItems::Baseline => taffy::style::AlignItems::Baseline,
        AlignItems::Stretch => taffy::style::AlignItems::Stretch,
    })
}

fn to_taffy_align_content(ac: AlignContent) -> Option<taffy::style::AlignContent> {
    Some(match ac {
        AlignContent::FlexStart => taffy::style::AlignContent::FlexStart,
        AlignContent::FlexEnd => taffy::style::AlignContent::FlexEnd,
        AlignContent::Center => taffy::style::AlignContent::Center,
        AlignContent::SpaceBetween => taffy::style::AlignContent::SpaceBetween,
        AlignContent::SpaceAround => taffy::style::AlignContent::SpaceAround,
        AlignContent::Stretch => taffy::style::AlignContent::Stretch,
    })
}

pub (super) fn style_context_to_taffy(style_context: &StyleContext) -> taffy::style::Style {
    taffy::style::Style {
        display: to_taffy_display(style_context.display),

        margin: taffy::geometry::Rect {
            left: to_taffy_lpa(style_context.margin_left),
            right: to_taffy_lpa(style_context.margin_right),
            top: to_taffy_lpa(style_context.margin_top),
            bottom: to_taffy_lpa(style_context.margin_bottom),
        },

        padding: taffy::geometry::Rect {
            left: to_taffy_lp(style_context.padding_left),
            right: to_taffy_lp(style_context.padding_right),
            top: to_taffy_lp(style_context.padding_top),
            bottom: to_taffy_lp(style_context.padding_bottom),
        },

        border: taffy::geometry::Rect {
            left: to_taffy_lp(style_context.border_width),  // assuming uniform border width
            right: to_taffy_lp(style_context.border_width),
            top: to_taffy_lp(style_context.border_width),
            bottom: to_taffy_lp(style_context.border_width),
        },

        flex_direction: to_taffy_flex_direction(style_context.flex_direction),
        flex_wrap: to_taffy_flex_wrap(style_context.flex_wrap),
        justify_content: to_taffy_justify_content(style_context.justify_content),
        align_items: to_taffy_align_items(style_context.align_items),
        align_self: None,
        justify_items: None,
        justify_self: None,
        align_content: to_taffy_align_content(style_context.align_content),

        flex_grow: style_context.flex_grow,
        flex_shrink: style_context.flex_shrink,
        flex_basis: to_taffy_length(style_context.flex_basis),

        size: to_taffy_size(style_context.width, style_context.height),
        min_size: to_taffy_size(style_context.min_width, style_context.min_height),
        max_size: to_taffy_size(style_context.max_width, style_context.max_height),

        item_is_table: false,
        item_is_replaced: false,
        scrollbar_width: 0.0,

        aspect_ratio: None,

        ..Default::default()
    }
}

