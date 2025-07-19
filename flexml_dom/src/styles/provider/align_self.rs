use crate::styles::provider::AtomicStyleProvider;
use crate::styles::style::{StyleValue, StyleValueParser};
use crate::styles::style::StyleValueParser::MatchParser;

pub struct AlignSelf;

impl AtomicStyleProvider for AlignSelf {
    fn name(&self) -> &'static str {
        "alignSelf"
    }

    fn parser(&self) -> StyleValueParser {
        MatchParser(&["auto", "flex-start", "flex-end", "center", "baseline", "stretch"])
    }

    fn builtins(&self) -> &'static [(&'static str, StyleValue)] {
        &[
            ("selfStart", StyleValue::Match(1)),
            ("selfStretch", StyleValue::Match(5)),
        ]
    }

    fn apply(&self, style: &StyleValue) {
        todo!()
    }
}