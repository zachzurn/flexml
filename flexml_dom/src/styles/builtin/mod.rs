use crate::styles::context::{Color, Dimension, StyleContext};
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
pub mod page;

pub struct BuiltInStyle {
    pub name: &'static str,
    pub parser: StyleValueParser,
    pub styles: &'static [(&'static str, StyleValue)],
    pub apply_style: fn(&StyleValue, &mut StyleContext),
}

fn style_context_match<T>(
    value: &StyleValue,
    variants: &[T],
) -> Option<T> where
    T: Copy,
{
    if let StyleValue::Match(i) = value && let Some(val) = variants.get(*i as usize) {
        Some(*val)
    } else{
      None
   }
}

fn style_context_color(value: &StyleValue) -> Option<Color> {
    if let StyleValue::Color(c) = value {
        Some(Color(c.r,c.g,c.b,c.a))
    } else {
        None
    }
}

fn dimension_to_context(value: &StyleValue) -> Option<Dimension> {
    match value {
        StyleValue::PositiveNumber(dimension) |
        StyleValue::NegativeNumber(dimension)
        => Some(*dimension),
        _ => None
    }
}

fn float_to_context(value: &StyleValue) -> Option<f32> {
    if let StyleValue::Float(f) = value { Some(*f) }
    else { None }
}


fn length_to_context(value: &StyleValue, variants: &[Dimension]) -> Option<Dimension> {
    match value {
        StyleValue::PositiveNumber(dimension) |
        StyleValue::NegativeNumber(dimension) => {
            Some(*dimension)
        },
        StyleValue::Match(i) => {
            variants.get(*i as usize).copied()
        }
        _ => None
    }
}

pub static ROOT_STYLE_NAME: &str = "flexml";

pub static DEFAULT_BUILTINS : &[&BuiltInStyle] = &[
    &page::PAGE_HEIGHT,
    &page::PAGE_WIDTH,
    &page::PAGE_DPI,
    &page::BASE_PATH,

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