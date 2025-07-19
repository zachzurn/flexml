use crate::styles::provider::AtomicStyleProvider;
use crate::styles::style::{StyleValue, StyleValueParser};
use crate::styles::style::StyleValueParser::MatchParser;

pub struct BgSize;

impl AtomicStyleProvider for BgSize {
    fn name(&self) -> &'static str {
        "bgSize"
    }

    fn parser(&self) -> StyleValueParser {
        MatchParser(&["auto", "cover", "contain"])
    }

    fn builtins(&self) -> &'static [(&'static str, StyleValue)] {
        &[
            ("cover", StyleValue::Match(1)),
            ("contain", StyleValue::Match(2)),
        ]
    }

    fn apply(&self, style: &StyleValue) {
        todo!()
    }
}