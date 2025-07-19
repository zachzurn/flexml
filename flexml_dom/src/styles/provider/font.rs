use crate::styles::provider::AtomicStyleProvider;
use crate::styles::style::{StyleValue, StyleValueParser};
use crate::styles::style::StyleValueParser::{ColorParser, FontParser, NumberParser};

pub struct Font;

impl AtomicStyleProvider for Font {
    fn name(&self) -> &'static str {
        "font"
    }

    fn parser(&self) -> StyleValueParser { FontParser }

    fn builtins(&self) -> &'static [(&'static str, StyleValue)] { &[] }

    fn apply(&self, style: &StyleValue) {
        todo!()
    }
}
