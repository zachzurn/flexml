use lazy_static::lazy_static;
use url::Url;
use crate::parsing::style::StyleValue::{Invalid, Percent};

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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PercentFloat(f32);

impl PercentFloat {
    pub fn new(value: f32) -> Self {
        if value >= 0.0 {
            PercentFloat(value)
        } else {
            PercentFloat(0.0)
        }
    }

    pub fn get(self) -> f32 {
        self.0
    }
}

pub enum StyleValueParser {
    NumberParser,
    PositiveNumberParser,
    MatchParser(&'static [&'static str]),
    ColorParser,
    FontParser,
    UrlParser,
}

#[derive(PartialEq, Clone, Debug)]
pub enum StyleValue {
    Number(u16),
    NegativeNumber(u16),
    Percent(PercentFloat),

    Url(Url),
    Match(u8),
    Color(Color),
    Font(FontRef),
    Invalid(&'static str, &'static [&'static str]),
    Empty,
}

pub type StyleId = usize;

#[derive(Debug, PartialEq, Clone)]
pub struct AtomicStyle {
    pub(crate) id: StyleId,
    pub(crate) value: StyleValue,
}

pub struct RawStyle<'a> {
    pub(crate) name: &'a str,
    pub(crate) value: Option<&'a str>,
}

impl RawStyle<'_> {
    pub fn new<'a>(name: &'a str, value: Option<&'a str>) -> RawStyle<'a> {
        RawStyle {
            name,
            value,
        }
    }
}

lazy_static! {
    // We use a base url for parsing relative urls
    pub static ref BASE_URL: Url = Url::parse("flexml://")
        .expect("Failed to parse BASE_URL URL at compile time. This should be a valid URL.");
}

//Static
static NUMBER_SUGGESTIONS: &[&str] = &["1","100","-100","100%"];
static POSITIVE_NUMBER_SUGGESTIONS: &[&str] = &["1","100","100%"];
static PERCENT_SUGGESTIONS: &[&str] = &["1.5%","100%","200%"];
static URL_SUGGESTIONS: &[&str] = &["http://www.google.com/image.png", "image.jpg","../image.png"];
static FATAL_MATCH_SUGGESTIONS: &[&str] = &["This atomic style is broken"];

static INVALID_PERCENT: &str = "Invalid percent value";
static INVALID_NUMBER: &str = "Invalid number";
static INVALID_MATCH: &str = "Invalid value";
static FATAL_MATCH: &str = "Fatal error when matching";
static INVALID_NEGATIVE_PERCENT: &str = "Invalid value, percent numbers can't be negative";
static INVALID_NEGATIVE_NUMBER: &str = "Invalid value, number can't be negative";
static INVALID_URL: &str = "Invalid URL";

static PERCENT_POSTFIX: &str = "%";

impl StyleValueParser {

    pub fn parse(&self, s: &str) -> StyleValue {
        match self {
            &StyleValueParser::MatchParser(_) => self.parse_match(s),
            &StyleValueParser::ColorParser => self.parse_color(s),
            &StyleValueParser::FontParser => self.parse_font(s),
            &StyleValueParser::NumberParser => self.parse_number(s),
            &StyleValueParser::PositiveNumberParser => self.parse_number(s),
            &StyleValueParser::UrlParser => self.parse_url(s),
        }
    }

    fn parse_percent(&self, s: &str) -> StyleValue {
        return if let Ok(float) = s.parse::<f32>() {
            if float < 0.0 {
                return Invalid(INVALID_NEGATIVE_PERCENT, PERCENT_SUGGESTIONS);
            }
            Percent(PercentFloat(float))
        } else {
            Invalid(INVALID_PERCENT, PERCENT_SUGGESTIONS)
        }
    }

    // Parses a whole number
    fn parse_number(&self, s: &str) -> StyleValue {

        let trimmed = s.trim();
        let negative = trimmed.starts_with('-');
        let number = trimmed.trim_start_matches('-');

        if number.ends_with(PERCENT_POSTFIX) {
            return self.parse_percent(number.trim_end_matches(PERCENT_POSTFIX));
        }

        if number.is_empty() { return StyleValue::Empty }

        let mut digits = 0;

        for char in number.chars() {
            if char.is_ascii_digit() { digits += 1 }
            else { break }
        }

        if digits == 0 {
            return StyleValue::Invalid(INVALID_NUMBER, &NUMBER_SUGGESTIONS);
        }
        else {
            return if let Ok(number) = &number[0..digits].parse::<u16>() {
                if negative { StyleValue::NegativeNumber(*number) } else { StyleValue::Number(*number) }
            } else {
                StyleValue::Invalid(INVALID_NUMBER, NUMBER_SUGGESTIONS)
            }
        }


    }

    fn parse_positive_number(&self, s: &str) -> StyleValue {
        let value = self.parse_number(s);

        match value {
            StyleValue::Number(_) => value,
            StyleValue::Percent(_) => value,
            StyleValue::NegativeNumber(_) => StyleValue::Invalid(INVALID_NEGATIVE_NUMBER, POSITIVE_NUMBER_SUGGESTIONS),
            StyleValue::Empty => StyleValue::Empty,
            _ => StyleValue::Invalid(INVALID_NUMBER, POSITIVE_NUMBER_SUGGESTIONS),
        }
    }

    fn parse_url(&self, s: &str) -> StyleValue {
        if let Ok(url) = BASE_URL.join(s) {
            StyleValue::Url(url)
        } else {
            StyleValue::Invalid(INVALID_URL, URL_SUGGESTIONS)
        }
    }

    fn parse_match(&self, s: &str) -> StyleValue {
        let lc = s.to_ascii_lowercase();
        if let StyleValueParser::MatchParser(values) = self {

            // Match list is too long
            if values.len() > 254 { return StyleValue::Invalid(FATAL_MATCH, FATAL_MATCH_SUGGESTIONS); }

            for (i, v) in values.iter().enumerate() {
                if v.eq_ignore_ascii_case(&lc) {
                    return StyleValue::Match(i as u8);
                }
            }

            // No match found
            return StyleValue::Invalid(INVALID_MATCH, values);
        }
        StyleValue::Invalid(FATAL_MATCH, FATAL_MATCH_SUGGESTIONS)
    }

    fn parse_color(&self, s: &str) -> StyleValue {
        StyleValue::Color(Color{
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        })
    }

    //TODO
    fn parse_font(&self, s: &str) -> StyleValue {
        StyleValue::Font(FontRef {})
    }

}