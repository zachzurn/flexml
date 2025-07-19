use crate::styles::provider::AtomicStyleProvider;
use crate::styles::style::{StyleValue, StyleValueParser};
use crate::styles::style::StyleValueParser::PositiveNumberParser;

pub struct LineHeight;

impl AtomicStyleProvider for LineHeight {
    fn name(&self) -> &'static str {
        "lineHeight"
    }

    fn parser(&self) -> StyleValueParser { PositiveNumberParser }

    fn builtins(&self) -> &'static [(&'static str, StyleValue)] { &[] }

    fn apply(&self, style: &StyleValue) {
        todo!()
    }
}
