use crate::styles::provider::AtomicStyleProvider;
use crate::styles::style::{StyleValue, StyleValueParser};
use crate::styles::style::StyleValueParser::MatchParser;

pub struct TextAlign;

impl AtomicStyleProvider for TextAlign {
    fn name(&self) -> &'static str {
        "textAlign"
    }

    fn parser(&self) -> StyleValueParser {
        MatchParser(&["left", "right", "center", "justify"])
    }

    fn builtins(&self) -> &'static [(&'static str, StyleValue)] {
        &[
            ("left", StyleValue::Match(0)),
            ("center", StyleValue::Match(2)),
            ("right", StyleValue::Match(1)),
            ("justify", StyleValue::Match(3)),
        ]
    }

    fn apply(&self, style: &StyleValue) {
        todo!()
    }
}