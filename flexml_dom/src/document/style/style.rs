use crate::document::style::style_registry::StyleId;

#[derive(PartialEq, Clone, Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

#[derive(PartialEq, Clone, Debug)]
pub struct FontRef {

}

pub enum StyleValueParser {
    Float,
    Bool,
    Tiny,
    Short,
    Long,
    Match(&'static [&'static str]),
    Color,
    Font,
}

#[derive(PartialEq, Clone, Debug)]
pub enum StyleValue {
    Float(f64),
    Bool(bool),
    Tiny(u8),
    Short(u16),
    Long(u32),
    Match(u8),
    Color(Color),
    Font(FontRef),
    None,
    Error
}

impl StyleValueParser {

    pub fn parse(&self, s: &str) -> StyleValue {
        match self {
            StyleValueParser::Float => self.parse_float(s),
            StyleValueParser::Bool => self.parse_bool(s),
            StyleValueParser::Tiny => self.parse_tiny(s),
            StyleValueParser::Short => self.parse_short(s),
            StyleValueParser::Long => self.parse_long(s),
            StyleValueParser::Match(_) => self.parse_match(s),
            StyleValueParser::Color => self.parse_color(s),
            StyleValueParser::Font => self.parse_font(s),
        }
    }

    fn parse_float(&self, s: &str) -> StyleValue {
        StyleValue::Float(0.0)
    }

    fn parse_bool(&self, s: &str) -> StyleValue {
        StyleValue::Bool(false)
    }

    fn parse_tiny(&self, s: &str) -> StyleValue {
        StyleValue::Tiny(1u8)
    }

    fn parse_short(&self, s: &str) -> StyleValue {
        StyleValue::Short(1u16)
    }

    fn parse_long(&self, s: &str) -> StyleValue {
        StyleValue::Long(1u32)
    }

    fn parse_match(&self, s: &str) -> StyleValue {
        StyleValue::Match(0)
    }

    fn parse_color(&self, s: &str) -> StyleValue {
        StyleValue::Color(Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        })
    }

    fn parse_font(&self, s: &str) -> StyleValue {
        StyleValue::Font(FontRef {})
    }

}