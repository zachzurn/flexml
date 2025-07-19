use crate::styles::provider::{AtomicStyleProvider};
use crate::styles::style::{StyleValue, StyleValueParser};
use crate::styles::style::StyleValueParser::MatchParser;

pub struct Display;

impl AtomicStyleProvider for Display {
    fn name(&self) -> &'static str {
        "display"
    }

    fn parser(&self) -> StyleValueParser {
        MatchParser(&[
            "block",
            "inline",
            "inline-block",
            "flex",
            "table"
        ])
    }

    fn builtins(&self) -> &'static [(&'static str, StyleValue)] {
        &[
            ("box", StyleValue::Match(0)),
            ("block", StyleValue::Match(0)),
            ("inline", StyleValue::Match(1)),
            ("span", StyleValue::Match(2)),
            ("flex", StyleValue::Match(3)),
            ("table", StyleValue::Match(3)),
        ]
    }

    fn apply(&self, style: &StyleValue) {
        todo!()
    }
}