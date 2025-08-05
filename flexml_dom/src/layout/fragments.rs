use parley::{Font, Glyph, PositionedLayoutItem, Style};
use parley::swash::{NormalizedCoord, Synthesis};
use crate::styles::context::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self{ x, y, width, height }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Radius {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_left: f32,
    pub bottom_right: f32,
}

impl Radius {
    pub fn new(top_left: f32, top_right: f32, bottom_left: f32, bottom_right: f32) -> Self {
        Self { top_left, top_right, bottom_left, bottom_right }
    }
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}


#[derive(Debug)]
pub struct GlyphRunFragment {
    pub glyphs: Vec<Glyph>,
    pub baseline: f32,
    pub offset: f32,
    pub advance: f32,
    pub synthesis: Synthesis,
    pub font_size: f32,
    pub font: Font,
    pub normalized_coords: Vec<NormalizedCoord>,
    pub style: Style<[u8; 4]>,
}

#[derive(Debug)]
pub enum FragmentKind {
    Text(GlyphRunFragment),
    ColorBackground{color: Color, radius: Radius},
    ColorBorder{radius: Radius, color: Color, weight: f32},
    Debug
}

#[derive(Debug)]
pub struct Fragment {
    pub bounds: Rect,
    pub kind: FragmentKind
}

impl Fragment {
    pub fn bg(bounds: Rect, radius: Radius, color: Color) -> Self {
        Self {
            bounds,
            kind: FragmentKind::ColorBackground {color, radius },
        }
    }

    pub fn border(bounds: Rect, radius: Radius, color: Color, weight: f32) -> Self {
        Self {
            bounds,
            kind: FragmentKind::ColorBorder {
                radius, color, weight,
            },
        }
    }
}

#[derive(Debug)]
pub struct FragmentGroup {
    pub bounds: Rect,
    pub fragments: Vec<Fragment>,
    pub subgroups: Vec<FragmentGroup>,
    pub splittable: bool,
}

impl FragmentGroup {
    pub(crate) fn new(bounds: Rect) -> Self {
        Self {
            bounds,
            fragments: vec![],
            subgroups: vec![],
            splittable: true,
        }
    }
}



use taffy::NodeId;
use crate::layout::tree::{LayoutNode, LayoutNodeKind, LayoutTree};

impl FragmentGroup {
    pub fn print_tree(&self, name: &str) {
        println!("\n{}:", name);
        self.print_tree_ascii("", false)
    }

    fn print_tree_ascii(&self, indent: &str, last: bool) {
        // Print branch prefix
        let branch = if last { "└── " } else { "├── " };
        println!("{}{}Group: (x={}, y={}, w={}, h={})", indent, branch, self.bounds.x, self.bounds.y, self.bounds.width, self.bounds.height);

        let child_indent = if last {
            format!("{}    ", indent)
        } else {
            format!("{}│   ", indent)
        };

        // Print fragments
        let frag_len = self.fragments.len();
        for (i, fragment) in self.fragments.iter().enumerate() {
            let last_frag = i == frag_len - 1 && self.subgroups.is_empty();
            let frag_branch = if last_frag { "└── " } else { "├── " };

            match &fragment.kind {
                FragmentKind::Text(glyph_run) => {
                    println!("{}{}Text: {}:{}:{}:{} ({:?} color, {:?} glyphs, advance {:.2}, size {:.1}, baseline {:.1})",
                             child_indent,
                             frag_branch,
                             fragment.bounds.x,
                             fragment.bounds.y,
                             fragment.bounds.width,
                             fragment.bounds.height,
                             glyph_run.style.brush,
                             glyph_run.glyphs.len(),
                             glyph_run.advance,
                             glyph_run.font_size,
                             glyph_run.baseline,
                    );
                }
                FragmentKind::ColorBackground { color, .. } => {
                    println!("{}{}Background Color: {:?}", child_indent, frag_branch, color);
                }
                FragmentKind::ColorBorder { color, weight, .. } => {
                    println!("{}{}Border Color: {:?}, weight: {:.1}", child_indent, frag_branch, color, weight);
                }
                FragmentKind::Debug => {
                    println!("Debug");
                }
            }
        }

        // Print subgroups recursively
        let group_len = self.subgroups.len();
        for (i, subgroup) in self.subgroups.iter().enumerate() {
            let last_group = i == group_len - 1;
            subgroup.print_tree_ascii(&child_indent, last_group);
        }
    }
}



