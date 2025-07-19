use crate::styles::provider::AtomicStyleProvider;
use crate::styles::style::{StyleValue, StyleValueParser};
use crate::styles::style::StyleValueParser::{ColorParser, NumberParser};

pub struct BgColor;

impl AtomicStyleProvider for BgColor {
    fn name(&self) -> &'static str {
        "bgColor"
    }

    fn parser(&self) -> StyleValueParser { ColorParser }

    fn builtins(&self) -> &'static [(&'static str, StyleValue)] { &[] }

    fn apply(&self, style: &StyleValue) {
        todo!()
    }
}
