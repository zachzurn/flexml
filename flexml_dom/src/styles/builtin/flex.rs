use crate::styles::context::{AlignContent, AlignItems, AlignSelf, FlexDirection, FlexWrap, JustifyContent, Length, StyleContext};
use crate::styles::builtin::{apply_dimension, apply_float, apply_length, apply_match_style, BuiltInStyle};
use crate::styles::style::{StyleValue};
use crate::styles::style::StyleValueParser::{FloatParser, MatchOrFloatParser, MatchParser, NumberParser};


fn apply_align_content(value: &StyleValue, context: &mut StyleContext) {
    apply_match_style(value, &mut context.align_content, &[
        AlignContent::FlexStart,
        AlignContent::FlexEnd,
        AlignContent::Center,
        AlignContent::SpaceBetween,
        AlignContent::SpaceAround,
        AlignContent::Stretch,
    ])
}

pub static ALIGN_CONTENT: BuiltInStyle = BuiltInStyle {
    name: "alignContent",
    parser: MatchParser(&[
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




fn apply_flex_direction(value: &StyleValue, context: &mut StyleContext) {
    apply_match_style(value, &mut context.flex_direction, &[
        FlexDirection::Row,
        FlexDirection::RowReverse,
        FlexDirection::Column,
        FlexDirection::ColumnReverse,
    ])
}

pub static FLEX_DIRECTION: BuiltInStyle = BuiltInStyle {
    name: "flexDirection",
    parser: MatchParser(&["row", "row-reverse", "column", "column-reverse"]),
    styles: &[
        ("row", StyleValue::Match(0)),
        ("col", StyleValue::Match(2)),
        ("column", StyleValue::Match(2)),
    ],
    apply_style: apply_flex_direction,
};



fn apply_flex_basis(value: &StyleValue, context: &mut StyleContext) {
    apply_length(value, &mut context.flex_basis, &[
        Length::Auto,
        Length::Content
    ])
}

pub static FLEX_BASIS: BuiltInStyle = BuiltInStyle {
    name: "flexBasis",
    parser: MatchOrFloatParser(&["auto", "content"]),
    styles: &[
        ("basisAuto", StyleValue::Match(0)),
        ("basisContent", StyleValue::Match(1)),
    ],
    apply_style: apply_flex_basis,
};


fn apply_flex_grow(value: &StyleValue, context: &mut StyleContext) {
    apply_float(value, &mut context.flex_grow);
}

pub static FLEX_GROW: BuiltInStyle = BuiltInStyle {
    name: "flexGrow",
    parser: FloatParser,
    styles: &[
        ("grow", StyleValue::Float(1.0)),
    ],
    apply_style: apply_flex_grow,
};


fn apply_flex_shrink(value: &StyleValue, context: &mut StyleContext) {
    apply_float(value, &mut context.flex_grow);
}

pub static FLEX_SHRINK: BuiltInStyle = BuiltInStyle {
    name: "flexShrink",
    parser: FloatParser,
    styles: &[
        ("shrink", StyleValue::Float(1.0)),
    ],
    apply_style: apply_flex_shrink,
};



fn apply_flex_wrap(value: &StyleValue, context: &mut StyleContext) {
    apply_match_style(value, &mut context.flex_wrap, &[
        FlexWrap::NoWrap,
        FlexWrap::Wrap,
        FlexWrap::WrapReverse
    ]);
}

pub static FLEX_WRAP: BuiltInStyle = BuiltInStyle {
    name: "flexWrap",
    parser: MatchParser(&["nowrap", "wrap", "wrap-reverse"]),
    styles: &[
        ("noWrap", StyleValue::Match(0)),
        ("wrapReverse", StyleValue::Match(2)),
    ],
    apply_style: apply_flex_wrap,
};


fn apply_column_gap(value: &StyleValue, context: &mut StyleContext) {
    apply_dimension(value, &mut context.column_gap);
}

pub static COLUMN_GAP: BuiltInStyle = BuiltInStyle {
    name: "columnGap",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_column_gap,
};


fn apply_align_items(value: &StyleValue, context: &mut StyleContext) {
    apply_match_style(value, &mut context.align_items, &[
        AlignItems::FlexStart,
        AlignItems::FlexEnd,
        AlignItems::Center,
        AlignItems::Baseline,
        AlignItems::Stretch,
    ])
}

pub static ALIGN_ITEMS: BuiltInStyle = BuiltInStyle {
    name: "alignItems",
    parser: MatchParser(&["flex-start", "flex-end", "center", "baseline", "stretch"]),
    styles: &[
        ("itemsStart", StyleValue::Match(0)),
        ("itemsCenter", StyleValue::Match(2)),
    ],
    apply_style: apply_align_items,
};


fn apply_align_self(value: &StyleValue, context: &mut StyleContext) {
    apply_match_style(value, &mut context.align_self, &[
        AlignSelf::Auto,
        AlignSelf::FlexStart,
        AlignSelf::FlexEnd,
        AlignSelf::Center,
        AlignSelf::Baseline,
        AlignSelf::Stretch,
    ])
}

pub static ALIGN_SELF: BuiltInStyle = BuiltInStyle {
    name: "alignSelf",
    parser: MatchParser(&["auto", "flex-start", "flex-end", "center", "baseline", "stretch"]),
    styles: &[
        ("selfStart", StyleValue::Match(1)),
        ("selfStretch", StyleValue::Match(5)),
    ],
    apply_style: apply_align_self,
};


fn apply_gap(value: &StyleValue, context: &mut StyleContext) {
    apply_dimension(value, &mut context.gap);
}

pub static GAP: BuiltInStyle = BuiltInStyle {
    name: "gap",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_gap,
};


fn apply_justify_content(value: &StyleValue, context: &mut StyleContext) {
    apply_match_style(value, &mut context.justify_content, &[
        JustifyContent::FlexStart,
        JustifyContent::FlexEnd,
        JustifyContent::Center,
        JustifyContent::SpaceBetween,
        JustifyContent::SpaceAround,
        JustifyContent::SpaceEvenly
    ])
}

pub static JUSTIFY_CONTENT: BuiltInStyle = BuiltInStyle {
    name: "justifyContent",
    parser: MatchParser(&[
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
    apply_dimension(value, &mut context.row_gap);
}

pub static ROW_GAP: BuiltInStyle = BuiltInStyle {
    name: "rowGap",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_row_gap,
};