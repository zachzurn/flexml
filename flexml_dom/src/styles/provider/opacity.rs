use crate::styles::provider::AtomicStyleProvider;
use crate::styles::style::{StyleValue, StyleValueParser};
use crate::styles::style::StyleValueParser::{FloatParser};

pub struct Opacity;

impl AtomicStyleProvider for Opacity {
    fn name(&self) -> &'static str {
        "opacity"
    }

    fn parser(&self) -> StyleValueParser {
        FloatParser
    }

    fn builtins(&self) -> &'static [(&'static str, StyleValue)] { &[] }

    fn apply(&self, style: &StyleValue) {
        todo!()
    }
}
