use crate::styles::provider::AtomicStyleProvider;
use crate::styles::style::{StyleValue, StyleValueParser};
use crate::styles::style::StyleValueParser::FloatParser;

pub struct FlexShrink;

impl AtomicStyleProvider for FlexShrink {
    fn name(&self) -> &'static str {
        "flexShrink"
    }

    fn parser(&self) -> StyleValueParser {
        FloatParser
    }

    fn builtins(&self) -> &'static [(&'static str, StyleValue)] {
        &[
            ("shrink", StyleValue::Float(1.0)),
        ]
    }

    fn apply(&self, style: &StyleValue) {
        todo!()
    }
}
