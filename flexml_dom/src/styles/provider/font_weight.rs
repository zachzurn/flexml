use crate::styles::provider::{AtomicStyleProvider};
use crate::styles::style::{StyleValue, StyleValueParser};
use crate::styles::style::StyleValueParser::MatchParser;

pub struct FontWeight;

impl AtomicStyleProvider for FontWeight {
    fn name(&self) -> &'static str {
        "fontWeight"
    }

    fn parser(&self) -> StyleValueParser {
        MatchParser(&[
            "normal",
            "bold",
            "light",
            "bolder",
            "lighter",
            "100",
            "200",
            "300",
            "400",
            "500",
            "600",
            "700",
            "800",
            "900"
        ])
    }

    fn builtins(&self) -> &'static [(&'static str, StyleValue)] {
        &[
            ("normal", StyleValue::Match(0)),
            ("bold", StyleValue::Match(1)),
            ("light", StyleValue::Match(2)),
        ]
    }

    fn apply(&self, style: &StyleValue) {
        todo!()
    }
}