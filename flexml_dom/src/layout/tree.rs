use taffy::util::print_tree;
use taffy::{compute_block_layout, compute_cached_layout, compute_flexbox_layout, compute_leaf_layout, compute_root_layout, prelude::*, round_layout, Cache, CacheTree, LayoutOutput};
use crate::layout::FlexmlLayoutContext;
use crate::layout::inline::{compute_inline_layout};
use crate::layout::taffy_style::style_context_to_taffy;
use crate::styles::context::StyleContext;

#[derive(Debug, Copy, Clone)]
/// Content is flex and block containers.
/// Content should never have direct Text.
///
/// InlineContent is a leaf node that measures
/// and lays out its contents.
pub(super) enum LayoutNodeKind {
    Container,
    InlineContent, // Inline content
    Text //Pure text
}

pub(super)  struct LayoutNode {
    pub(super) kind: LayoutNodeKind,
    pub(super) style: Style,
    pub(super) style_context: StyleContext,
    pub(super) children: Vec<usize>,
    pub(super) text: Option<String>,
    pub(super) inline_layout: Option<parley::Layout<[u8; 4]>>,
    pub(crate) cache: Cache,
    pub(crate) unrounded_layout: Layout,
    pub(crate) final_layout: Layout,
}

impl LayoutNode {
    pub (super) fn new_container(kind: LayoutNodeKind, style_context: StyleContext, children: Vec<usize>) -> Self {
        Self {
            kind,
            style: style_context_to_taffy(&style_context),
            style_context,
            children,
            text: None,
            inline_layout: None,
            cache: Default::default(),
            unrounded_layout: Default::default(),
            final_layout: Default::default(),
        }
    }

    pub (super) fn new_text(style_context: StyleContext, text: String) -> Self {
        Self {
            kind: LayoutNodeKind::Text,
            style: style_context_to_taffy(&style_context),
            style_context,
            children: vec![],
            text: Some(text),
            inline_layout: None,
            cache: Default::default(),
            unrounded_layout: Default::default(),
            final_layout: Default::default(),
        }
    }

}

pub(super) struct LayoutTree {
    nodes: Vec<LayoutNode>,
    pub(super) context: FlexmlLayoutContext,
}

impl LayoutTree {
    pub fn new(context: FlexmlLayoutContext) -> Self {
        Self {
            nodes: Vec::new(),
            context
        }
    }

    pub fn add_node(&mut self, node: LayoutNode) -> usize {
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    pub fn append_child(&mut self, parent: usize, child: usize) {
        self.nodes[parent].children.push(child);
    }

    #[inline(always)]
    pub(super) fn node_from_id(&self, node_id: NodeId) -> &LayoutNode {
        &self.nodes[usize::from(node_id)]
    }

    #[inline(always)]
    pub(super) fn node_from_id_mut(&mut self, node_id: NodeId) -> &mut LayoutNode {
        &mut self.nodes[usize::from(node_id)]
    }

    pub fn compute_layout(&mut self, root: usize, available_space: Size<AvailableSpace>, use_rounding: bool) {
        compute_root_layout(self, NodeId::from(root), available_space);
        if use_rounding {
            round_layout(self, NodeId::from(root))
        }
    }

    pub fn print_tree(&mut self, root: usize) {
        print_tree(self, NodeId::from(root));
    }
}

pub struct ChildIter<'a>(std::slice::Iter<'a, usize>);
impl Iterator for ChildIter<'_> {
    type Item = NodeId;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().copied().map(NodeId::from)
    }
}

impl TraversePartialTree for LayoutTree {
    type ChildIter<'a> = ChildIter<'a>;

    fn child_ids(&self, node_id: NodeId) -> Self::ChildIter<'_> {
        ChildIter(self.node_from_id(node_id).children.iter())
    }

    fn child_count(&self, node_id: NodeId) -> usize {
        self.node_from_id(node_id).children.len()
    }

    fn get_child_id(&self, node_id: NodeId, index: usize) -> NodeId {
        NodeId::from(self.node_from_id(node_id).children[index])
    }
}

impl LayoutPartialTree for LayoutTree {
    type CoreContainerStyle<'a>
    = &'a Style
    where
        Self: 'a;

