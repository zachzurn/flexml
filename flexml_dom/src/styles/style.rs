use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use crate::strings::{Chars, ValueErrors, ValueHelp};
use crate::styles::context::Dimension;

#[derive(PartialEq, Clone, Debug)]
pub struct Rgba {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
    pub(crate) a: u8
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PathType {
    Directory,
    Font,
    Image
}

pub enum StyleValueParser {
    MatchOrFloat(&'static [&'static str]),
    Float,
    Number,
    PositiveNumber,
    Match(&'static [&'static str]),
    Color,
    Path(PathType),
}

#[derive(PartialEq, Clone, Debug)]
pub enum StyleValue {
    /// Forward is a special style value
    /// only used in built ins as a proxy for Empty
    Forward,
    NegativeNumber(Dimension),
    PositiveNumber(Dimension),
    Float(f32),
    FontPath(PathBuf),
    ImagePath(PathBuf),
    DirectoryPath(PathBuf),
    Match(u8, &'static str),
    Color(Rgba),
    Font(PathId),
    Image(PathId),
    Directory(PathId),
    Unset,
    Invalid(&'static str, &'static [&'static str]),
    Empty,
}

pub type PathId = usize;
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
    #[allow(dead_code)]
    pub fn new<'a>(name: &'a str, value: Option<&'a str>) -> RawStyle<'a> {
        RawStyle {
            name,
            value,
        }
    }
}

pub enum DimensionKind {
    Px,
    Millimeters,
    Inches,
    Em,
    Rem,
    Percent,
    Point,
}

static DIMENSION_STR: &[&str; 7] = &[
    Chars::PX,
    Chars::PERCENT,
    Chars::PT,
    Chars::IN,
    Chars::MM,
    Chars::REM,
    Chars::EM
];

static DIMENSION_KIND: &[DimensionKind; 7] = &[
    DimensionKind::Px,
    DimensionKind::Percent,
    DimensionKind::Point,
    DimensionKind::Inches,
    DimensionKind::Millimeters,
    DimensionKind::Rem,
    DimensionKind::Em,
];

impl StyleValueParser {

    pub fn parse(&self, s: &str) -> StyleValue {
        match self {
            StyleValueParser::Match(matches) => Self::parse_match(matches, s),
            StyleValueParser::MatchOrFloat(matches) => Self::parse_match_or_float(matches, s),
            StyleValueParser::Color => Self::parse_color(s),
            StyleValueParser::Number => Self::parse_number(s),
            StyleValueParser::PositiveNumber => Self::parse_positive_number(s),
            StyleValueParser::Path(kind) => Self::parse_path(kind, s),
            StyleValueParser::Float => Self::parse_float(s),
        }
    }

    fn parse_match_or_float(matches: &'static [&'static str], s: &str) -> StyleValue {
        let value = StyleValueParser::parse_match(matches, s);

        match value {
            StyleValue::Match(_,_) => value,
            _ => Self::parse_float(s)
        }
    }

    fn parse_float(s: &str) -> StyleValue {
        if let Ok(float) = s.parse::<f32>() {
            StyleValue::Float(float)
        } else {
            StyleValue::Invalid(ValueErrors::FLOAT, ValueHelp::FLOAT)
        }

    }

    // Parses a number, can be positive or negative
    fn parse_number(s: &str) -> StyleValue {
        // precheck empty so we know empty value later on is invalid input
        if s.is_empty() { return StyleValue::Empty }

        let value = Self::parse_dimension_number(s);

        match value {
            // Any number or invalid get passed through as is
            StyleValue::PositiveNumber(_) |
            StyleValue::NegativeNumber(_) |
            StyleValue::Invalid(..) => value,

            // Empty or any other value in this case indicates a bad format, so we convert to invalid
            _ => StyleValue::Invalid(ValueErrors::NUMBER, ValueHelp::NUMBER),
        }
    }

    fn parse_positive_number(s: &str) -> StyleValue {
        // precheck empty so we know empty value later on is invalid input
        if s.is_empty() { return StyleValue::Empty }

        let value = Self::parse_dimension_number(s);

        match value {
            // Positive number and invalid get passed through as is
            StyleValue::PositiveNumber(_) |
            StyleValue::Invalid(..) => value,

            // Negative numbers are invalid
            StyleValue::NegativeNumber(_) => StyleValue::Invalid(ValueErrors::NEGATIVE_NUMBER, ValueHelp::POSITIVE_NUMBER),

            // Empty or any other value in this case indicates a bad format, so we convert to invalid
            _ => StyleValue::Invalid(ValueErrors::NUMBER, ValueHelp::POSITIVE_NUMBER),
        }
    }

    fn parse_dimension_number(input: &str) -> StyleValue {
        let input = input.to_ascii_lowercase();

        for (i, unit) in DIMENSION_STR.iter().enumerate() {
            if input.ends_with(unit) {
                let number_part = &input[..input.len() - unit.len()];

                return if let Ok(value) = number_part.parse::<f32>() {
                    let dim = match DIMENSION_KIND[i] {
                        DimensionKind::Rem => Dimension::Rem(value),
                        DimensionKind::Millimeters => Dimension::Mm(value),
                        DimensionKind::Inches => Dimension::Inch(value),
                        DimensionKind::Point => Dimension::Point(value),
                        DimensionKind::Em => Dimension::Em(value),
                        DimensionKind::Px => Dimension::Px(value),
                        DimensionKind::Percent => {
                            if value < 0.0 {
                                // Percentages cannot be negative
                                return StyleValue::Invalid(ValueErrors::NEGATIVE_PERCENT, ValueHelp::PERCENT);
                            } else {
                                Dimension::Percent(value)
                            }
                        },
                    };

                    return if value < 0.0 {
                        StyleValue::NegativeNumber(dim)
                    } else {
                        StyleValue::PositiveNumber(dim)
                    }
                } else {
                    StyleValue::Empty // Indicates invalid
                }
            }
        }

        if let Ok(value) = input.parse::<f32>() {
            return if value < 0.0 {
                StyleValue::NegativeNumber(Dimension::Px(value))
            } else {
                StyleValue::PositiveNumber(Dimension::Px(value))
            }
        }

        StyleValue::Empty
    }

    fn parse_path(kind: &PathType, s: &str) -> StyleValue {
        if s == "none" { return StyleValue::Unset }

        let path = Path::new(s);
        let is_dir = path.is_dir();
        let ext = path.extension().unwrap_or("".as_ref()).to_ascii_lowercase();

        match kind {
            PathType::Image => {
                if is_dir {
                    StyleValue::Invalid(ValueErrors::FILE, ValueHelp::FILE)
                }
                else if ext == "png" || ext == "jpg" || ext == "jpeg" || ext == "svg" {
                    StyleValue::ImagePath(path.to_path_buf())
                } else {
                    StyleValue::Invalid(ValueErrors::IMAGE, ValueHelp::IMAGE)
                }
            },
            PathType::Font => {
                if is_dir { StyleValue::Invalid(ValueErrors::FILE, ValueHelp::FILE) }
                else if ext == "ttf" || ext == "otf" || ext == "woff" || ext == "woff2" {
                    StyleValue::FontPath(path.to_path_buf())
                } else {
                    StyleValue::Invalid(ValueErrors::FONT, ValueHelp::FONT)
                }
            },
            PathType::Directory => {
                if is_dir {
                    StyleValue::DirectoryPath(path.to_path_buf())
                }
                else {
                    StyleValue::Invalid(ValueErrors::DIRECTORY, ValueHelp::DIRECTORY)
                }
            },
        }
    }

    fn parse_match(matches: &'static [&'static str], s: &str) -> StyleValue {
        let lc = s.to_ascii_lowercase();

        // Match list is too long
        if matches.len() > 255 { return StyleValue::Invalid(ValueErrors::FATAL_MATCH, ValueHelp::FATAL_MATCH) ; }

        for (i, v) in matches.iter().enumerate() {
            if v.eq_ignore_ascii_case(&lc) {
                return StyleValue::Match(i as u8, v);
            }
        }

        // No match found
        StyleValue::Invalid(ValueErrors::MATCH, matches)
    }

