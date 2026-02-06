use crate::styles::context::{BgPosition, BgRepeat, BgSize, Image, StyleContext};
use crate::styles::builtin::{match_value, style_context_color, style_context_match, BuiltInStyle};
use crate::styles::style::{PathType, StyleValue};
use crate::styles::style::StyleValue::Forward;
use crate::styles::style::StyleValueParser::{Color, Match, Path};

fn apply_bg_color(value: &StyleValue, context: &mut StyleContext) {
    if let Some(color) = style_context_color(value) {
        context.set_bg_color(color);
    }
}

pub static BG_COLOR: BuiltInStyle = BuiltInStyle {
    name: "bgColor",
    parser: Color,
    styles: &[("bg", Forward)],
    apply_style: apply_bg_color,
};


fn apply_bg_image(value: &StyleValue, context: &mut StyleContext) {
    if let StyleValue::Image(image_id) = value {
        context.set_bg_image(Image::UserDefined(*image_id))
    };
}

pub static BG_IMAGE: BuiltInStyle = BuiltInStyle {
    name: "bgImage",
    parser: Path(PathType::Image),
    styles: &[],
    apply_style: apply_bg_image,
};

const  BG_POSITION_VARIANTS: &[BgPosition] = &[
    BgPosition::Center,
    BgPosition::Top,
    BgPosition::Bottom,
    BgPosition::Left,
    BgPosition::Right,
];

fn apply_bg_position(value: &StyleValue, context: &mut StyleContext) {
    if let Some(v) = style_context_match(value, BG_POSITION_VARIANTS){
        context.set_bg_position(v);
    }
}

pub static BG_POSITION_MATCHES: &[&str] = &[
    "center",
    "top",
    "bottom",
    "left",
    "right",
];

pub static BG_POSITION: BuiltInStyle = BuiltInStyle {
    name: "bgPosition",
    parser: Match(BG_POSITION_MATCHES),
    styles: &[
        ("bgCenter", match_value(0, BG_POSITION_MATCHES)),
    ],
    apply_style: apply_bg_position,
};



const BG_REPEAT_VARIANTS: &[BgRepeat] = &[
    BgRepeat::Repeat,
    BgRepeat::RepeatX,
    BgRepeat::RepeatY,
    BgRepeat::NoRepeat,
];

fn apply_bg_repeat(value: &StyleValue, context: &mut StyleContext) {
    if let Some(v) = style_context_match(value, BG_REPEAT_VARIANTS) {
        context.set_bg_repeat(v);
    }
}

pub static BG_REPEAT_MATCHES: &[&str] = &[
    "repeat",
    "repeat-x",
    "repeat-y",
    "no-repeat",
];

pub static BG_REPEAT: BuiltInStyle = BuiltInStyle {
    name: "bgRepeat",
    parser: Match(BG_REPEAT_MATCHES),
    styles: &[
        ("noRepeat", match_value(3, BG_REPEAT_MATCHES)),
        ("repeat", match_value(0, BG_REPEAT_MATCHES)),
        ("repeatX", match_value(1, BG_REPEAT_MATCHES)),
        ("repeatY", match_value(2, BG_REPEAT_MATCHES)),
    ],
    apply_style: apply_bg_repeat,
};



const BG_SIZE_VARIANTS: &[BgSize] = &[
    BgSize::Auto,
    BgSize::Cover,
    BgSize::Contain,
];

fn apply_bg_size(value: &StyleValue, context: &mut StyleContext) {
    if let Some(v) = style_context_match(value, BG_SIZE_VARIANTS) {
        context.set_bg_size(v);
    }
}

pub static BG_SIZE_MATCHES: &[&str] = &[
    "auto",
    "cover",
    "contain",
];

pub static BG_SIZE: BuiltInStyle = BuiltInStyle {
    name: "bgSize",
    parser: Match(BG_SIZE_MATCHES),
    styles: &[
        ("cover", match_value(1, BG_SIZE_MATCHES)),
        ("contain", match_value(2, BG_SIZE_MATCHES)),
    ],
    apply_style: apply_bg_size,
};
