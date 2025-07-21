use crate::styles::builtin::BuiltInStyle;
use crate::styles::style::StyleValue;
use crate::styles::style::StyleValueParser::NumberParser;

fn apply_padding(_: &StyleValue) {
    todo!()
}

pub static PADDING: BuiltInStyle = BuiltInStyle {
    name: "padding",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_padding,
};

fn apply_padding_top(_: &StyleValue) {
    todo!()
}

pub static PADDING_TOP: BuiltInStyle = BuiltInStyle {
    name: "paddingTop",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_padding_top,
};

fn apply_padding_right(_: &StyleValue) {
    todo!()
}

pub static PADDING_RIGHT: BuiltInStyle = BuiltInStyle {
    name: "paddingRight",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_padding_right,
};

fn apply_padding_bottom(_: &StyleValue) {
    todo!()
}

pub static PADDING_BOTTOM: BuiltInStyle = BuiltInStyle {
    name: "paddingBottom",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_padding_bottom,
};

fn apply_padding_left(_: &StyleValue) {
    todo!()
}

pub static PADDING_LEFT: BuiltInStyle = BuiltInStyle {
    name: "paddingLeft",
    parser: NumberParser,
    styles: &[],
    apply_style: apply_padding_left,
};
