use crate::styles::context::{Display, StyleContext};
use crate::styles::builtin::{style_context_match, BuiltInStyle};
use crate::styles::style::{StyleValue};
use crate::styles::style::StyleValueParser::Match;

const DISPLAY_VARIANTS: &[Display] = &[
    Display::Block,
    Display::Inline,
    Display::InlineBlock,
    Display::Flex,
    Display::Table,
];

fn apply_display(value: &StyleValue, context: &mut StyleContext) {
    if let Some(v) = style_context_match(value, DISPLAY_VARIANTS) {
        context.set_display(v);
    }
}


pub static DISPLAY: BuiltInStyle = BuiltInStyle {
    name: "display",
    parser: Match(&[
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
