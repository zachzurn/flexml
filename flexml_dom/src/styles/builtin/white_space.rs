use crate::styles::context::{StyleContext, WhiteSpace};
use crate::styles::builtin::{match_value, style_context_match, BuiltInStyle};
use crate::styles::style::{StyleValue};
use crate::styles::style::StyleValueParser::Match;

const WHITE_SPACE_VARIANTS: &[WhiteSpace] = &[
    WhiteSpace::Normal,
    WhiteSpace::NoWrap,
    WhiteSpace::Pre,
    WhiteSpace::PreWrap,
    WhiteSpace::PreLine,
];

fn apply_white_space(value: &StyleValue, context: &mut StyleContext) {
    if let Some(v) = style_context_match(value, WHITE_SPACE_VARIANTS) {
        context.set_white_space(v);
    }
}


pub static WHITE_SPACE_MATCHES: &[&str] = &[
    "normal",
    "nowrap",
    "pre",
    "pre-wrap",
    "pre-line",
];

pub static WHITE_SPACE: BuiltInStyle = BuiltInStyle {
    name: "whiteSpace",
    parser: Match(WHITE_SPACE_MATCHES),
    styles: &[
        ("pre", match_value(2, WHITE_SPACE_MATCHES)),
        ("code", match_value(2, WHITE_SPACE_MATCHES)),
    ],
    apply_style: apply_white_space,
};

