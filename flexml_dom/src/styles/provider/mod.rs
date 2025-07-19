use crate::styles::style::{AtomicStyle, StyleId, StyleValue, StyleValueParser};
use crate::styles::style_registry::StyleRegistry;

mod font_weight;
mod display;
mod border_style;
mod flex_direction;
mod flex_wrap;
mod justify_content;
mod align_items;
mod align_content;
mod align_self;
mod font_style;
mod text_align;
mod text_decoration;
mod text_transform;
mod white_space;
mod bg_repeat;
mod bg_position;
mod bg_size;
mod width;
mod height;
mod min_width;
mod max_width;
mod min_height;
mod max_height;
mod margin;
mod margin_top;
mod margin_bottom;
mod margin_left;
mod margin_right;
mod padding;
mod padding_top;
mod padding_bottom;
mod padding_left;
mod padding_right;
mod border_width;
mod border_radius;
mod border_top_left_radius;
mod border_top_right_radius;
mod border_bottom_left_radius;
mod border_bottom_right_radius;
mod gap;
mod row_gap;
mod column_gap;
mod font_size;
mod line_height;
mod letter_spacing;
mod word_spacing;
mod text_color;
mod bg_color;
mod border_color;
mod font;
mod bg_image;
mod order;
mod flex_grow;
mod flex_shrink;
mod flex_basis;
mod color;
mod opacity;

trait AtomicStyleProvider {

    /// Return an atomic style name
    /// for example "fontWeight"
    fn name(&self) -> &'static str;

    /// Return an atomic style parser
    /// for example a MatchParser(["bold","regular",..])
    fn parser(&self) -> StyleValueParser;

    /// Return a list of alias styles, name and value
    /// for example "bold" with StyleValue::Match(0)
    fn builtins(&self) -> &'static [(&'static str, StyleValue)];

    /// Not implemented yet. Apply style to a context
    /// for example, context.fontWeight = FontWeight::Bold
    fn apply(&self, style: &StyleValue);


}