use crate::styles::provider::AtomicStyleProvider;
use crate::styles::style::{StyleValue, StyleValueParser};
use crate::styles::style::StyleValueParser::MatchParser;

pub struct TextDecoration;

impl AtomicStyleProvider for TextDecoration {
    fn name(&self) -> &'static str {
        "textDecoration"
    }

    fn parser(&self) -> StyleValueParser {
        MatchParser(&["none", "underline", "overline", "line-through"])
    }

    fn builtins(&self) -> &'static [(&'static str, StyleValue)] {
        &[
            ("underline", StyleValue::Match(1)),
            ("strike", StyleValue::Match(3)),
            ("overline", StyleValue::Match(2)),
        ]
    }

    fn apply(&self, style: &StyleValue) {
        todo!()
    }
}