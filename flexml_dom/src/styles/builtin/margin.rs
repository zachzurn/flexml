use crate::styles::builtin::BuiltInStyle;
use crate::styles::style::StyleValue;
use crate::styles::style::StyleValueParser::NumberParser;

fn apply_margin(_: &StyleValue) {
    todo!()
}

pub static MARGIN: BuiltInStyle = BuiltInStyle {
    name: "margin",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_margin,
};

fn apply_margin_top(_: &StyleValue) {
    todo!()
}

pub static MARGIN_TOP: BuiltInStyle = BuiltInStyle {
    name: "marginTop",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_margin_top,
};

fn apply_margin_right(_: &StyleValue) {
    todo!()
}

pub static MARGIN_RIGHT: BuiltInStyle = BuiltInStyle {
    name: "marginRight",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_margin_right,
};

fn apply_margin_bottom(_: &StyleValue) {
    todo!()
}

pub static MARGIN_BOTTOM: BuiltInStyle = BuiltInStyle {
    name: "marginBottom",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_margin_bottom,
};

fn apply_margin_left(_: &StyleValue) {
    todo!()
}

pub static MARGIN_LEFT: BuiltInStyle = BuiltInStyle {
    name: "marginLeft",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_margin_left,
};
