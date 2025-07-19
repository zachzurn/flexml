use crate::styles::provider::AtomicStyleProvider;
use crate::styles::style::{StyleValue, StyleValueParser};
use crate::styles::style::StyleValueParser::{FloatParser};

pub struct FlexGrow;

impl AtomicStyleProvider for FlexGrow {
    fn name(&self) -> &'static str {
        "flexGrow"
    }

    fn parser(&self) -> StyleValueParser {
        FloatParser
    }

    fn builtins(&self) -> &'static [(&'static str, StyleValue)] {
        &[
            ("grow", StyleValue::Float(1.0)),
        ]
    }

    fn apply(&self, style: &StyleValue) {
        todo!()
    }
}
