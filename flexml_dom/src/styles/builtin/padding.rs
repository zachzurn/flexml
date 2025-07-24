use crate::styles::context::StyleContext;
use crate::styles::builtin::{apply_dimension, BuiltInStyle};
use crate::styles::style::StyleValue;
use crate::styles::style::StyleValueParser::NumberParser;

fn apply_padding(value: &StyleValue, context: &mut StyleContext) {
    apply_dimension(value, &mut context.padding_top);
    apply_dimension(value, &mut context.padding_bottom);
    apply_dimension(value, &mut context.padding_left);
    apply_dimension(value, &mut context.padding_right);
}

pub static PADDING: BuiltInStyle = BuiltInStyle {
    name: "padding",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_padding,
};

fn apply_padding_top(value: &StyleValue, context: &mut StyleContext) {
    apply_dimension(value, &mut context.padding_top);
}

pub static PADDING_TOP: BuiltInStyle = BuiltInStyle {
    name: "paddingTop",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_padding_top,
};

fn apply_padding_right(value: &StyleValue, context: &mut StyleContext) {
    apply_dimension(value, &mut context.padding_right);
}

pub static PADDING_RIGHT: BuiltInStyle = BuiltInStyle {
    name: "paddingRight",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_padding_right,
};

fn apply_padding_bottom(value: &StyleValue, context: &mut StyleContext) {
    apply_dimension(value, &mut context.padding_bottom);
}

pub static PADDING_BOTTOM: BuiltInStyle = BuiltInStyle {
    name: "paddingBottom",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_padding_bottom,
};

fn apply_padding_left(value: &StyleValue, context: &mut StyleContext) {
    apply_dimension(value, &mut context.padding_left);
}

pub static PADDING_LEFT: BuiltInStyle = BuiltInStyle {
    name: "paddingLeft",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_padding_left,
};
