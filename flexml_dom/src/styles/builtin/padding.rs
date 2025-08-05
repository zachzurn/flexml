use crate::styles::context::StyleContext;
use crate::styles::builtin::{dimension_to_context, BuiltInStyle};
use crate::styles::style::StyleValue;
use crate::styles::style::StyleValueParser::Number;

fn apply_padding(value: &StyleValue, context: &mut StyleContext) {
    if let Some(d) = dimension_to_context(value) {
        context.set_padding_top(d);
        context.set_padding_bottom(d);
        context.set_padding_left(d);
        context.set_padding_right(d);
    }
}

pub static PADDING: BuiltInStyle = BuiltInStyle {
    name: "padding",
    parser: Number,
    styles: &[],
    apply_style: apply_padding,
};

fn apply_padding_top(value: &StyleValue, context: &mut StyleContext) {
    if let Some(d) = dimension_to_context(value) {
        context.set_padding_top(d);
    }
}

pub static PADDING_TOP: BuiltInStyle = BuiltInStyle {
    name: "paddingTop",
    parser: Number,
    styles: &[],
    apply_style: apply_padding_top,
};

fn apply_padding_right(value: &StyleValue, context: &mut StyleContext) {
    if let Some(d) = dimension_to_context(value) {
        context.set_padding_right(d);
    }
}

pub static PADDING_RIGHT: BuiltInStyle = BuiltInStyle {
    name: "paddingRight",
    parser: Number,
    styles: &[],
    apply_style: apply_padding_right,
};

fn apply_padding_bottom(value: &StyleValue, context: &mut StyleContext) {
    if let Some(d) = dimension_to_context(value) {
        context.set_padding_bottom(d);
    }
}

pub static PADDING_BOTTOM: BuiltInStyle = BuiltInStyle {
    name: "paddingBottom",
    parser: Number,
    styles: &[],
    apply_style: apply_padding_bottom,
};

fn apply_padding_left(value: &StyleValue, context: &mut StyleContext) {
    if let Some(d) = dimension_to_context(value) {
        context.set_padding_left(d);
    }
}

pub static PADDING_LEFT: BuiltInStyle = BuiltInStyle {
    name: "paddingLeft",
    parser: Number,
    styles: &[],
    apply_style: apply_padding_left,
};
