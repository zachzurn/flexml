use crate::styles::provider::{AtomicStyleProvider};
use crate::styles::style::{StyleValue, StyleValueParser};
use crate::styles::style::StyleValueParser::MatchParser;

pub struct BorderStyle;

impl AtomicStyleProvider for BorderStyle {
    fn name(&self) -> &'static str {
        "borderStyle"
    }

    fn parser(&self) -> StyleValueParser {
        MatchParser(&[
            "solid",
            "dashed",
            "dotted",
            "none"
        ])
    }

    fn builtins(&self) -> &'static [(&'static str, StyleValue)] {
        &[
            ("solidBorder", StyleValue::Match(0)),
            ("dashedBorder", StyleValue::Match(1)),
            ("dottedBorder", StyleValue::Match(2)),
            ("borderless", StyleValue::Match(2)),
        ]
    }

    fn apply(&self, style: &StyleValue) {
        todo!()
    }
}