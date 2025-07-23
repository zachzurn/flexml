use crate::layout::context::StyleContext;
use crate::styles::builtin::{apply_dimension, apply_length, BuiltInStyle};
use crate::styles::style::StyleValue;
use crate::styles::style::StyleValueParser::{PositiveNumberParser};

fn apply_width(value: &StyleValue, context: &mut StyleContext) {
    apply_dimension(value, &mut context.width);
}
pub static WIDTH: BuiltInStyle = BuiltInStyle {
    name: "width",
    parser: PositiveNumberParser,
    styles: &[],
    apply_style: apply_width,
};

fn apply_max_width(value: &StyleValue, context: &mut StyleContext) {
    apply_dimension(value, &mut context.max_width);
}
pub static MAX_WIDTH: BuiltInStyle = BuiltInStyle {
    name: "maxWidth",
    parser: PositiveNumberParser,
    styles: &[],
    apply_style: apply_max_width,
};

fn apply_min_width(value: &StyleValue, context: &mut StyleContext) {
    apply_dimension(value, &mut context.min_width);
}
pub static MIN_WIDTH: BuiltInStyle = BuiltInStyle {
    name: "minWidth",
    parser: PositiveNumberParser,
    styles: &[],
    apply_style: apply_min_width,
};

fn apply_height(value: &StyleValue, context: &mut StyleContext) {
    apply_dimension(value, &mut context.height);
}
pub static HEIGHT: BuiltInStyle = BuiltInStyle {
    name: "height",
    parser: PositiveNumberParser,
    styles: &[],
    apply_style: apply_height,
};

fn apply_max_height(value: &StyleValue, context: &mut StyleContext) {
    apply_dimension(value, &mut context.max_height);
}
pub static MAX_HEIGHT: BuiltInStyle = BuiltInStyle {
    name: "maxHeight",
    parser: PositiveNumberParser,
    styles: &[],
    apply_style: apply_max_height,
};

fn apply_min_height(value: &StyleValue, context: &mut StyleContext) {
    apply_dimension(value, &mut context.min_height);
}
pub static MIN_HEIGHT: BuiltInStyle = BuiltInStyle {
    name: "minHeight",
    parser: PositiveNumberParser,
    styles: &[],
    apply_style: apply_min_height,
};
