use crate::styles::provider::AtomicStyleProvider;
use crate::styles::style::{StyleValue, StyleValueParser};
use crate::styles::style::StyleValueParser::PositiveNumberParser;

pub struct LetterSpacing;

impl AtomicStyleProvider for LetterSpacing {
    fn name(&self) -> &'static str {
        "letterSpacing"
    }

    fn parser(&self) -> StyleValueParser { PositiveNumberParser }

    fn builtins(&self) -> &'static [(&'static str, StyleValue)] { &[] }

    fn apply(&self, style: &StyleValue) {
        todo!()
    }
}
