use crate::styles::provider::AtomicStyleProvider;
use crate::styles::style::{StyleValue, StyleValueParser};
use crate::styles::style::StyleValueParser::MatchParser;

pub struct WhiteSpace;

impl AtomicStyleProvider for WhiteSpace {
    fn name(&self) -> &'static str {
        "whiteSpace"
    }

    fn parser(&self) -> StyleValueParser {
        MatchParser(&["normal", "nowrap", "pre", "pre-wrap", "pre-line"])
    }

    fn builtins(&self) -> &'static [(&'static str, StyleValue)] {
        &[
            ("pre", StyleValue::Match(3)),
            ("code", StyleValue::Match(3)),
        ]
    }

    fn apply(&self, style: &StyleValue) {
        todo!()
    }
}