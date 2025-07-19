use crate::styles::provider::AtomicStyleProvider;
use crate::styles::style::{StyleValue, StyleValueParser};
use crate::styles::style::StyleValueParser::MatchParser;

pub struct TextTransform;

impl AtomicStyleProvider for TextTransform {
    fn name(&self) -> &'static str {
        "textTransform"
    }

    fn parser(&self) -> StyleValueParser {
        MatchParser(&["none", "capitalize", "uppercase", "lowercase"])
    }

    fn builtins(&self) -> &'static [(&'static str, StyleValue)] {
        &[
            ("uppercase", StyleValue::Match(2)),
            ("lowercase", StyleValue::Match(3)),
            ("capitalize", StyleValue::Match(4)),
        ]
    }

    fn apply(&self, style: &StyleValue) {
        todo!()
    }
}