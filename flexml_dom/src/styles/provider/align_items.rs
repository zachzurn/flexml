use crate::styles::provider::AtomicStyleProvider;
use crate::styles::style::{StyleValue, StyleValueParser};
use crate::styles::style::StyleValueParser::MatchParser;

pub struct AlignItems;

impl AtomicStyleProvider for AlignItems {
    fn name(&self) -> &'static str {
        "alignItems"
    }

    fn parser(&self) -> StyleValueParser {
        MatchParser(&["flex-start", "flex-end", "center", "baseline", "stretch"])
    }

    fn builtins(&self) -> &'static [(&'static str, StyleValue)] {
        &[
            ("itemsStart", StyleValue::Match(0)),
            ("itemsCenter", StyleValue::Match(2)),
        ]
    }

    fn apply(&self, style: &StyleValue) {
        todo!()
    }
}