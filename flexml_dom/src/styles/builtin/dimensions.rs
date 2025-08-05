use crate::styles::context::StyleContext;
use crate::styles::builtin::{dimension_to_context, BuiltInStyle};
use crate::styles::style::StyleValue;
use crate::styles::style::StyleValueParser::{PositiveNumber};

fn apply_width(value: &StyleValue, context: &mut StyleContext) {
    if let Some(d) = dimension_to_context(value) {
        context.set_width(d);
    }
}
pub static WIDTH: BuiltInStyle = BuiltInStyle {
    name: "width",
    parser: PositiveNumber,
    styles: &[],
    apply_style: apply_width,
};

fn apply_max_width(value: &StyleValue, context: &mut StyleContext) {
    if let Some(d) = dimension_to_context(value) {
        context.set_max_width(d);
    }
}
pub static MAX_WIDTH: BuiltInStyle = BuiltInStyle {
    name: "maxWidth",
    parser: PositiveNumber,
    styles: &[],
    apply_style: apply_max_width,
};

fn apply_min_width(value: &StyleValue, context: &mut StyleContext) {
    if let Some(d) = dimension_to_context(value) {
        context.set_min_width(d);
    }
}
pub static MIN_WIDTH: BuiltInStyle = BuiltInStyle {
    name: "minWidth",
    parser: PositiveNumber,
    styles: &[],
    apply_style: apply_min_width,
};

fn apply_height(value: &StyleValue, context: &mut StyleContext) {
    if let Some(d) = dimension_to_context(value) {
        context.set_height(d);
    }
}
pub static HEIGHT: BuiltInStyle = BuiltInStyle {
    name: "height",
    parser: PositiveNumber,
    styles: &[],
    apply_style: apply_height,
};

fn apply_max_height(value: &StyleValue, context: &mut StyleContext) {
    if let Some(d) = dimension_to_context(value) {
        context.set_max_height(d);
    }
}
pub static MAX_HEIGHT: BuiltInStyle = BuiltInStyle {
    name: "maxHeight",
    parser: PositiveNumber,
    styles: &[],
    apply_style: apply_max_height,
};

fn apply_min_height(value: &StyleValue, context: &mut StyleContext) {
    if let Some(d) = dimension_to_context(value) {
        context.set_min_height(d);
    }
}
pub static MIN_HEIGHT: BuiltInStyle = BuiltInStyle {
    name: "minHeight",
    parser: PositiveNumber,
    styles: &[],
    apply_style: apply_min_height,
};
