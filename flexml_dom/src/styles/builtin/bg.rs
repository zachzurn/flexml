use crate::styles::builtin::BuiltInStyle;
use crate::styles::style::StyleValue;
use crate::styles::style::StyleValueParser::{ColorParser, MatchParser, UrlParser};

fn apply_bg_color(_: &StyleValue) {
    todo!()
}

pub static BG_COLOR: BuiltInStyle = BuiltInStyle {
    name: "bgColor",
    parser: ColorParser,
    styles: &[],
    apply_style: apply_bg_color,
};


fn apply_bg_image(_: &StyleValue) {
    todo!()
}

pub static BG_IMAGE: BuiltInStyle = BuiltInStyle {
    name: "bgImage",
    parser: UrlParser,
    styles: &[],
    apply_style: apply_bg_image,
};


fn apply_bg_position(_: &StyleValue) {
    todo!()
}

pub static BG_POSITION: BuiltInStyle = BuiltInStyle {
    name: "bgPosition",
    parser: MatchParser(&["center", "top", "bottom", "left", "right"]),
    styles: &[
        ("bgCenter", StyleValue::Match(0)),
    ],
    apply_style: apply_bg_position,
};


fn apply_bg_repeat(_: &StyleValue) {
    todo!()
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


fn apply_bg_size(_: &StyleValue) {
    todo!()
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
