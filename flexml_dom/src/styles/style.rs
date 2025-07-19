use lazy_static::lazy_static;
use url::Url;

#[derive(PartialEq, Clone, Debug)]
pub struct RGBA {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
    pub(crate) a: u8
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
    MatchOrFloatParser(&'static [&'static str]),
    FloatParser,
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
    Float(f32),
    Url(Url),
    Match(u8),
    Color(RGBA),
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
static COLOR_SUGGESTIONS: &[&str] = &["#FFFFFF","#FF", "#FF0000FF"];
static FLOAT_SUGGESTIONS: &[&str] = &["0","1.0","-1.0"];

static INVALID_PERCENT: &str = "Invalid percent value";
static INVALID_NUMBER: &str = "Invalid number";
static INVALID_MATCH: &str = "Invalid value";
static FATAL_MATCH: &str = "Fatal error when matching";
static INVALID_NEGATIVE_PERCENT: &str = "Invalid value, percent numbers can't be negative";
static INVALID_NEGATIVE_NUMBER: &str = "Invalid value, number can't be negative";
static INVALID_URL: &str = "Invalid URL";
static INVALID_COLOR: &str = "Invalid color";
static INVALID_FLOAT: &str = "Invalid decimal number";

static PERCENT_POSTFIX: &str = "%";
static HEX_PREFIX: &str = "#";

impl StyleValueParser {

    pub fn parse(&self, s: &str) -> StyleValue {
        match self {
            &StyleValueParser::MatchParser(matches) => self.parse_match(matches, s),
            &StyleValueParser::MatchOrFloatParser(matches) => self.parse_match_or_float(matches, s),
            &StyleValueParser::ColorParser => self.parse_color(s),
            &StyleValueParser::FontParser => self.parse_font(s),
            &StyleValueParser::NumberParser => self.parse_number(s),
            &StyleValueParser::PositiveNumberParser => self.parse_number(s),
            &StyleValueParser::UrlParser => self.parse_url(s),
            &StyleValueParser::FloatParser => self.parse_float(s),
        }
    }

    fn parse_match_or_float(&self,  matches: &'static [&'static str], s: &str) -> StyleValue {
        let value = self.parse_match(matches, s);

        match value {
            StyleValue::Match(_) => value,
            _ => self.parse_float(s)
        }
    }

    fn parse_float(&self, s: &str) -> StyleValue {

        let trimmed = s.trim();

        if let Ok(float) = trimmed.parse::<f32>() {
            StyleValue::Float(float)
        } else {
            StyleValue::Invalid(INVALID_FLOAT, FLOAT_SUGGESTIONS)
        }

    }

    fn parse_percent(&self, s: &str) -> StyleValue {
        return if let Ok(float) = s.parse::<f32>() {
            if float < 0.0 {
                return StyleValue::Invalid(INVALID_NEGATIVE_PERCENT, PERCENT_SUGGESTIONS);
            }
            StyleValue::Percent(PercentFloat(float))
        } else {
            StyleValue::Invalid(INVALID_PERCENT, PERCENT_SUGGESTIONS)
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

    fn parse_match(&self, matches: &'static [&'static str], s: &str) -> StyleValue {
        let lc = s.to_ascii_lowercase();

        // Match list is too long
        if matches.len() > 255 { return StyleValue::Invalid(FATAL_MATCH, FATAL_MATCH_SUGGESTIONS); }

        for (i, v) in matches.iter().enumerate() {
            if v.eq_ignore_ascii_case(&lc) {
                return StyleValue::Match(i as u8);
            }
        }

        // No match found
        return StyleValue::Invalid(INVALID_MATCH, matches);
    }

    fn parse_color(&self, s: &str) -> StyleValue {
        let trimmed = s.trim();

        if trimmed.is_empty() { return StyleValue::Empty }

        let has_hex_prefix = trimmed.starts_with(HEX_PREFIX);

        if !has_hex_prefix {
            return StyleValue::Invalid(INVALID_COLOR, COLOR_SUGGESTIONS);
        }

        if let Some((r,g,b,a)) = self.parse_hex_string(&trimmed[1..]){
            StyleValue::Color(RGBA { r, g, b, a })
        } else {
            StyleValue::Invalid(INVALID_COLOR, COLOR_SUGGESTIONS)
        }
    }

    fn parse_hex_char(&self, c: char) -> (u8, bool) {
        if let Some(digit) = c.to_digit(16) {
            (digit as u8, true)
        } else {
            (0, false)
        }
    }

    fn parse_hex_string(&self, s: &str) -> Option<(u8, u8, u8, u8)> {
        match s.len() {
            2 => { // #HH -> 0xHHHHHHFF
                let chars: Vec<char> = s.chars().collect();
                let (h1, h1_valid) = self.parse_hex_char(chars[0]);
                let (h2, h2_valid) = self.parse_hex_char(chars[1]);

                if !h1_valid || !h2_valid {
                    return None
                }

                let value = (h1 << 4) | h2;

                Some((value, value, value, 0xFF))
            },
            3 => { // #RGB -> 0xRRGGBBFF
                let chars: Vec<char> = s.chars().collect();
                let (r, r_valid) = self.parse_hex_char(chars[0]);
                let (g, g_valid) = self.parse_hex_char(chars[1]);
                let (b, b_valid) = self.parse_hex_char(chars[2]);

                if !r_valid || !g_valid || !b_valid {
                    return None
                }

                Some((r * 0x11, g * 0x11, b * 0x11, 0xFF))
            },
            4 => { // #RGBA -> 0xRRGGBBAA
                let chars: Vec<char> = s.chars().collect();
                let (r, r_valid) = self.parse_hex_char(chars[0]);
                let (g, g_valid) = self.parse_hex_char(chars[1]);
                let (b, b_valid) = self.parse_hex_char(chars[2]);
                let (a, a_valid) = self.parse_hex_char(chars[3]);

                if !a_valid || !r_valid || !g_valid || !b_valid {
                    return None
                }

                Some((r * 0x11, g * 0x11, b * 0x11, a * 0x11))
            },
            6 => { // #RRGGBB -> 0xRRGGBBFF
                if let Ok(rgb) = u32::from_str_radix(s, 16) {
                    Some((
                        ((rgb >> 16) & 0xFF) as u8,
                        ((rgb >> 8) & 0xFF) as u8,
                        (rgb & 0xFF) as u8,
                        0xFF
                    ))
                } else {
                    None
                }
            },
            8 => { // #RRGGBBAA
                if let Ok(rgba) = u32::from_str_radix(s, 16) {
                    Some((
                        ((rgba >> 24) & 0xFF) as u8,
                        ((rgba >> 16) & 0xFF) as u8,
                        ((rgba >> 8) & 0xFF) as u8,
                        (rgba & 0xFF) as u8,
                    ))
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    //TODO
    fn parse_font(&self, _: &str) -> StyleValue {
        StyleValue::Font(FontRef {})
    }

}