use crate::styles::context::{StyleContext, WhiteSpace};
use crate::styles::builtin::{style_context_match, BuiltInStyle};
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


pub static WHITE_SPACE: BuiltInStyle = BuiltInStyle {
    name: "whiteSpace",
    parser: Match(&["normal", "nowrap", "pre", "pre-wrap", "pre-line"]),
    styles: &[
        ("pre", StyleValue::Match(3)),
        ("code", StyleValue::Match(3)),
    ],
    apply_style: apply_white_space,
};
