use crate::layout::context::StyleContext;
use crate::styles::builtin::{apply_float, BuiltInStyle};
use crate::styles::style::{StyleValue};
use crate::styles::style::StyleValueParser::{FloatParser};

fn apply_opacity(value: &StyleValue, context: &mut StyleContext) {
    apply_float(value, &mut context.opacity);
}

pub static OPACITY: BuiltInStyle = BuiltInStyle {
    name: "opacity",
    parser: FloatParser,
    styles: &[],
    apply_style: apply_opacity,
};
