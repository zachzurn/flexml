use crate::styles::context::{AlignContent, AlignItems, AlignSelf, Dimension, FlexDirection, FlexWrap, JustifyContent, StyleContext};
use crate::styles::builtin::{dimension_to_context, float_to_context, length_to_context, match_value, style_context_match, BuiltInStyle};
use crate::styles::style::{StyleValue};
use crate::styles::style::StyleValueParser::{Float, MatchOrFloat, Match, Number};


const ALIGN_CONTENT_VARIANTS: &[AlignContent] = &[
    AlignContent::FlexStart,
    AlignContent::FlexEnd,
    AlignContent::Center,
    AlignContent::SpaceBetween,
    AlignContent::SpaceAround,
    AlignContent::Stretch,
];

fn apply_align_content(value: &StyleValue, context: &mut StyleContext) {
    if let Some(v) = style_context_match(value, ALIGN_CONTENT_VARIANTS) {
        context.set_align_content(v);
    }
}

pub static ALIGN_CONTENT_MATCHES: &[&str] = &[
    "flex-start",
    "flex-end",
    "center",
    "space-between",
    "space-around",
    "stretch",
];

pub static ALIGN_CONTENT: BuiltInStyle = BuiltInStyle {
    name: "alignContent",
    parser: Match(ALIGN_CONTENT_MATCHES),
    styles: &[
        ("contentStart", match_value(0, ALIGN_CONTENT_MATCHES)),
        ("contentStretch", match_value(5, ALIGN_CONTENT_MATCHES)),
    ],
    apply_style: apply_align_content,
};




const FLEX_DIRECTION_VARIANTS: &[FlexDirection] = &[
    FlexDirection::Row,
    FlexDirection::RowReverse,
    FlexDirection::Column,
    FlexDirection::ColumnReverse,
];

fn apply_flex_direction(value: &StyleValue, context: &mut StyleContext) {
    if let Some(v) = style_context_match(value, FLEX_DIRECTION_VARIANTS) {
        context.set_flex_direction(v);
    }
}

pub static FLEX_DIRECTION_MATCHES: &[&str] = &[
    "row",
    "row-reverse",
    "column",
    "column-reverse",
];

pub static FLEX_DIRECTION: BuiltInStyle = BuiltInStyle {
    name: "flexDirection",
    parser: Match(FLEX_DIRECTION_MATCHES),
    styles: &[
        ("row", match_value(0, FLEX_DIRECTION_MATCHES)),
        ("col", match_value(2, FLEX_DIRECTION_MATCHES)),
        ("column", match_value(2, FLEX_DIRECTION_MATCHES)),
    ],
    apply_style: apply_flex_direction,
};


pub static FLEX_BASIS_VARIANTS: &[Dimension] = &[
    Dimension::Auto,
    Dimension::Content
];

fn apply_flex_basis(value: &StyleValue, context: &mut StyleContext) {
    if let Some(d) = length_to_context(value, FLEX_BASIS_VARIANTS) {
        context.set_flex_basis(d);
    }
}

pub static FLEX_BASIS_MATCHES: &[&str] = &[
    "auto",
    "content",
];

pub static FLEX_BASIS: BuiltInStyle = BuiltInStyle {
    name: "flexBasis",
    parser: MatchOrFloat(FLEX_BASIS_MATCHES),
    styles: &[
        ("basisAuto", match_value(0, FLEX_BASIS_MATCHES)),
        ("basisContent", match_value(1, FLEX_BASIS_MATCHES)),
    ],
    apply_style: apply_flex_basis,
};


fn apply_flex_grow(value: &StyleValue, context: &mut StyleContext) {
    if let Some(f) = float_to_context(value) {
        context.set_flex_grow(f);
    }
}

pub static FLEX_GROW: BuiltInStyle = BuiltInStyle {
    name: "flexGrow",
    parser: Float,
    styles: &[
        ("grow", StyleValue::Float(1.0)),
    ],
    apply_style: apply_flex_grow,
};


fn apply_flex_shrink(value: &StyleValue, context: &mut StyleContext) {
    if let Some(f) = float_to_context(value) {
        context.set_flex_shrink(f);
    }
}

pub static FLEX_SHRINK: BuiltInStyle = BuiltInStyle {
    name: "flexShrink",
    parser: Float,
    styles: &[
        ("shrink", StyleValue::Float(1.0)),
    ],
    apply_style: apply_flex_shrink,
};



const FLEX_WRAP_VARIANTS: &[FlexWrap] = &[
    FlexWrap::NoWrap,
    FlexWrap::Wrap,
    FlexWrap::WrapReverse,
];

fn apply_flex_wrap(value: &StyleValue, context: &mut StyleContext) {
    if let Some(v) = style_context_match(value, FLEX_WRAP_VARIANTS) {
        context.set_flex_wrap(v);
    }
}

pub static FLEX_WRAP_MATCHES: &[&str] = &[
    "nowrap",
    "wrap",
    "wrap-reverse",
];

