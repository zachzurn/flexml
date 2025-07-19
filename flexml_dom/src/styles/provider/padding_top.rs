use crate::styles::provider::AtomicStyleProvider;
use crate::styles::style::{StyleValue, StyleValueParser};
use crate::styles::style::StyleValueParser::NumberParser;

pub struct PaddingTop;

impl AtomicStyleProvider for PaddingTop {
    fn name(&self) -> &'static str {
        "paddingTop"
    }

    fn parser(&self) -> StyleValueParser { NumberParser }

    fn builtins(&self) -> &'static [(&'static str, StyleValue)] { &[] }

    fn apply(&self, style: &StyleValue) {
        todo!()
    }
}
