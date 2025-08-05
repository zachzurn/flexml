use crate::styles::context::StyleContext;
use crate::styles::builtin::{float_to_context, BuiltInStyle};
use crate::styles::style::{StyleValue};
use crate::styles::style::StyleValueParser::{Float};

fn apply_opacity(value: &StyleValue, context: &mut StyleContext) {
    if let Some(f) = float_to_context(value) {
        context.set_opacity(f);
    }
}

pub static OPACITY: BuiltInStyle = BuiltInStyle {
    name: "opacity",
    parser: Float,
    styles: &[],
    apply_style: apply_opacity,
};
