use crate::styles::builtin::{BuiltInStyle};
use crate::styles::style::{StyleValue};
use crate::styles::style::StyleValueParser::MatchParser;

fn apply_white_space(_: &StyleValue) {
    todo!()
}

pub static WHITE_SPACE: BuiltInStyle = BuiltInStyle {
    name: "whiteSpace",
    parser: MatchParser(&["normal", "nowrap", "pre", "pre-wrap", "pre-line"]),
    styles: &[
        ("pre", StyleValue::Match(3)),
        ("code", StyleValue::Match(3)),
    ],
    apply_style: apply_white_space,
};
