use crate::styles::context::{BorderStyle, StyleContext};
use crate::styles::builtin::{apply_color, apply_dimension, apply_match_style, BuiltInStyle};
use crate::styles::style::StyleValue;
use crate::styles::style::StyleValueParser::{ColorParser, MatchParser, NumberParser};


fn apply_border_radius(value: &StyleValue, context: &mut StyleContext) {
    apply_dimension(value, &mut context.border_bottom_left_radius);
    apply_dimension(value, &mut context.border_bottom_right_radius);
    apply_dimension(value, &mut context.border_top_left_radius);
    apply_dimension(value, &mut context.border_top_right_radius);
}

pub static BORDER_RADIUS: BuiltInStyle = BuiltInStyle {
    name: "borderRadius",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_border_radius,
};



fn apply_border_bottom_left_radius(value: &StyleValue, context: &mut StyleContext) {
    apply_dimension(value, &mut context.border_bottom_left_radius);
}

pub static BORDER_BOTTOM_LEFT_RADIUS: BuiltInStyle = BuiltInStyle {
    name: "borderBottomLeftRadius",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_border_bottom_left_radius,
};


fn apply_border_bottom_right_radius(value: &StyleValue, context: &mut StyleContext) {
    apply_dimension(value, &mut context.border_bottom_right_radius);
}

pub static BORDER_BOTTOM_RIGHT_RADIUS: BuiltInStyle = BuiltInStyle {
    name: "borderBottomRightRadius",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_border_bottom_right_radius,
};

fn apply_border_top_left_radius(value: &StyleValue, context: &mut StyleContext) {
    apply_dimension(value, &mut context.border_top_left_radius);
}

pub static BORDER_TOP_LEFT_RADIUS: BuiltInStyle = BuiltInStyle {
    name: "borderTopLeftRadius",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_border_top_left_radius,
};


fn apply_border_top_right_radius(value: &StyleValue, context: &mut StyleContext) {
    apply_dimension(value, &mut context.border_top_right_radius);
}

pub static BORDER_TOP_RIGHT_RADIUS: BuiltInStyle = BuiltInStyle {
    name: "borderTopRightRadius",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_border_top_right_radius,
};


fn apply_border_color(value: &StyleValue, context: &mut StyleContext) {
    apply_color(value, &mut context.border_color);
}

pub static BORDER_COLOR: BuiltInStyle = BuiltInStyle {
    name: "borderColor",
    parser: ColorParser,
    styles: &[],
    apply_style: apply_border_color,
};



fn apply_border_style(value: &StyleValue, context: &mut StyleContext) {
    apply_match_style(value, &mut context.border_style, &[
        BorderStyle::Solid,
        BorderStyle::Dashed,
        BorderStyle::Dotted,
        BorderStyle::None
    ])
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


fn apply_border_width(value: &StyleValue, context: &mut StyleContext) {
    apply_dimension(value, &mut context.border_width);
}

pub static BORDER_WIDTH: BuiltInStyle = BuiltInStyle {
    name: "borderWidth",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_border_width,
};
