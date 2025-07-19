use crate::styles::provider::AtomicStyleProvider;
use crate::styles::style::{StyleValue, StyleValueParser};
use crate::styles::style::StyleValueParser::MatchOrFloatParser;

pub struct FlexBasis;

impl AtomicStyleProvider for FlexBasis {
    fn name(&self) -> &'static str {
        "flexBasis"
    }

    fn parser(&self) -> StyleValueParser {
        MatchOrFloatParser(&["auto", "content"])
    }

    fn builtins(&self) -> &'static [(&'static str, StyleValue)] {
        &[
            ("basisAuto", StyleValue::Match(0)),
            ("basisContent", StyleValue::Match(1)),
        ]
    }

    fn apply(&self, style: &StyleValue) {
        todo!()
    }
}
