use crate::styles::provider::AtomicStyleProvider;
use crate::styles::style::{StyleValue, StyleValueParser};
use crate::styles::style::StyleValueParser::MatchParser;

pub struct BgRepeat;

impl AtomicStyleProvider for BgRepeat {
    fn name(&self) -> &'static str {
        "bgRepeat"
    }

    fn parser(&self) -> StyleValueParser {
        MatchParser(&["repeat", "repeat-x", "repeat-y", "no-repeat"])
    }

    fn builtins(&self) -> &'static [(&'static str, StyleValue)] {
        &[
            ("noRepeat", StyleValue::Match(3)),
            ("repeat", StyleValue::Match(0)),
            ("repeatX", StyleValue::Match(1)),
            ("repeatY", StyleValue::Match(2)),
        ]
    }

    fn apply(&self, style: &StyleValue) {
        todo!()
    }
}