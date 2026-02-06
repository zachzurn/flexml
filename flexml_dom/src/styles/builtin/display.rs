use crate::styles::context::{Display, StyleContext};
use crate::styles::builtin::{match_value, style_context_match, BuiltInStyle};
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


pub static DISPLAY_MATCHES: &[&str] = &[
    "block",
    "inline",
    "inline-block",
    "flex",
    "table",
];

pub static DISPLAY: BuiltInStyle = BuiltInStyle {
    name: "display",
    parser: Match(DISPLAY_MATCHES),
    styles: &[
        ("box", match_value(0, DISPLAY_MATCHES)),
        ("block", match_value(0, DISPLAY_MATCHES)),
        ("inline", match_value(1, DISPLAY_MATCHES)),
        ("span", match_value(2, DISPLAY_MATCHES)),
        ("flex", match_value(3, DISPLAY_MATCHES)),
        ("table", match_value(4, DISPLAY_MATCHES)),
    ],
    apply_style: apply_display,
};

