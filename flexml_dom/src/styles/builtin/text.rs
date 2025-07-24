use crate::styles::context::{FontFamily, FontStyle, FontWeight, StyleContext, TextAlign, TextDecoration, TextTransform};
use crate::styles::builtin::{apply_color, apply_dimension, apply_match_style, BuiltInStyle};
use crate::styles::style::{StyleValue, UrlType};
use crate::styles::style::StyleValue::{Forward};
use crate::styles::style::StyleValueParser::{ColorParser, MatchParser, PositiveNumberParser, UrlParser};

fn apply_text_color(value: &StyleValue, context: &mut StyleContext) {
    apply_color(value, &mut context.color);
}

pub static TEXT_COLOR: BuiltInStyle = BuiltInStyle {
    name: "color",
    parser: ColorParser,
    styles: &[ ("textColor", Forward) ],
    apply_style: apply_text_color,
};


fn apply_text_font(value: &StyleValue, context: &mut StyleContext) {
    match value {
        StyleValue::Font(value) => {
            context.font_family = FontFamily::UserDefined(*value);
        }
        _ => return
    }
}

pub static TEXT_FONT: BuiltInStyle = BuiltInStyle {
    name: "fontFamily",
    parser: UrlParser(&UrlType::Font),
    styles: &[
        ("font", Forward),
        ("textFont", Forward)
    ],
    apply_style: apply_text_font,
};


fn apply_text_size(value: &StyleValue, context: &mut StyleContext) {
    apply_dimension(value, &mut context.font_size);
}

pub static TEXT_SIZE: BuiltInStyle = BuiltInStyle {
    name: "fontSize",
    parser: PositiveNumberParser,
    styles: &[ ("textSize", Forward) ],
    apply_style: apply_text_size,
};


fn apply_text_style(value: &StyleValue, context: &mut StyleContext) {
    apply_match_style(value, &mut context.font_style, &[
        FontStyle::Normal,
        FontStyle::Italic,
        FontStyle::Oblique
    ])
}

pub static TEXT_STYLE: BuiltInStyle = BuiltInStyle {
    name: "fontStyle",
    parser: MatchParser(&["normal", "italic", "oblique"]),
    styles: &[
        ("textStyle", Forward),
        ("style", Forward),
        ("italic", StyleValue::Match(1)),
    ],
    apply_style: apply_text_style,
};


fn apply_text_weight(value: &StyleValue, context: &mut StyleContext) {
    apply_match_style(value, &mut context.font_weight, &[
        FontWeight::Normal,
        FontWeight::Bold,
        FontWeight::Light,
        FontWeight::Bolder,
        FontWeight::Lighter,
        FontWeight::W100,
        FontWeight::W200,
        FontWeight::W300,
        FontWeight::W400,
        FontWeight::W500,
        FontWeight::W600,
        FontWeight::W700,
        FontWeight::W800,
        FontWeight::W900,
    ])
}

pub static TEXT_WEIGHT: BuiltInStyle = BuiltInStyle {
    name: "fontWeight",
    parser: MatchParser(&[
        "normal",
        "bold",
        "light",
        "bolder",
        "lighter",
        "100",
        "200",
        "300",
        "400",
        "500",
        "600",
        "700",
        "800",
        "900"
    ]),
    styles: &[
        ("fontWeight", Forward),
        ("textWeight", Forward),
        ("normal", StyleValue::Match(0)),
        ("bold", StyleValue::Match(1)),
        ("light", StyleValue::Match(2)),
    ],
    apply_style: apply_text_weight,
};


fn apply_text_letter_spacing(value: &StyleValue, context: &mut StyleContext) {
    apply_dimension(value, &mut context.letter_spacing);
}

pub static TEXT_LETTER_SPACING: BuiltInStyle = BuiltInStyle {
    name: "letterSpacing",
    parser: PositiveNumberParser,
    styles: &[ ("textLetterSpacing", Forward) ],
    apply_style: apply_text_letter_spacing,
};


fn apply_text_word_spacing(value: &StyleValue, context: &mut StyleContext) {
    apply_dimension(value, &mut context.word_spacing);
}

pub static TEXT_WORD_SPACING: BuiltInStyle = BuiltInStyle {
    name: "wordSpacing",
    parser: PositiveNumberParser,
    styles: &[ ("textWordSpacing", Forward) ],
    apply_style: apply_text_word_spacing,
};


fn apply_text_line_height(value: &StyleValue, context: &mut StyleContext) {
    apply_dimension(value, &mut context.line_height);
}

pub static TEXT_LINE_HEIGHT: BuiltInStyle = BuiltInStyle {
    name: "lineHeight",
    parser: PositiveNumberParser,
    styles: &[ ("textLineHeight", Forward) ],
    apply_style: apply_text_line_height,
};


fn apply_text_transform(value: &StyleValue, context: &mut StyleContext) {
    apply_match_style(value, &mut context.text_transform, &[
        TextTransform::None,
        TextTransform::Capitalize,
        TextTransform::Uppercase,
        TextTransform::Lowercase,
    ])
}

pub static TEXT_TRANSFORM: BuiltInStyle = BuiltInStyle {
    name: "textTransform",
    parser: MatchParser(&["none", "capitalize", "uppercase", "lowercase"]),
    styles: &[
        ("uppercase", StyleValue::Match(2)),
        ("lowercase", StyleValue::Match(3)),
        ("capitalize", StyleValue::Match(4)),
    ],
    apply_style: apply_text_transform,
};


fn apply_text_align(value: &StyleValue, context: &mut StyleContext) {
    apply_match_style(value, &mut context.text_align, &[
        TextAlign::Left,
        TextAlign::Right,
        TextAlign::Center,
        TextAlign::Justify
    ])
}

pub static TEXT_ALIGN: BuiltInStyle = BuiltInStyle {
    name: "textAlign",
    parser: MatchParser(&["left", "right", "center", "justify"]),
    styles: &[
        ("left", StyleValue::Match(0)),
        ("right", StyleValue::Match(1)),
        ("center", StyleValue::Match(2)),
        ("justify", StyleValue::Match(3)),
    ],
    apply_style: apply_text_align,
};


fn apply_text_decoration(value: &StyleValue, context: &mut StyleContext) {
    apply_match_style(value, &mut context.text_decoration, &[
        TextDecoration::None,
        TextDecoration::Underline,
        TextDecoration::Overline,
        TextDecoration::LineThrough
    ])
}

pub static TEXT_DECORATION: BuiltInStyle = BuiltInStyle {
    name: "textDecoration",
    parser: MatchParser(&["none", "underline", "overline", "line-through"]),
    styles: &[
        ("underline", StyleValue::Match(1)),
        ("overline", StyleValue::Match(2)),
        ("strike", StyleValue::Match(3)),
    ],
    apply_style: apply_text_decoration,
};
