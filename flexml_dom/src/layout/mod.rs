use crate::layout::context::{Dimension, StyleContext};
use crate::document::nodes::Node;
use crate::styles::style::AtomicStyle;
use crate::styles::style_registry::StyleRegistry;

pub mod context;

#[derive(Debug, Clone, Copy)]
pub struct LayoutBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

fn resolve_dimension(dim: Dimension, parent_size: f32) -> f32 {
    match dim {
        Dimension::Px(val) => val as f32,
        Dimension::Percent(pct) => pct * parent_size,
    }
}