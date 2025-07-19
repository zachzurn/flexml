use crate::styles::provider::AtomicStyleProvider;
use crate::styles::style::{StyleValue, StyleValueParser};
use crate::styles::style::StyleValueParser::MatchParser;

pub struct JustifyContent;

impl AtomicStyleProvider for JustifyContent {
    fn name(&self) -> &'static str {
        "justifyContent"
    }

    fn parser(&self) -> StyleValueParser {
        MatchParser(&[
            "flex-start", "flex-end", "center", "space-between", "space-around", "space-evenly"
        ])
    }

    fn builtins(&self) -> &'static [(&'static str, StyleValue)] {
        &[
            ("contentStart", StyleValue::Match(0)),
            ("contentEnd", StyleValue::Match(1)),
            ("contentCenter", StyleValue::Match(2)),
            ("contentSpaceBetween", StyleValue::Match(3)),
            ("contentSpaceAround", StyleValue::Match(4)),
            ("contentSpaceEvenly", StyleValue::Match(5)),
        ]
    }

    fn apply(&self, style: &StyleValue) {
        todo!()
    }
}