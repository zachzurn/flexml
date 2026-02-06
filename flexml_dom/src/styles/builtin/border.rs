use crate::styles::context::{BorderStyle, StyleContext};
use crate::styles::builtin::{dimension_to_context, match_value, style_context_color, style_context_match, BuiltInStyle};
use crate::styles::style::StyleValue;
use crate::styles::style::StyleValueParser::{Color, Match, Number};


fn apply_border_radius(value: &StyleValue, context: &mut StyleContext) {
    if let Some(d) = dimension_to_context(value) {
        context.set_border_top_left_radius(d);
        context.set_border_top_right_radius(d);
        context.set_border_bottom_left_radius(d);
        context.set_border_bottom_right_radius(d);
    }
}

pub static BORDER_RADIUS: BuiltInStyle = BuiltInStyle {
    name: "borderRadius",
    parser: Number,
    styles: &[],
    apply_style: apply_border_radius,
};



fn apply_border_bottom_left_radius(value: &StyleValue, context: &mut StyleContext) {
    if let Some(d) = dimension_to_context(value) {
        context.set_border_bottom_left_radius(d);
    }
}

pub static BORDER_BOTTOM_LEFT_RADIUS: BuiltInStyle = BuiltInStyle {
    name: "borderBottomLeftRadius",
    parser: Number,
    styles: &[],
    apply_style: apply_border_bottom_left_radius,
};


fn apply_border_bottom_right_radius(value: &StyleValue, context: &mut StyleContext) {
    if let Some(d) = dimension_to_context(value) {
        context.set_border_bottom_right_radius(d);
    }
}

pub static BORDER_BOTTOM_RIGHT_RADIUS: BuiltInStyle = BuiltInStyle {
    name: "borderBottomRightRadius",
    parser: Number,
    styles: &[],
    apply_style: apply_border_bottom_right_radius,
};

fn apply_border_top_left_radius(value: &StyleValue, context: &mut StyleContext) {
    if let Some(d) = dimension_to_context(value) {
        context.set_border_top_left_radius(d);
    }
}

pub static BORDER_TOP_LEFT_RADIUS: BuiltInStyle = BuiltInStyle {
    name: "borderTopLeftRadius",
    parser: Number,
    styles: &[],
    apply_style: apply_border_top_left_radius,
};


fn apply_border_top_right_radius(value: &StyleValue, context: &mut StyleContext) {
    if let Some(d) = dimension_to_context(value) {
        context.set_border_top_right_radius(d);
    }
}

pub static BORDER_TOP_RIGHT_RADIUS: BuiltInStyle = BuiltInStyle {
    name: "borderTopRightRadius",
    parser: Number,
    styles: &[],
    apply_style: apply_border_top_right_radius,
};


fn apply_border_color(value: &StyleValue, context: &mut StyleContext) {
    if let Some(color) = style_context_color(value) {
        context.set_border_color(color);
    }
}

pub static BORDER_COLOR: BuiltInStyle = BuiltInStyle {
    name: "borderColor",
    parser: Color,
    styles: &[],
    apply_style: apply_border_color,
};



const BORDER_STYLE_VARIANTS: &[BorderStyle] = &[
    BorderStyle::Solid,
    BorderStyle::Dashed,
    BorderStyle::Dotted,
    BorderStyle::None,
];

fn apply_border_style(value: &StyleValue, context: &mut StyleContext) {
    if let Some(v) = style_context_match(value, BORDER_STYLE_VARIANTS) {
        context.set_border_style(v);
    }
}


pub static BORDER_STYLE_MATCHES: &[&str] = &[
    "solid",
    "dashed",
    "dotted",
    "none",
];

pub static BORDER_STYLE: BuiltInStyle = BuiltInStyle {
    name: "borderStyle",
    parser: Match(BORDER_STYLE_MATCHES),
    styles: &[
        ("solidBorder", match_value(0, BORDER_STYLE_MATCHES)),
        ("dashedBorder", match_value(1, BORDER_STYLE_MATCHES)),
        ("dottedBorder", match_value(2, BORDER_STYLE_MATCHES)),
        ("borderless", match_value(3, BORDER_STYLE_MATCHES)),
    ],
    apply_style: apply_border_style,
};



fn apply_border_width(value: &StyleValue, context: &mut StyleContext) {
    if let Some(d) = dimension_to_context(value) {
        context.set_border_width(d);
    }
}

pub static BORDER_WIDTH: BuiltInStyle = BuiltInStyle {
    name: "borderWidth",
    parser: Number,
    styles: &[],
    apply_style: apply_border_width,
};
