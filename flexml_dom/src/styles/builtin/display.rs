use crate::styles::builtin::{BuiltInStyle};
use crate::styles::style::{StyleValue};
use crate::styles::style::StyleValueParser::MatchParser;

fn apply_display(_: &StyleValue) {
    todo!()
}

pub static DISPLAY: BuiltInStyle = BuiltInStyle {
    name: "display",
    parser: MatchParser(&[
        "block",
        "inline",
        "inline-block",
        "flex",
        "table",
    ]),
    styles: &[
        ("box", StyleValue::Match(0)),
        ("block", StyleValue::Match(0)),
        ("inline", StyleValue::Match(1)),
        ("span", StyleValue::Match(2)),
        ("flex", StyleValue::Match(3)),
        ("table", StyleValue::Match(4)),
    ],
    apply_style: apply_display,
};
