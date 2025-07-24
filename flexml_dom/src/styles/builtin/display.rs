use crate::styles::context::{Display, StyleContext};
use crate::styles::builtin::{apply_match_style, BuiltInStyle};
use crate::styles::style::{StyleValue};
use crate::styles::style::StyleValueParser::MatchParser;

fn apply_display(value: &StyleValue, context: &mut StyleContext) {
    apply_match_style(value, &mut context.display, &[
        Display::Block,
        Display::Inline,
        Display::InlineBlock,
        Display::Flex,
        Display::Table
    ]);
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
