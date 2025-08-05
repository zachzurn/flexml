use crate::styles::context::StyleContext;
use crate::styles::builtin::{dimension_to_context, BuiltInStyle};
use crate::styles::style::StyleValue;
use crate::styles::style::StyleValueParser::Number;

fn apply_margin(value: &StyleValue, context: &mut StyleContext) {
    if let Some(d) = dimension_to_context(value) {
        context.set_margin_top(d);
        context.set_margin_bottom(d);
        context.set_margin_left(d);
        context.set_margin_right(d);
    }
}

pub static MARGIN: BuiltInStyle = BuiltInStyle {
    name: "margin",
    parser: Number,
    styles: &[],
    apply_style: apply_margin,
};

fn apply_margin_top(value: &StyleValue, context: &mut StyleContext) {
    if let Some(d) = dimension_to_context(value) {
        context.set_margin_top(d);
    }
}

pub static MARGIN_TOP: BuiltInStyle = BuiltInStyle {
    name: "marginTop",
    parser: Number,
    styles: &[],
    apply_style: apply_margin_top,
};

fn apply_margin_right(value: &StyleValue, context: &mut StyleContext) {
    if let Some(d) = dimension_to_context(value) {
        context.set_margin_right(d);
    }
}

pub static MARGIN_RIGHT: BuiltInStyle = BuiltInStyle {
    name: "marginRight",
    parser: Number,
    styles: &[],
    apply_style: apply_margin_right,
};

fn apply_margin_bottom(value: &StyleValue, context: &mut StyleContext) {
    if let Some(d) = dimension_to_context(value) {
        context.set_margin_bottom(d);
    }
}

pub static MARGIN_BOTTOM: BuiltInStyle = BuiltInStyle {
    name: "marginBottom",
    parser: Number,
    styles: &[],
    apply_style: apply_margin_bottom,
};

fn apply_margin_left(value: &StyleValue, context: &mut StyleContext) {
    if let Some(d) = dimension_to_context(value) {
        context.set_margin_left(d);
    }
}

pub static MARGIN_LEFT: BuiltInStyle = BuiltInStyle {
    name: "marginLeft",
    parser: Number,
    styles: &[],
    apply_style: apply_margin_left,
};
