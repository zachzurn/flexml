use crate::styles::builtin::BuiltInStyle;
use crate::styles::style::StyleValue;
use crate::styles::style::StyleValueParser::{ColorParser, MatchParser, NumberParser};


fn apply_border_radius(_: &StyleValue) {
    todo!()
}

pub static BORDER_RADIUS: BuiltInStyle = BuiltInStyle {
    name: "borderRadius",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_border_radius,
};



fn apply_border_bottom_left_radius(_: &StyleValue) {
    todo!()
}

pub static BORDER_BOTTOM_LEFT_RADIUS: BuiltInStyle = BuiltInStyle {
    name: "borderBottomLeftRadius",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_border_bottom_left_radius,
};


fn apply_border_bottom_right_radius(_: &StyleValue) {
    todo!()
}

pub static BORDER_BOTTOM_RIGHT_RADIUS: BuiltInStyle = BuiltInStyle {
    name: "borderBottomRightRadius",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_border_bottom_right_radius,
};

fn apply_border_top_left_radius(_: &StyleValue) {
    todo!()
}

pub static BORDER_TOP_LEFT_RADIUS: BuiltInStyle = BuiltInStyle {
    name: "borderTopLeftRadius",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_border_top_left_radius,
};


fn apply_border_top_right_radius(_: &StyleValue) {
    todo!()
}

pub static BORDER_TOP_RIGHT_RADIUS: BuiltInStyle = BuiltInStyle {
    name: "borderTopRightRadius",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_border_top_right_radius,
};


fn apply_border_color(_: &StyleValue) {
    todo!()
}

pub static BORDER_COLOR: BuiltInStyle = BuiltInStyle {
    name: "borderColor",
    parser: ColorParser,
    styles: &[],
    apply_style: apply_border_color,
};



fn apply_border_style(_: &StyleValue) {
    todo!()
}

pub static BORDER_STYLE: BuiltInStyle = BuiltInStyle {
    name: "borderStyle",
    parser: MatchParser(&["solid", "dashed", "dotted", "none"]),
    styles: &[
        ("solidBorder", StyleValue::Match(0)),
        ("dashedBorder", StyleValue::Match(1)),
        ("dottedBorder", StyleValue::Match(2)),
        ("borderless", StyleValue::Match(2)),
    ],
    apply_style: apply_border_style,
};


fn apply_border_width(_: &StyleValue) {
    todo!()
}

pub static BORDER_WIDTH: BuiltInStyle = BuiltInStyle {
    name: "borderWidth",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_border_width,
};
