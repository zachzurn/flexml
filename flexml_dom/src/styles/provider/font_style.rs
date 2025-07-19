use crate::styles::provider::AtomicStyleProvider;
use crate::styles::style::{StyleValue, StyleValueParser};
use crate::styles::style::StyleValueParser::MatchParser;

pub struct FontStyle;

impl AtomicStyleProvider for FontStyle {
    fn name(&self) -> &'static str {
        "fontStyle"
    }

    fn parser(&self) -> StyleValueParser {
        MatchParser(&["normal", "italic", "oblique"])
    }

    fn builtins(&self) -> &'static [(&'static str, StyleValue)] {
        &[
            ("italic", StyleValue::Match(1)),
        ]
    }

    fn apply(&self, style: &StyleValue) {
        todo!()
    }
}