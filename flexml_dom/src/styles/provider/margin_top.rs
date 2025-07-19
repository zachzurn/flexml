use crate::styles::provider::AtomicStyleProvider;
use crate::styles::style::{StyleValue, StyleValueParser};
use crate::styles::style::StyleValueParser::NumberParser;

pub struct MarginTop;

impl AtomicStyleProvider for MarginTop {
    fn name(&self) -> &'static str {
        "marginTop"
    }

    fn parser(&self) -> StyleValueParser { NumberParser }

    fn builtins(&self) -> &'static [(&'static str, StyleValue)] { &[] }

    fn apply(&self, style: &StyleValue) {
        todo!()
    }
}
