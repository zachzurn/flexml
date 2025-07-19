use crate::styles::provider::AtomicStyleProvider;
use crate::styles::style::{StyleValue, StyleValueParser};
use crate::styles::style::StyleValueParser::MatchParser;

pub struct AlignContent;

impl AtomicStyleProvider for AlignContent {
    fn name(&self) -> &'static str {
        "alignContent"
    }

    fn parser(&self) -> StyleValueParser {
        MatchParser(&[
            "flex-start", "flex-end", "center", "space-between", "space-around", "stretch"
        ])
    }

    fn builtins(&self) -> &'static [(&'static str, StyleValue)] {
        &[
            ("contentStart", StyleValue::Match(0)),
            ("contentStretch", StyleValue::Match(5)),
        ]
    }

    fn apply(&self, style: &StyleValue) {
        todo!()
    }
}