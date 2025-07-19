use crate::styles::provider::AtomicStyleProvider;
use crate::styles::style::{StyleValue, StyleValueParser};
use crate::styles::style::StyleValueParser::MatchParser;

pub struct FlexWrap;

impl AtomicStyleProvider for FlexWrap {
    fn name(&self) -> &'static str {
        "flexWrap"
    }

    fn parser(&self) -> StyleValueParser {
        MatchParser(&["nowrap", "wrap", "wrap-reverse"])
    }

    fn builtins(&self) -> &'static [(&'static str, StyleValue)] {
        &[
            ("noWrap", StyleValue::Match(0)),
            ("wrapReverse", StyleValue::Match(2)),
        ]
    }

    fn apply(&self, style: &StyleValue) {
        todo!()
    }
}