use crate::styles::provider::AtomicStyleProvider;
use crate::styles::style::{StyleValue, StyleValueParser};
use crate::styles::style::StyleValueParser::MatchParser;

pub struct FlexDirection;

impl AtomicStyleProvider for FlexDirection {
    fn name(&self) -> &'static str {
        "flexDirection"
    }

    fn parser(&self) -> StyleValueParser {
        MatchParser(&["row", "row-reverse", "column", "column-reverse"])
    }

    fn builtins(&self) -> &'static [(&'static str, StyleValue)] {
        &[
            ("row", StyleValue::Match(0)),
            ("col", StyleValue::Match(2)),
            ("column", StyleValue::Match(2)),
        ]
    }

    fn apply(&self, style: &StyleValue) {
        todo!()
    }
}