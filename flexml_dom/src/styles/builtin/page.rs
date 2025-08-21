use crate::styles::context::{Image, StyleContext};
use crate::styles::builtin::{dimension_to_context, float_to_context, BuiltInStyle};
use crate::styles::style::{StyleValue, UrlType};
use crate::styles::style::StyleValueParser::{Float, PositiveNumber, Url};

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

fn apply_base_path(value: &StyleValue, context: &mut StyleContext) {
    if context.is_root() && let StyleValue::PathUrl(_) = value {
        // TODO this should be set on some kind of root context
    };
}

pub static BASE_PATH: BuiltInStyle = BuiltInStyle {
    name: "basePath",
    parser: Url(&UrlType::Path),
    styles: &[],
    apply_style: apply_base_path
};


