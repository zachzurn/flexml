use crate::styles::context::{BgPosition, BgRepeat, BgSize, Color, Image, StyleContext};
use crate::styles::builtin::{apply_color, apply_match_style, BuiltInStyle};
use crate::styles::style::{StyleValue, UrlType};
use crate::styles::style::StyleValueParser::{ColorParser, MatchParser, UrlParser};

fn apply_bg_color(value: &StyleValue, context: &mut StyleContext) {
    apply_color(value, &mut context.bg_color);
}

pub static BG_COLOR: BuiltInStyle = BuiltInStyle {
    name: "bgColor",
    parser: ColorParser,
    styles: &[],
    apply_style: apply_bg_color,
};


fn apply_bg_image(value: &StyleValue, context: &mut StyleContext) {
    if let StyleValue::Image(image_id) = value {
        context.set_bg_image(Image::UserDefined(*image_id))
    };
}

pub static BG_IMAGE: BuiltInStyle = BuiltInStyle {
    name: "bgImage",
    parser: UrlParser(&UrlType::Image),
    styles: &[],
    apply_style: apply_bg_image,
};


fn apply_bg_position(value: &StyleValue, context: &mut StyleContext) {
    apply_match_style(value, &mut context.bg_position, &[
        BgPosition::Center,
        BgPosition::Top,
        BgPosition::Bottom,
        BgPosition::Left,
        BgPosition::Right,
    ]);
}

pub static BG_POSITION: BuiltInStyle = BuiltInStyle {
    name: "bgPosition",
    parser: MatchParser(&["center", "top", "bottom", "left", "right"]),
    styles: &[
        ("bgCenter", StyleValue::Match(0)),
    ],
    apply_style: apply_bg_position,
};


fn apply_bg_repeat(value: &StyleValue, context: &mut StyleContext) {
    apply_match_style(value, &mut context.bg_repeat, &[
        BgRepeat::Repeat,
        BgRepeat::RepeatX,
        BgRepeat::RepeatY,
        BgRepeat::NoRepeat
    ]);
}

pub static BG_REPEAT: BuiltInStyle = BuiltInStyle {
    name: "bgRepeat",
    parser: MatchParser(&["repeat", "repeat-x", "repeat-y", "no-repeat"]),
    styles: &[
        ("noRepeat", StyleValue::Match(3)),
        ("repeat", StyleValue::Match(0)),
        ("repeatX", StyleValue::Match(1)),
        ("repeatY", StyleValue::Match(2)),
    ],
    apply_style: apply_bg_repeat,
};


fn apply_bg_size(value: &StyleValue, context: &mut StyleContext) {
    apply_match_style(value, &mut context.bg_size, &[
        BgSize::Auto,
        BgSize::Cover,
        BgSize::Contain
    ]);
}

pub static BG_SIZE: BuiltInStyle = BuiltInStyle {
    name: "bgSize",
    parser: MatchParser(&["auto", "cover", "contain"]),
    styles: &[
        ("cover", StyleValue::Match(1)),
        ("contain", StyleValue::Match(2)),
    ],
    apply_style: apply_bg_size,
};
