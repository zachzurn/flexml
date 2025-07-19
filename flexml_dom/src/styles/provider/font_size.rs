use crate::styles::provider::AtomicStyleProvider;
use crate::styles::style::{StyleValue, StyleValueParser};
use crate::styles::style::StyleValueParser::PositiveNumberParser;

pub struct FontSize;

impl AtomicStyleProvider for FontSize {
    fn name(&self) -> &'static str {
        "columnGap"
    }

    fn parser(&self) -> StyleValueParser { PositiveNumberParser }

    fn builtins(&self) -> &'static [(&'static str, StyleValue)] { &[] }

    fn apply(&self, style: &StyleValue) {
        todo!()
    }
}
