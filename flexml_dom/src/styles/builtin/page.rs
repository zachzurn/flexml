use crate::styles::context::{BorderStyle, StyleContext};
use crate::styles::builtin::{apply_color, apply_dimension, apply_float, apply_match_style, BuiltInStyle};
use crate::styles::style::StyleValue;
use crate::styles::style::StyleValueParser::{ColorParser, FloatParser, MatchParser, NumberParser, PositiveNumberParser};


fn apply_page_width(value: &StyleValue, context: &mut StyleContext) {
    if context.is_root(){
        apply_dimension(value, &mut context.width)
    }
}

pub static PAGE_WIDTH: BuiltInStyle = BuiltInStyle {
    name: "pageWidth",
    parser: PositiveNumberParser,
    styles: &[],
    apply_style: apply_page_width,
};


fn apply_page_height(value: &StyleValue, context: &mut StyleContext) {
    if context.is_root() {
        apply_dimension(value, &mut context.height)
    }
}

pub static PAGE_HEIGHT: BuiltInStyle = BuiltInStyle {
    name: "pageHeight",
    parser: PositiveNumberParser,
    styles: &[],
    apply_style: apply_page_height,
};


fn apply_page_dpi(value: &StyleValue, context: &mut StyleContext) {
    if context.is_root() {
        apply_float(value, &mut context.dpi)
    }
}

pub static PAGE_DPI: BuiltInStyle = BuiltInStyle {
    name: "pixelsPerInch",
    parser: FloatParser,
    styles: &[],
    apply_style: apply_page_dpi,
};




