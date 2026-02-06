use crate::styles::context::{FontFamily, FontStyle, StyleContext, TextAlign, TextDecoration, TextTransform};
use crate::styles::builtin::{dimension_to_context, match_value, style_context_color, style_context_match, BuiltInStyle};
use crate::styles::style::{PathType, StyleValue};
use crate::styles::style::StyleValue::{Forward};
use crate::styles::style::StyleValueParser::{Color, Match, Path, PositiveNumber};

fn apply_text_color(value: &StyleValue, context: &mut StyleContext) {
    if let Some(color) = style_context_color(value) {
        context.set_color(color);
    }
}

pub static TEXT_COLOR: BuiltInStyle = BuiltInStyle {
    name: "color",
    parser: Color,
    styles: &[ ("textColor", Forward) ],
    apply_style: apply_text_color,
};


fn apply_text_font(value: &StyleValue, context: &mut StyleContext) {
    if let StyleValue::Font(value) = value {
        context.set_font_family(FontFamily::UserDefined(*value));
    }
}

pub static TEXT_FONT: BuiltInStyle = BuiltInStyle {
    name: "fontFamily",
    parser: Path(PathType::Font),
    styles: &[
        ("font", Forward),
        ("textFont", Forward)
    ],
    apply_style: apply_text_font,
};


fn apply_text_size(value: &StyleValue, context: &mut StyleContext) {
    if let Some(d) = dimension_to_context(value) {
        context.set_font_size(d);
    }
}

pub static TEXT_SIZE: BuiltInStyle = BuiltInStyle {
    name: "fontSize",
    parser: PositiveNumber,
    styles: &[ ("textSize", Forward) ],
    apply_style: apply_text_size,
};


const FONT_STYLE_VARIANTS: &[FontStyle] = &[
    FontStyle::Normal,
    FontStyle::Italic,
    FontStyle::Oblique,
];

fn apply_text_style(value: &StyleValue, context: &mut StyleContext) {
    if let Some(v) = style_context_match(value, FONT_STYLE_VARIANTS) {
        context.set_font_style(v);
    }
}


pub static TEXT_STYLE_MATCHES: &[&str] = &[
    "normal",
    "italic",
    "oblique",
];

pub static TEXT_STYLE: BuiltInStyle = BuiltInStyle {
    name: "fontStyle",
    parser: Match(TEXT_STYLE_MATCHES),
    styles: &[
        ("textStyle", Forward),
        ("style", Forward),
        ("italic", match_value(1, TEXT_STYLE_MATCHES)),
    ],
    apply_style: apply_text_style,
};



const FONT_WEIGHT_VARIANTS: &[u16] = &[
    100,
    700,
    400,
    800,
    300,
    100,
    200,
    300,
    400,
    500,
    600,
    700,
    800,
    900,
];

fn apply_text_weight(value: &StyleValue, context: &mut StyleContext) {
    if let Some(v) = style_context_match(value, FONT_WEIGHT_VARIANTS) {
        context.set_font_weight(v);
    }
}

pub static TEXT_WEIGHT_MATCHES: &[&str] = &[
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
    "900",
];

pub static TEXT_WEIGHT: BuiltInStyle = BuiltInStyle {
    name: "fontWeight",
    parser: Match(TEXT_WEIGHT_MATCHES),
    styles: &[
        ("fontWeight", Forward),
        ("textWeight", Forward),
        ("normal", match_value(0, TEXT_WEIGHT_MATCHES)),
        ("bold", match_value(1, TEXT_WEIGHT_MATCHES)),
        ("light", match_value(2, TEXT_WEIGHT_MATCHES)),
    ],
    apply_style: apply_text_weight,
};


fn apply_text_letter_spacing(value: &StyleValue, context: &mut StyleContext) {
    if let Some(d) = dimension_to_context(value) {
        context.set_letter_spacing(d);
    }
}

pub static TEXT_LETTER_SPACING: BuiltInStyle = BuiltInStyle {
    name: "letterSpacing",
    parser: PositiveNumber,
    styles: &[ ("textLetterSpacing", Forward) ],
    apply_style: apply_text_letter_spacing,
};