    fn parse_color(s: &str) -> StyleValue {
        if s.is_empty() { return StyleValue::Empty }

        let has_hex_prefix = s.starts_with(Chars::HEX);

        if !has_hex_prefix {
            return StyleValue::Invalid(ValueErrors::COLOR, ValueHelp::COLOR);
        }

        if let Some((r,g,b,a)) = Self::parse_hex_string(&s[1..]){
            StyleValue::Color(Rgba { r, g, b, a })
        } else {
            StyleValue::Invalid(ValueErrors::COLOR, ValueHelp::COLOR)
        }
    }

    fn parse_hex_char(c: char) -> (u8, bool) {
        if let Some(digit) = c.to_digit(16) {
            (digit as u8, true)
        } else {
            (0, false)
        }
    }

    fn parse_hex_string(s: &str) -> Option<(u8, u8, u8, u8)> {
        match s.len() {
            2 => { // #HH -> 0xHHHHHHFF
                let chars: Vec<char> = s.chars().collect();
                let (h1, h1_valid) = Self::parse_hex_char(chars[0]);
                let (h2, h2_valid) = Self::parse_hex_char(chars[1]);

                if !h1_valid || !h2_valid {
                    return None
                }

                let value = (h1 << 4) | h2;

                Some((value, value, value, 0xFF))
            },
            3 => { // #RGB -> 0xRRGGBBFF
                let chars: Vec<char> = s.chars().collect();
                let (r, r_valid) = Self::parse_hex_char(chars[0]);
                let (g, g_valid) = Self::parse_hex_char(chars[1]);
                let (b, b_valid) = Self::parse_hex_char(chars[2]);

                if !r_valid || !g_valid || !b_valid {
                    return None
                }

                Some((r * 0x11, g * 0x11, b * 0x11, 0xFF))
            },
            4 => { // #RGBA -> 0xRRGGBBAA
                let chars: Vec<char> = s.chars().collect();
                let (r, r_valid) = Self::parse_hex_char(chars[0]);
                let (g, g_valid) = Self::parse_hex_char(chars[1]);
                let (b, b_valid) = Self::parse_hex_char(chars[2]);
                let (a, a_valid) = Self::parse_hex_char(chars[3]);

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

}



impl Display for StyleValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StyleValue::Empty => {
                write!(f, "Empty")
            }
            StyleValue::NegativeNumber(dim) | StyleValue::PositiveNumber(dim) => {
                write!(f, "{}", dim)
            }
            StyleValue::DirectoryPath(p) | StyleValue::FontPath(p) | StyleValue::ImagePath(p) => {
                write!(f, "{}", p.to_str().unwrap_or("invalid path"))
            }
            StyleValue::Match(_idx, name) => {
                write!(f, "{}", name)
            }
            StyleValue::Color(rgba) => {
                write!(f, "{}", format!("#{:02X}{:02X}{:02X}{:02X}", rgba.r, rgba.g, rgba.b, rgba.a))
            }
            StyleValue::Forward => {
                write!(f, "Fwd")
            }
            StyleValue::Invalid(_,_) => {
                write!(f, "Err")
            }
            StyleValue::Float(fl) => {
                write!(f, "{}", fl)
            }
            StyleValue::Unset => {
                write!(f, "Uns")
            }
            StyleValue::Font(id) => {
                write!(f, "Fnt({})", id)
            }
            StyleValue::Image(id) => {
                write!(f, "Img({})", id)
            }
            StyleValue::Directory(id) => {
                write!(f, "Dir({})", id)
            }
        }
    }
}