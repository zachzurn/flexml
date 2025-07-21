use crate::styles::builtin::{BuiltInStyle};
use crate::styles::style::{StyleValue};
use crate::styles::style::StyleValueParser::{FloatParser};

fn apply_opacity(_: &StyleValue) {
    todo!()
}

pub static OPACITY: BuiltInStyle = BuiltInStyle {
    name: "opacity",
    parser: FloatParser,
    styles: &[],
    apply_style: apply_opacity,
};
