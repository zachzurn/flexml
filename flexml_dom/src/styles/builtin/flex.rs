use crate::styles::context::{AlignContent, AlignItems, AlignSelf, Dimension, FlexDirection, FlexWrap, JustifyContent, StyleContext};
use crate::styles::builtin::{dimension_to_context, float_to_context, length_to_context, style_context_match, BuiltInStyle};
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


pub static ALIGN_CONTENT: BuiltInStyle = BuiltInStyle {
    name: "alignContent",
    parser: Match(&[
        "flex-start",
        "flex-end",
        "center",
        "space-between",
        "space-around",
        "stretch"
    ]),
    styles: &[
        ("contentStart", StyleValue::Match(0)),
        ("contentStretch", StyleValue::Match(5)),
    ],
    apply_style: apply_align_content
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

pub static FLEX_DIRECTION: BuiltInStyle = BuiltInStyle {
    name: "flexDirection",
    parser: Match(&["row", "row-reverse", "column", "column-reverse"]),
    styles: &[
        ("row", StyleValue::Match(0)),
        ("col", StyleValue::Match(2)),
        ("column", StyleValue::Match(2)),
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

pub static FLEX_BASIS: BuiltInStyle = BuiltInStyle {
    name: "flexBasis",
    parser: MatchOrFloat(&["auto", "content"]),
    styles: &[
        ("basisAuto", StyleValue::Match(0)),
        ("basisContent", StyleValue::Match(1)),
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

pub static FLEX_WRAP: BuiltInStyle = BuiltInStyle {
    name: "flexWrap",
    parser: Match(&["nowrap", "wrap", "wrap-reverse"]),
    styles: &[
        ("noWrap", StyleValue::Match(0)),
        ("wrapReverse", StyleValue::Match(2)),
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

pub static ALIGN_ITEMS: BuiltInStyle = BuiltInStyle {
    name: "alignItems",
    parser: Match(&["flex-start", "flex-end", "center", "baseline", "stretch"]),
    styles: &[
        ("itemsStart", StyleValue::Match(0)),
        ("itemsCenter", StyleValue::Match(2)),
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


pub static ALIGN_SELF: BuiltInStyle = BuiltInStyle {
    name: "alignSelf",
    parser: Match(&["auto", "flex-start", "flex-end", "center", "baseline", "stretch"]),
    styles: &[
        ("selfStart", StyleValue::Match(1)),
        ("selfStretch", StyleValue::Match(5)),
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


pub static JUSTIFY_CONTENT: BuiltInStyle = BuiltInStyle {
    name: "justifyContent",
    parser: Match(&[
        "flex-start", "flex-end", "center", "space-between", "space-around", "space-evenly"
    ]),
    styles: &[
        ("contentStart", StyleValue::Match(0)),
        ("contentEnd", StyleValue::Match(1)),
        ("contentCenter", StyleValue::Match(2)),
        ("contentSpaceBetween", StyleValue::Match(3)),
        ("contentSpaceAround", StyleValue::Match(4)),
        ("contentSpaceEvenly", StyleValue::Match(5)),
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