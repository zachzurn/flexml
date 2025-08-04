use crate::styles::context::{StyleContext, WhiteSpace};
use crate::styles::builtin::{apply_match_style, BuiltInStyle};
use crate::styles::style::{StyleValue};
use crate::styles::style::StyleValueParser::Match;

fn apply_white_space(value: &StyleValue, context: &mut StyleContext) {
    apply_match_style(value, &mut context.white_space, &[
        WhiteSpace::Normal,
        WhiteSpace::NoWrap,
        WhiteSpace::Pre,
        WhiteSpace::PreWrap,
        WhiteSpace::PreLine
    ])
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
