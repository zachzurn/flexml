use crate::styles::context::{BgPosition, BgRepeat, BgSize, Image, StyleContext};
use crate::styles::builtin::{style_context_color, style_context_match, BuiltInStyle};
use crate::styles::style::{StyleValue, UrlType};
use crate::styles::style::StyleValue::Forward;
use crate::styles::style::StyleValueParser::{Color, Match, Url};

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
    parser: Url(&UrlType::Image),
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

pub static BG_POSITION: BuiltInStyle = BuiltInStyle {
    name: "bgPosition",
    parser: Match(&["center", "top", "bottom", "left", "right"]),
    styles: &[
        ("bgCenter", StyleValue::Match(0)),
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

pub static BG_REPEAT: BuiltInStyle = BuiltInStyle {
    name: "bgRepeat",
    parser: Match(&["repeat", "repeat-x", "repeat-y", "no-repeat"]),
    styles: &[
        ("noRepeat", StyleValue::Match(3)),
        ("repeat", StyleValue::Match(0)),
        ("repeatX", StyleValue::Match(1)),
        ("repeatY", StyleValue::Match(2)),
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

pub static BG_SIZE: BuiltInStyle = BuiltInStyle {
    name: "bgSize",
    parser: Match(&["auto", "cover", "contain"]),
    styles: &[
        ("cover", StyleValue::Match(1)),
        ("contain", StyleValue::Match(2)),
    ],
    apply_style: apply_bg_size,
};
