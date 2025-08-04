use crate::styles::context::StyleContext;
use crate::styles::builtin::{apply_dimension, BuiltInStyle};
use crate::styles::style::StyleValue;
use crate::styles::style::StyleValueParser::Number;

fn apply_margin(value: &StyleValue, context: &mut StyleContext) {
    apply_dimension(value, &mut context.margin_top);
    apply_dimension(value, &mut context.margin_bottom);
    apply_dimension(value, &mut context.margin_left);
    apply_dimension(value, &mut context.margin_right);
}

pub static MARGIN: BuiltInStyle = BuiltInStyle {
    name: "margin",
    parser: Number,
    styles: &[],
    apply_style: apply_margin,
};

fn apply_margin_top(value: &StyleValue, context: &mut StyleContext) {
    apply_dimension(value, &mut context.margin_top);
}

pub static MARGIN_TOP: BuiltInStyle = BuiltInStyle {
    name: "marginTop",
    parser: Number,
    styles: &[],
    apply_style: apply_margin_top,
};

fn apply_margin_right(value: &StyleValue, context: &mut StyleContext) {
    apply_dimension(value, &mut context.margin_right);
}

pub static MARGIN_RIGHT: BuiltInStyle = BuiltInStyle {
    name: "marginRight",
    parser: Number,
    styles: &[],
    apply_style: apply_margin_right,
};

fn apply_margin_bottom(value: &StyleValue, context: &mut StyleContext) {
    apply_dimension(value, &mut context.margin_bottom);
}

pub static MARGIN_BOTTOM: BuiltInStyle = BuiltInStyle {
    name: "marginBottom",
    parser: Number,
    styles: &[],
    apply_style: apply_margin_bottom,
};

fn apply_margin_left(value: &StyleValue, context: &mut StyleContext) {
    apply_dimension(value, &mut context.margin_left);
}

pub static MARGIN_LEFT: BuiltInStyle = BuiltInStyle {
    name: "marginLeft",
    parser: Number,
    styles: &[],
    apply_style: apply_margin_left,
};