pub static FLEX_WRAP: BuiltInStyle = BuiltInStyle {
    name: "flexWrap",
    parser: Match(FLEX_WRAP_MATCHES),
    styles: &[
        ("noWrap", match_value(0, FLEX_WRAP_MATCHES)),
        ("wrapReverse", match_value(2, FLEX_WRAP_MATCHES)),
    ],
    apply_style: apply_flex_wrap,
};


fn apply_column_gap(value: &StyleValue, context: &mut StyleContext) {
    if let Some(d) = dimension_to_context(value) {
        context.set_column_gap(d);
    }
}

pub static COLUMN_GAP: BuiltInStyle = BuiltInStyle {
    name: "columnGap",
    parser: Number,
    styles: &[],
    apply_style: apply_column_gap,
};


const ALIGN_ITEMS_VARIANTS: &[AlignItems] = &[
    AlignItems::FlexStart,
    AlignItems::FlexEnd,
    AlignItems::Center,
    AlignItems::Baseline,
    AlignItems::Stretch,
];

fn apply_align_items(value: &StyleValue, context: &mut StyleContext) {
    if let Some(v) = style_context_match(value, ALIGN_ITEMS_VARIANTS) {
        context.set_align_items(v);
    }
}

pub static ALIGN_ITEMS_MATCHES: &[&str] = &[
    "flex-start",
    "flex-end",
    "center",
    "baseline",
    "stretch",
];

pub static ALIGN_ITEMS: BuiltInStyle = BuiltInStyle {
    name: "alignItems",
    parser: Match(ALIGN_ITEMS_MATCHES),
    styles: &[
        ("itemsStart", match_value(0, ALIGN_ITEMS_MATCHES)),
        ("itemsCenter", match_value(2, ALIGN_ITEMS_MATCHES)),
    ],
    apply_style: apply_align_items,
};


const ALIGN_SELF_VARIANTS: &[AlignSelf] = &[
    AlignSelf::Auto,
    AlignSelf::FlexStart,
    AlignSelf::FlexEnd,
    AlignSelf::Center,
    AlignSelf::Baseline,
    AlignSelf::Stretch,
];

fn apply_align_self(value: &StyleValue, context: &mut StyleContext) {
    if let Some(v) = style_context_match(value, ALIGN_SELF_VARIANTS) {
        context.set_align_self(v);
    }
}


pub static ALIGN_SELF_MATCHES: &[&str] = &[
    "auto",
    "flex-start",
    "flex-end",
    "center",
    "baseline",
    "stretch",
];

pub static ALIGN_SELF: BuiltInStyle = BuiltInStyle {
    name: "alignSelf",
    parser: Match(ALIGN_SELF_MATCHES),
    styles: &[
        ("selfStart", match_value(1, ALIGN_SELF_MATCHES)),
        ("selfStretch", match_value(5, ALIGN_SELF_MATCHES)),
    ],
    apply_style: apply_align_self,
};


fn apply_gap(value: &StyleValue, context: &mut StyleContext) {
    if let Some(d) = dimension_to_context(value) {
        context.set_row_gap(d);
        context.set_column_gap(d);
    }
}

pub static GAP: BuiltInStyle = BuiltInStyle {
    name: "gap",
    parser: Number,
    styles: &[],
    apply_style: apply_gap,
};


const JUSTIFY_CONTENT_VARIANTS: &[JustifyContent] = &[
    JustifyContent::FlexStart,
    JustifyContent::FlexEnd,
    JustifyContent::Center,
    JustifyContent::SpaceBetween,
    JustifyContent::SpaceAround,
    JustifyContent::SpaceEvenly,
];

fn apply_justify_content(value: &StyleValue, context: &mut StyleContext) {
    if let Some(v) = style_context_match(value, JUSTIFY_CONTENT_VARIANTS) {
        context.set_justify_content(v);
    }
}

pub static JUSTIFY_CONTENT_MATCHES: &[&str] = &[
    "flex-start",
    "flex-end",
    "center",
    "space-between",
    "space-around",
    "space-evenly"
];

pub static JUSTIFY_CONTENT: BuiltInStyle = BuiltInStyle {
    name: "justifyContent",
    parser: Match(JUSTIFY_CONTENT_MATCHES),
    styles: &[
        ("contentStart", match_value(0, JUSTIFY_CONTENT_MATCHES)),
        ("contentEnd", match_value(1, JUSTIFY_CONTENT_MATCHES)),
        ("contentCenter", match_value(2, JUSTIFY_CONTENT_MATCHES)),
        ("contentSpaceBetween", match_value(3, JUSTIFY_CONTENT_MATCHES)),
        ("contentSpaceAround", match_value(4, JUSTIFY_CONTENT_MATCHES)),
        ("contentSpaceEvenly", match_value(5, JUSTIFY_CONTENT_MATCHES)),
    ],
    apply_style: apply_justify_content,
};



fn apply_row_gap(value: &StyleValue, context: &mut StyleContext) {
    if let Some(d) = dimension_to_context(value) {
        context.set_row_gap(d);
    }
}

pub static ROW_GAP: BuiltInStyle = BuiltInStyle {
    name: "rowGap",
    parser: Number,
    styles: &[],
    apply_style: apply_row_gap,
};