fn apply_text_word_spacing(value: &StyleValue, context: &mut StyleContext) {
    if let Some(d) = dimension_to_context(value) {
        context.set_word_spacing(d);
    }
}

pub static TEXT_WORD_SPACING: BuiltInStyle = BuiltInStyle {
    name: "wordSpacing",
    parser: PositiveNumber,
    styles: &[ ("textWordSpacing", Forward) ],
    apply_style: apply_text_word_spacing,
};


fn apply_text_line_height(value: &StyleValue, context: &mut StyleContext) {
    if let Some(d) = dimension_to_context(value) {
        context.set_line_height(d);
    }
}

pub static TEXT_LINE_HEIGHT: BuiltInStyle = BuiltInStyle {
    name: "lineHeight",
    parser: PositiveNumber,
    styles: &[ ("textLineHeight", Forward) ],
    apply_style: apply_text_line_height,
};


const TEXT_TRANSFORM_VARIANTS: &[TextTransform] = &[
    TextTransform::None,
    TextTransform::Capitalize,
    TextTransform::Uppercase,
    TextTransform::Lowercase,
];

fn apply_text_transform(value: &StyleValue, context: &mut StyleContext) {
    if let Some(v) = style_context_match(value, TEXT_TRANSFORM_VARIANTS) {
        context.set_text_transform(v);
    }
}

pub static TEXT_TRANSFORM_MATCHES: &[&str] = &[
    "none",
    "capitalize",
    "uppercase",
    "lowercase",
];

pub static TEXT_TRANSFORM: BuiltInStyle = BuiltInStyle {
    name: "textTransform",
    parser: Match(TEXT_TRANSFORM_MATCHES),
    styles: &[
        ("uppercase", match_value(2, TEXT_TRANSFORM_MATCHES)),
        ("lowercase", match_value(3, TEXT_TRANSFORM_MATCHES)),
        ("capitalize", match_value(1, TEXT_TRANSFORM_MATCHES)),
    ],
    apply_style: apply_text_transform,
};



const TEXT_ALIGN_VARIANTS: &[TextAlign] = &[
    TextAlign::Left,
    TextAlign::Right,
    TextAlign::Center,
    TextAlign::Justify,
];

fn apply_text_align(value: &StyleValue, context: &mut StyleContext) {
    if let Some(v) = style_context_match(value, TEXT_ALIGN_VARIANTS) {
        context.set_text_align(v);
    }
}

pub static TEXT_ALIGN_MATCHES: &[&str] = &[
    "left",
    "right",
    "center",
    "justify",
];

pub static TEXT_ALIGN: BuiltInStyle = BuiltInStyle {
    name: "textAlign",
    parser: Match(TEXT_ALIGN_MATCHES),
    styles: &[
        ("left", match_value(0, TEXT_ALIGN_MATCHES)),
        ("right", match_value(1, TEXT_ALIGN_MATCHES)),
        ("center", match_value(2, TEXT_ALIGN_MATCHES)),
        ("justify", match_value(3, TEXT_ALIGN_MATCHES)),
    ],
    apply_style: apply_text_align,
};



const TEXT_DECORATION_VARIANTS: &[TextDecoration] = &[
    TextDecoration::None,
    TextDecoration::Underline,
    TextDecoration::Overline,
    TextDecoration::LineThrough,
];

fn apply_text_decoration(value: &StyleValue, context: &mut StyleContext) {
    if let Some(v) = style_context_match(value, TEXT_DECORATION_VARIANTS) {
        context.set_text_decoration(v);
    }
}

pub static TEXT_DECORATION_MATCHES: &[&str] = &[
    "none",
    "underline",
    "overline",
    "line-through",
];

pub static TEXT_DECORATION: BuiltInStyle = BuiltInStyle {
    name: "textDecoration",
    parser: Match(TEXT_DECORATION_MATCHES),
    styles: &[
        ("underline", match_value(1, TEXT_DECORATION_MATCHES)),
        ("overline", match_value(2, TEXT_DECORATION_MATCHES)),
        ("strike", match_value(3, TEXT_DECORATION_MATCHES)),
    ],
    apply_style: apply_text_decoration,
};
