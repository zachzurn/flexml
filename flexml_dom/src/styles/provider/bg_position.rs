use crate::styles::provider::AtomicStyleProvider;
use crate::styles::style::{StyleValue, StyleValueParser};
use crate::styles::style::StyleValueParser::MatchParser;

pub struct BgPosition;

impl AtomicStyleProvider for BgPosition {
    fn name(&self) -> &'static str {
        "bgPosition"
    }

    fn parser(&self) -> StyleValueParser {
        MatchParser(&["center", "top", "bottom", "left", "right"])
    }

    fn builtins(&self) -> &'static [(&'static str, StyleValue)] {
        &[
            ("bgCenter", StyleValue::Match(0)),
        ]
    }

    fn apply(&self, style: &StyleValue) {
        todo!()
    }
}