    fn get_core_container_style(&self, node_id: NodeId) -> Self::CoreContainerStyle<'_> {
        &self.node_from_id(node_id).style
    }

    fn resolve_calc_value(&self, _val: *const (), _basis: f32) -> f32 {
        0.0
    }

    fn set_unrounded_layout(&mut self, node_id: NodeId, layout: &Layout) {
        self.node_from_id_mut(node_id).unrounded_layout = *layout;
    }

    fn compute_child_layout(&mut self, node_id: NodeId, inputs: taffy::tree::LayoutInput) -> LayoutOutput {
        compute_cached_layout(self, node_id, inputs, |tree, node_id, inputs| {
            let node = &mut tree.nodes[usize::from(node_id)];

            match node.kind {
                LayoutNodeKind::Container => {
                    match node.style.display {
                        Display::Block => compute_block_layout(tree, node_id, inputs),
                        Display::Flex => compute_flexbox_layout(tree, node_id, inputs),
                        _ => LayoutOutput::HIDDEN
                    }
                },

                LayoutNodeKind::InlineContent => {
                    compute_inline_layout(tree, node_id, inputs)
                }

                // Text should not appear outside InlineContent
                LayoutNodeKind::Text => {
                    LayoutOutput::from_outer_size(Size::ZERO)
                }
            }
        })
    }
}

impl CacheTree for LayoutTree {
    fn cache_get(
        &self,
        node_id: NodeId,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        run_mode: taffy::RunMode,
    ) -> Option<LayoutOutput> {
        self.node_from_id(node_id).cache.get(known_dimensions, available_space, run_mode)
    }

    fn cache_store(
        &mut self,
        node_id: NodeId,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        run_mode: taffy::RunMode,
        layout_output: LayoutOutput,
    ) {
        self.node_from_id_mut(node_id).cache.store(known_dimensions, available_space, run_mode, layout_output)
    }

    fn cache_clear(&mut self, node_id: NodeId) {
        self.node_from_id_mut(node_id).cache.clear();
    }
}

impl taffy::LayoutFlexboxContainer for LayoutTree {
    type FlexboxContainerStyle<'a>
    = &'a Style
    where
        Self: 'a;

    type FlexboxItemStyle<'a>
    = &'a Style
    where
        Self: 'a;

    fn get_flexbox_container_style(&self, node_id: NodeId) -> Self::FlexboxContainerStyle<'_> {
        &self.node_from_id(node_id).style
    }

    fn get_flexbox_child_style(&self, child_node_id: NodeId) -> Self::FlexboxItemStyle<'_> {
        &self.node_from_id(child_node_id).style
    }
}

impl taffy::LayoutBlockContainer for LayoutTree {
    type BlockContainerStyle<'a>
    = &'a Style
    where
        Self: 'a;

    type BlockItemStyle<'a>
    = &'a Style
    where
        Self: 'a;

    fn get_block_container_style(&self, node_id: NodeId) -> Self::BlockContainerStyle<'_> {
        &self.node_from_id(node_id).style
    }

    fn get_block_child_style(&self, child_node_id: NodeId) -> Self::BlockItemStyle<'_> {
        &self.node_from_id(child_node_id).style
    }
}

impl TraverseTree for LayoutTree {}

impl RoundTree for LayoutTree {
    fn get_unrounded_layout(&self, node_id: NodeId) -> &Layout {
        &self.node_from_id(node_id).unrounded_layout
    }

    fn set_final_layout(&mut self, node_id: NodeId, layout: &Layout) {
        self.node_from_id_mut(node_id).final_layout = *layout;
    }
}

impl PrintTree for LayoutTree {
    fn get_debug_label(&self, node_id: NodeId) -> &'static str {
        let node = self.node_from_id(node_id);
        match node.kind {
            LayoutNodeKind::Container => "Container",
            LayoutNodeKind::InlineContent => "Inline Content",
            LayoutNodeKind::Text => "Text",
        }
    }

    fn get_final_layout(&self, node_id: NodeId) -> &Layout {
        &self.node_from_id(node_id).final_layout
    }
}