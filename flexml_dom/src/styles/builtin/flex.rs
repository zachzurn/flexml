use crate::styles::builtin::BuiltInStyle;
use crate::styles::style::{StyleValue};
use crate::styles::style::StyleValueParser::{FloatParser, MatchOrFloatParser, MatchParser, NumberParser};


fn apply_align_content(_: &StyleValue) {
    todo!()
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




fn apply_flex_direction(_: &StyleValue) {
    todo!()
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



fn apply_flex_basis(_: &StyleValue) {
    todo!()
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


fn apply_flex_grow(_: &StyleValue) {
    todo!()
}

pub static FLEX_GROW: BuiltInStyle = BuiltInStyle {
    name: "flexGrow",
    parser: FloatParser,
    styles: &[
        ("grow", StyleValue::Float(1.0)),
    ],
    apply_style: apply_flex_grow,
};


fn apply_flex_shrink(_: &StyleValue) {
    todo!()
}

pub static FLEX_SHRINK: BuiltInStyle = BuiltInStyle {
    name: "flexShrink",
    parser: FloatParser,
    styles: &[
        ("shrink", StyleValue::Float(1.0)),
    ],
    apply_style: apply_flex_shrink,
};



fn apply_flex_wrap(_: &StyleValue) {
    todo!()
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


fn apply_column_gap(_: &StyleValue) {
    todo!()
}

pub static COLUMN_GAP: BuiltInStyle = BuiltInStyle {
    name: "columnGap",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_column_gap,
};


fn apply_align_items(_: &StyleValue) {
    todo!()
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


fn apply_align_self(_: &StyleValue) {
    todo!()
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


fn apply_gap(_: &StyleValue) {
    todo!()
}

pub static GAP: BuiltInStyle = BuiltInStyle {
    name: "gap",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_gap,
};


fn apply_justify_content(_: &StyleValue) {
    todo!()
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



fn apply_row_gap(_: &StyleValue) {
    todo!()
}

pub static ROW_GAP: BuiltInStyle = BuiltInStyle {
    name: "rowGap",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_row_gap,
};