use crate::styles::provider::AtomicStyleProvider;
use crate::styles::style::{StyleValue, StyleValueParser};
use crate::styles::style::StyleValueParser::{ColorParser, NumberParser, UrlParser};

pub struct BgImage;

impl AtomicStyleProvider for BgImage {
    fn name(&self) -> &'static str {
        "bgImage"
    }

    fn parser(&self) -> StyleValueParser { UrlParser }

    fn builtins(&self) -> &'static [(&'static str, StyleValue)] { &[] }

    fn apply(&self, style: &StyleValue) {
        todo!()
    }
}
