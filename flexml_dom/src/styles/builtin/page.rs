use crate::styles::context::{StyleContext};
use crate::styles::builtin::{dimension_to_context, float_to_context, BuiltInStyle};
use crate::styles::style::StyleValue;
use crate::styles::style::StyleValueParser::{Float, PositiveNumber};


fn apply_page_width(value: &StyleValue, context: &mut StyleContext) {
    if context.is_root() && let Some(d) = dimension_to_context(value) {
        context.set_width(d);
    }
}

pub static PAGE_WIDTH: BuiltInStyle = BuiltInStyle {
    name: "pageWidth",
    parser: PositiveNumber,
    styles: &[],
    apply_style: apply_page_width,
};


fn apply_page_height(value: &StyleValue, context: &mut StyleContext) {
    if context.is_root() && let Some(d) = dimension_to_context(value) {
        context.set_height(d);
    }
}

pub static PAGE_HEIGHT: BuiltInStyle = BuiltInStyle {
    name: "pageHeight",
    parser: PositiveNumber,
    styles: &[],
    apply_style: apply_page_height,
};


fn apply_page_dpi(value: &StyleValue, context: &mut StyleContext) {
    if context.is_root() && let Some(f) = float_to_context(value) {
        context.set_dpi(f);
    }
}

pub static PAGE_DPI: BuiltInStyle = BuiltInStyle {
    name: "pixelsPerInch",
    parser: Float,
    styles: &[],
    apply_style: apply_page_dpi,
};




