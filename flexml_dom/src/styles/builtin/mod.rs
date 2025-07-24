use crate::styles::context::{Color, Dimension, Length, StyleContext};
use crate::styles::style::{StyleValue, StyleValueParser};

pub mod display;
pub mod white_space;
pub mod margin;
pub mod padding;
pub mod opacity;
pub mod flex;
pub mod border;
pub mod bg;
pub mod text;
pub mod dimensions;

pub struct BuiltInStyle {
    pub name: &'static str,
    pub parser: StyleValueParser,
    pub styles: &'static [(&'static str, StyleValue)],
    pub apply_style: fn(&StyleValue, &mut StyleContext),
}


/// Helper for match type styles
fn apply_match_style<T>(
    value: &StyleValue,
    context_field: &mut T,
    variants: &[T],
) where
    T: Copy,
{
    if let StyleValue::Match(i) = value {
        if let Some(val) = variants.get(*i as usize) {
            *context_field = *val;
        }
    }
}

fn apply_color(
    value: &StyleValue,
    context_field: &mut Color,
) {
    match value {
        StyleValue::Color(c) => {
            *context_field = Color(c.r,c.g,c.b,c.a);
        },
        _ => return
    }
}

fn apply_float(
    value: &StyleValue,
    context_field: &mut f32,
) {
    match value {
        StyleValue::Float(v) => {
            *context_field = *v;
        },
        _ => return
    }
}


fn apply_dimension(
    value: &StyleValue,
    context_field: &mut Dimension,
) {
    match value {
        StyleValue::Number(number) => {
            *context_field = Dimension::Px(*number as i32);
        },
        StyleValue::NegativeNumber(number) => {
            *context_field = Dimension::Px(0 - *number as i32);
        },
        StyleValue::Percent(percent) => {
            *context_field = Dimension::Percent(percent.get())
        }
        _ => return
    }
}


fn apply_length(
    value: &StyleValue,
    context_field: &mut Length,
    variants: &[Length],
) {
    match value {
        StyleValue::Number(number) => {
            *context_field = Length::Px(*number as i32);
        },
        StyleValue::NegativeNumber(number) => {
            *context_field = Length::Px(0 - *number as i32);
        },
        StyleValue::Percent(percent) => {
            *context_field = Length::Percent(percent.get())
        }
        StyleValue::Match(i) => {
            if let Some(val) = variants.get(*i as usize) {
                *context_field = *val;
            }
        }
        _ => return
    }
}



pub static DEFAULT_BUILTINS : &[&'static BuiltInStyle] = &[
    &display::DISPLAY,
    &white_space::WHITE_SPACE,
    &opacity::OPACITY,

    &margin::MARGIN,
    &margin::MARGIN_TOP,
    &margin::MARGIN_BOTTOM,
    &margin::MARGIN_LEFT,
    &margin::MARGIN_RIGHT,

    &padding::PADDING,
    &padding::PADDING_LEFT,
    &padding::PADDING_RIGHT,
    &padding::PADDING_TOP,
    &padding::PADDING_BOTTOM,

    &flex::ALIGN_CONTENT,
    &flex::ALIGN_ITEMS,
    &flex::ALIGN_SELF,
    &flex::GAP,
    &flex::COLUMN_GAP,
    &flex::ROW_GAP,
    &flex::FLEX_BASIS,
    &flex::FLEX_DIRECTION,
    &flex::FLEX_GROW,
    &flex::FLEX_SHRINK,
    &flex::JUSTIFY_CONTENT,
    &flex::FLEX_WRAP,

    &dimensions::WIDTH,
    &dimensions::MAX_WIDTH,
    &dimensions::MIN_WIDTH,
    &dimensions::HEIGHT,
    &dimensions::MAX_HEIGHT,
    &dimensions::MIN_HEIGHT,

    &text::TEXT_ALIGN,
    &text::TEXT_COLOR,
    &text::TEXT_DECORATION,
    &text::TEXT_FONT,
    &text::TEXT_SIZE,
    &text::TEXT_STYLE,
    &text::TEXT_TRANSFORM,
    &text::TEXT_LETTER_SPACING,
    &text::TEXT_LINE_HEIGHT,
    &text::TEXT_WEIGHT,
    &text::TEXT_WORD_SPACING,

    &bg::BG_COLOR,
    &bg::BG_IMAGE,
    &bg::BG_POSITION,
    &bg::BG_REPEAT,
    &bg::BG_SIZE,

    &border::BORDER_RADIUS,
    &border::BORDER_TOP_LEFT_RADIUS,
    &border::BORDER_TOP_RIGHT_RADIUS,
    &border::BORDER_BOTTOM_LEFT_RADIUS,
    &border::BORDER_BOTTOM_RIGHT_RADIUS,

    &border::BORDER_COLOR,
    &border::BORDER_STYLE,
    &border::BORDER_WIDTH,
];