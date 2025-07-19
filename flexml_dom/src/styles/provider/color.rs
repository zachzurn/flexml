use crate::styles::provider::AtomicStyleProvider;
use crate::styles::style::{StyleValue, StyleValueParser};
use crate::styles::style::StyleValueParser::{ColorParser, NumberParser};

pub struct Color;

impl AtomicStyleProvider for Color {
    fn name(&self) -> &'static str {
        "color"
    }

    fn parser(&self) -> StyleValueParser { ColorParser }

    fn builtins(&self) -> &'static [(&'static str, StyleValue)] { &[] }

    fn apply(&self, style: &StyleValue) {
        todo!()
    }
}