fn container_style_fragments(container: &LayoutNode, offset_x: f32, offset_y: f32) -> Option<Vec<Fragment>> {
    let em = container.style_context.resolved_font_size();
    let rem = container.style_context.resolved_root_font_size();
    let dpi = container.style_context.dpi();

    let has_bg = container.style_context.bg_color().3 != 0;
    let has_border = container.style_context.border_color().3 != 0;

    //No fragments to render
    if !has_bg && !has_border { return None }

    let mut fragments = vec![];
    let layout = &container.final_layout;
    let style = &container.style_context;


    let bounds = Rect::new(
        offset_x + layout.location.x,
        offset_y + layout.location.y,
        layout.size.width,
        layout.size.height,
    );

    let radius = Radius::new(
        style.border_top_left_radius().to_pixels(layout.size.width, rem, em, dpi),
        style.border_top_right_radius().to_pixels(layout.size.width, rem, em, dpi),
        style.border_bottom_left_radius().to_pixels(layout.size.width, rem, em, dpi),
        style.border_bottom_right_radius().to_pixels(layout.size.width, rem, em, dpi)
    );

    if has_bg {
        fragments.push(Fragment::bg(bounds, radius, style.bg_color()))
    }

    if has_border {
        let border_weight = style.border_width().to_pixels(layout.size.width, rem, em, dpi);
        fragments.push(Fragment::border(bounds, radius, style.border_color(), border_weight))
    }

    Some(fragments)
}


pub(super) fn collect_fragments(
    tree: &LayoutTree,
    node_id: NodeId,
    offset_x: f32,
    offset_y: f32,
    out: &mut Vec<FragmentGroup>,
) {
    let node = tree.node_from_id(node_id);
    let node_children = node.children.clone();

    if matches!(node.kind, LayoutNodeKind::InlineContent) {
        if let Some(inline_layout) = &node.inline_layout {
            for line in inline_layout.lines() {
                let line_metrics = line.metrics();
                for item in line.items() {
                    match item {
                        PositionedLayoutItem::GlyphRun(glyph_run) => {
                            let x = glyph_run.offset();
                            let y = line_metrics.offset + line_metrics.baseline - glyph_run.baseline();

                            let fragment = Fragment {
                                bounds: Rect {
                                    x: offset_x + x,
                                    y: offset_y + y,
                                    width: glyph_run.advance(),
                                    height: line_metrics.line_height,
                                },
                                kind: FragmentKind::Text(GlyphRunFragment {
                                    glyphs: glyph_run.glyphs().collect(),
                                    baseline: glyph_run.baseline(),
                                    offset: glyph_run.offset(),
                                    advance: glyph_run.advance(),
                                    synthesis: glyph_run.run().synthesis(),
                                    font_size: glyph_run.run().font_size(),
                                    font: glyph_run.run().font().clone(),
                                    normalized_coords: glyph_run.run().normalized_coords().to_vec(),
                                    style: glyph_run.style().clone(),
                                }),
                            };

                            out.push(FragmentGroup {
                                bounds: fragment.bounds,
                                fragments: vec![fragment],
                                subgroups: vec![],
                                splittable: false,
                            });
                        }

                        PositionedLayoutItem::InlineBox(inline_box) => {
                            // Shift from top aligned to baseline aligned
                            // needs testing and should use the vertical align style property maybe
                            let baseline_y = inline_box.height - (line_metrics.max_coord + inline_box.y);
                            collect_fragments(tree, NodeId::from(inline_box.id), offset_x + inline_box.x, offset_y + baseline_y, out);
                        }
                    }
                }
            }
        }

        return;
    }

    let mut group = FragmentGroup::new(Rect::new(
        node.final_layout.location.x + offset_x,
        node.final_layout.location.y + offset_y,
        node.final_layout.size.width,
        node.final_layout.size.height,
    ));

    if  let LayoutNodeKind::Container = node.kind
        && let Some(style_fragments) = container_style_fragments(node, offset_x, offset_y)
    {
        group.fragments.extend(style_fragments);
    }

    for child_id in node_children {
        collect_fragments(tree, child_id, offset_x, offset_y, &mut group.subgroups);
    }

    if !group.fragments.is_empty() || !group.subgroups.is_empty() {
        out.push(group);
    }
}
