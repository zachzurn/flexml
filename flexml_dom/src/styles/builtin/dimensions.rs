use crate::styles::builtin::BuiltInStyle;
use crate::styles::style::StyleValue;
use crate::styles::style::StyleValueParser::{PositiveNumberParser};

fn apply_width(_: &StyleValue) {
    todo!()
}
pub static WIDTH: BuiltInStyle = BuiltInStyle {
    name: "width",
    parser: PositiveNumberParser,
    styles: &[],
    apply_style: apply_width,
};

fn apply_max_width(_: &StyleValue) {
    todo!()
}
pub static MAX_WIDTH: BuiltInStyle = BuiltInStyle {
    name: "maxWidth",
    parser: PositiveNumberParser,
    styles: &[],
    apply_style: apply_max_width,
};

fn apply_min_width(_: &StyleValue) {
    todo!()
}
pub static MIN_WIDTH: BuiltInStyle = BuiltInStyle {
    name: "minWidth",
    parser: PositiveNumberParser,
    styles: &[],
    apply_style: apply_min_width,
};

fn apply_height(_: &StyleValue) {
    todo!()
}
pub static HEIGHT: BuiltInStyle = BuiltInStyle {
    name: "height",
    parser: PositiveNumberParser,
    styles: &[],
    apply_style: apply_height,
};

fn apply_max_height(_: &StyleValue) {
    todo!()
}
pub static MAX_HEIGHT: BuiltInStyle = BuiltInStyle {
    name: "maxHeight",
    parser: PositiveNumberParser,
    styles: &[],
    apply_style: apply_max_height,
};

fn apply_min_height(_: &StyleValue) {
    todo!()
}
pub static MIN_HEIGHT: BuiltInStyle = BuiltInStyle {
    name: "minHeight",
    parser: PositiveNumberParser,
    styles: &[],
    apply_style: apply_min_height,
};
