use std::path::PathBuf;
use super::nodes::{Node};
use super::tokens::Token;
use super::tokens::Token::*;
pub(crate) use crate::document::warnings::ParserWarning;
use crate::document::warnings::ParserWarningKind::*;
use logos::{Lexer, Logos, Span};
use crate::styles::context::StyleContext;
use crate::styles::style::{AtomicStyle, RawStyle, StyleId};
use crate::styles::style_registry::StyleRegistry;

pub struct FlexmlDocument<'a> {
    pub(crate) input: &'a str,
    pub(crate) style_registry: StyleRegistry,
    pub(crate) root_style: StyleContext,
    pub(crate) warnings: Vec<ParserWarning>,
    pub(crate) nodes: Vec<Node<'a>>,
    pub(crate) styles: Vec<Node<'a>>,
    pub(crate) name: String,

    base_path: Option<PathBuf>,
    parsed: bool,
    lexer: Lexer<'a, Token>,
    peeked: Option<(Token, &'a str)>,
    header_parsed: bool,
    max_depth: usize,
    max_nodes: usize,
    node_guard: Guard,
    depth_guard: Guard,
}

/// Parse flexml
impl<'a> FlexmlDocument<'a> {
    /// Create a new flexml document
    pub fn new(input: &'a str) -> Self {
        let mut parser = Self {
            input,
            style_registry: StyleRegistry::with_builtins(),
            root_style: StyleContext::default(),
            warnings: Vec::new(),
            nodes: Vec::new(),
            styles: Vec::new(),
            name: "FlexmlDocument".to_string(),
            lexer: Token::lexer(input),
            peeked: None,
            header_parsed: false,
            max_depth: 50,
            max_nodes: 10_000,
            node_guard: Guard::new(10_000),
            depth_guard: Guard::new(50),
            parsed: false,
            base_path: None,
        };

        if input.is_empty() {
            parser.warn(0..0, EmptyInput)
        }

        parser
    }

    pub fn with_base_path(mut self, path: PathBuf) -> Self {
        self.style_registry.set_base_path(&path);
        self.base_path = Some(path);
        self
    }

    pub fn with_max_depth(mut self, max_depth: usize) -> Self {
        self.max_depth = max_depth;
        self.depth_guard.limit = max_depth;
        self
    }

    pub fn with_max_nodes(mut self, max_nodes: usize) -> Self {
        self.max_nodes = max_nodes;
        self.node_guard.limit = max_nodes;
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_owned();
        self
    }

    pub fn parse(mut self) -> Self {
        if self.parsed { return self }

        while let Some(node) = self.parse_next() {
            self.nodes.push(node);
        }

        // This ensures that root styles are applied properly
        self.style_registry.resolve_root_style(&mut self.root_style);

        self.parsed = true;
        self
    }

    pub fn parse_special(mut self) -> Self {
        if self.parsed { return self }

        let mut text_group = vec![];

        while let Some(node) = self.parse_next() {
            match node {
                Node::StyleDefinition(_) => {
                    self.styles.push(node);
                },
                Node::BoxContainer {..} => {
                    if !text_group.is_empty() {
                        self.nodes.push(Node::BoxContainer {
                            styles: vec![],
                            children: std::mem::take(&mut text_group),
                        })
                    }
                    self.nodes.push(node)
                },
                Node::Text(_) => {
                    text_group.push(node);
                },
                Node::Whitespace(_) => {
                    text_group.push(node);
                },
                Node::Tag{..} => {
                    todo!()
                }
            }
        }

        if !text_group.is_empty() {
            self.nodes.push(Node::BoxContainer {
                styles: vec![],
                children: std::mem::take(&mut text_group),
            })
        }

        self.parsed = true;

        self
    }
}


struct Guard {
    limit: usize,
    count: usize,
    exceeded: bool,
}

impl Guard {
    pub fn new(limit: usize) -> Guard {
        Guard {
            limit,
            count: 0,
            exceeded: false,
        }
    }

    pub fn tick(&mut self) -> bool {
        self.count += 1;
        self.count > self.limit
    }

    pub fn reset(&mut self) {
        self.exceeded = false;
        self.count = 0;
    }
}

// Utility
impl<'a> FlexmlDocument<'a> {
    pub fn max_nodes(&self) -> usize {
        self.max_nodes
    }

    pub fn max_depth(&self) -> usize {
        self.max_depth
    }

    pub fn print_document(&self) {
        println!("┌📋 {} ────────────────────────────", self.name);

        for warning in &self.warnings {
            println!("├⚠️ {}", warning.message);
        }

        for node in &self.nodes {
            node.print_tree(&self.style_registry, "", false)
        }

        println!("└──────────────────────────── END")
    }
}

// Parser
impl<'a> FlexmlDocument<'a> {
    /// Parse the next flexml node
    /// Call this method until None.
    fn parse_next(&mut self) -> Option<Node<'a>> {
        if self.node_guard.exceeded { return None; }

        // Parse for header nodes
        if !self.header_parsed {
            let header_node = self.parse_header();

            if header_node.is_some() {
                return header_node;
            } else {
                self.header_parsed = true;
            }
        }

        // Top level container, so reset the depth_guard
        self.depth_guard.reset();

        self.parse_content()
    }

    fn parse_header(&mut self) -> Option<Node<'a>> {
        while let Some((tok, _)) = self.peek() {
            match tok {
                StyleContainerOpen => {
                    if !self.spend_node_count() { return None; }

                    self.take();
                    return Some(self.parse_style_container());
                }
                Whitespace => {
                    self.take();
                }
                _ => {
                    break;
                }
            }
        }

        None
    }

    /// Parses content, text, box containers, etc.
    /// Every time this is called, we check the depth
    ///
    fn parse_content(&mut self) -> Option<Node<'a>> {
        while let Some((tok, slice)) = self.take() {
            return match tok {
                TagContainer => {
                    //We count this style container as one node
                    if !self.spend_node_count() { return None; }

                    let name = &slice[1..slice.len() - 1];
                    Some(Node::Tag { name })
                }

                RawOpen => {
                    if !self.spend_node_count() { return None; }
                    Some(self.parse_raw())
                }

                BoxContainerOpen => {
                    if !self.spend_node_count() { return None; }

                    return if !self.spend_node_depth() {
                        // Exceeded node depth, skip box
                        self.skip_box_container();
                        if let Some(warning) = self.warnings.last_mut()
                        && matches!(warning.kind, ExceededNodeDepth) {
                                warning.span.end = self.lexer.span().end;
                        }
                        continue;
                    } else {
                        Some(self.parse_box_container())
                    };
                }

                // Anything else gets gathered as text
                // It's still possible to miss some content
                // See next_with_slice above
                _ => {
                    if !self.spend_node_count() { return None; }

                    let current_span = self.lexer.span();
                    Some(self.parse_text_run(current_span))
                }
            };
        }

        None
    }

    /// Parse a contiguous text run starting with the current span
    /// This way we can include the matched token in the text run
    /// We start the span at the provided starting_span
    /// And we set the end to the current lexer span to avoid
    /// dropping any text between the start and the loop
    fn parse_text_run(&mut self, starting_span: Span) -> Node<'a> {
        // End at the current lexer span so we don't miss any text
        let mut end = self.lexer.span().end;

        while let Some(&(tok, _)) = self.peek().as_ref() {
            match tok {
                // We collect everything that is not one of these
                TagContainer | RawOpen | BoxContainerOpen | BoxContainerClose => {
                    break;
                }
                _ => {
                    self.take();
                    let span = self.lexer.span();
                    end = span.end;
                }
            }
        }

        let s = &self.input[starting_span.start..end];
        Node::Text(s)
    }

    /// Consume raw until we find an end tag
    /// An unclosed raw tag will produce a warning.
    ///
    /// Raw Tag Example:
    ///
    /// `
    /// |= This is all raw text [ This is also raw ] [and this is raw ] {and so is this } \=| <- This is raw as well  =|
    /// `
    fn parse_raw(&mut self) -> Node<'a> {
        let start = self.lexer.span().end; // Skip over the opening tag
        let mut end = start;
        let mut end_token_found = false;

        while let Some((tok, _)) = self.take() {
            match tok {
                RawClose => {
                    end = self.lexer.span().start;
                    end_token_found = true;
                    break;
                }
                _ => {
                    end = self.lexer.span().end;
                }
            }
        }

        if !end_token_found {
            self.warn(start..end, UnclosedRawContainer);
        }

        Node::Text(&self.input[start..end])
    }

    /// Style containers have named and then whitespace or separator
    /// and then styles and then an end tag.
    ///
    /// Style containers cannot have children of any kind.
    /// Since style containers are parsed at the top
    /// of the document only, we are going to be loose
    /// with the style container and issue warnings
    fn parse_style_container(&mut self) -> Node<'a> {
        let start_span = self.lexer.span(); // Span at '{'

        // We allow strictly whitespace before the name (no newlines)
        self.skip_whitespace();

        // Style name
        let name = match self.peek() {
            Some((StyleName, _)) => {
                let (_, name) = self.take().unwrap(); // consume the name
                name
            }
            _ => {
                // No Named, fallback to text
                // This is valid and is not an error
                // We will consume as text
                return self.parse_text_run(start_span);
            }
        };

        // = Separator is optional
        self.skip_separator(StyleNameSeparator);

        // the + can come before { myStyle = +bold+italic}
        // mainly used for multiline styling.
        self.skip_separator_or_ws(StyleSeparator);

        // Contiguous styles come next, with newlines being allowed
        // as separators. This is only allowed in style definitions.
        let (styles, forwarders) = self.parse_styles(true);

        if styles.is_empty() {
            self.warn(self.lexer.span(), StyleContainerNoStyles)
        }

        // Now peek for the closing '}'
        match self.peek() {
            Some((StyleContainerClose, _)) => {
                // Consume the closing tag
                self.take();
            }
            _ => {
                self.warn(self.lexer.span(), UnclosedStyleContainer)
            }
        }

        let registered = self.style_registry.register_style(name, styles, forwarders);

        if registered.atomic {
            // Tried to register a style for an atomic style definition
            self.warn(start_span.start..start_span.end, AtomicStyleDefinition);
        }

        if registered.overwrote && !registered.builtin {
            // Overwrote an already defined style (overwriting builtins is not a warning)
            self.warn(start_span.start..start_span.end, OverwroteStyleDefinition);
        }

        Node::StyleDefinition(registered.style_id)
    }

    /// Skip contents of a box container
    /// Useful for ignoring deep containers
    /// While allowing the parser to continue
    fn skip_box_container(&mut self) {
        let mut open_boxes = 1;

        while let Some((tok, _)) = self.take() {
            match tok {
                BoxContainerOpen => {
                    open_boxes += 1;
                }
                BoxContainerClose => {
                    open_boxes -= 1;
                    if open_boxes == 0 {
                        break;
                    }
                }
                _ => {
                    // Consumed
                }
            }
        }

        if open_boxes != 0 {
            self.warn(self.lexer.span(), UnclosedBoxContainer)
        }
    }

    /// Box containers can optionally have styles
    /// Box containers always have whitespace or newlines to start the children
    /// content parsing
    fn parse_box_container(&mut self) -> Node<'a> {
        // We ignore forwarders on inline styles
        // Forwarders only apply to style definitions
        let (styles, _forwarders) = self.parse_styles(false);
        let mut children = Vec::new();
        let mut close_found = false;

        while !self.node_guard.exceeded {
            if let Some((tok, _)) = self.peek() {
                match tok {
                    BoxContainerClose => {
                        close_found = true;
                        self.take();
                        break;
                    }
                    _ => {
                        if let Some(child) = self.parse_content() {
                            children.push(child);
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        // We are gracefully stopping parsing and don't
        // want to issue warnings that we haven't checked
        if self.node_guard.exceeded {
            close_found = true;
        }

        //Trim trailing text
        if let Some(Node::Text(last)) = children.last_mut() {
           *last = last.trim_end();
        }

        if !close_found {
            self.warn(self.lexer.span(), UnclosedBoxContainer);
        }

        Node::BoxContainer {styles, children }
    }

    /// Styles always start with a named with alternating separators
    /// Styles always end on a named and consume any trailing whitespace
    /// or newlines
    fn parse_styles(&mut self, allow_newline_separator: bool) -> (Vec<AtomicStyle>, Vec<StyleId>) {
        let mut styles = Vec::new();

        while let Some((tok, _)) = self.peek() {
            match tok {
                StyleName => {
                    let (_, name) = self.take().unwrap();
                    let mut value = None;

                    if let Some((StyleValue, arg_val)) = self.peek() {
                        // Style values come in with separators and possibly quoted
                        value = Some(arg_val
                            .trim_start_matches([':', ' ', '\t', '"'])
                            .trim_end_matches('"')
                        );
                        self.take();
                    }

                    styles.push(RawStyle { name, value });

                    if allow_newline_separator {
                        if !self.skip_separator_or_ws(StyleSeparator) {
                            break;
                        }
                    } else if !self.skip_separator(StyleSeparator) {
                        break;
                    }
                }

                _ => {
                    break;
                }
            }
        }

        self.style_registry.expand_raw_styles(&styles)
    }
}

/// Parsing Utilities
impl<'a> FlexmlDocument<'a> {
    /// Advances the lexer to the next valid token and returns it along with its matched input slice.
    /// Errors are ignored but sent off as warnings
    fn next_with_slice(&mut self) -> Option<(Token, &'a str)> {
        if let Some(result) = self.lexer.next() {
            return match result {
                Ok(tok) => {
                    let slice = self.lexer.slice();
                    Some((tok, slice))
                }
                Err(()) => {
                    let span = self.lexer.span();
                    let slice = &self.input[span.start..span.end];

                    self.warn(self.lexer.span(), UnexpectedToken);

                    // Errors should not happen but if they do
                    // we still forward it to a Text token to be
                    // collected as text
                    Some((Text, slice))
                }
            };
        }
        None
    }

    /// Consume the current or peeked token
    fn take(&mut self) -> Option<(Token, &'a str)> {
        if let Some(p) = self.peeked.take() {
            Some(p)
        } else {
            self.next_with_slice()
        }
    }

    /// Take a peek at the next token and hold onto it.
    /// We always have to hold onto consumed tokens.
    fn peek(&mut self) -> Option<&(Token, &'a str)> {
        if self.peeked.is_none() {
            self.peeked = self.next_with_slice();
        }
        self.peeked.as_ref()
    }

    /// Increment and check that we have not exceeded node count
    /// Returns false if we cannot afford any more nodes
    /// issues a single warning even on multiple guard failures
    /// unless the guard is reset
    fn spend_node_count(&mut self) -> bool {
        // We have produced too many nodes if tick is true
        if self.node_guard.tick() {

            // Fire off a single warning
            if !self.node_guard.exceeded {
                self.node_guard.exceeded = true;
                self.warn(self.lexer.span().start..self.input.len(), ExceededNodeCount);
            }
            false
        } else {
            true
        }
    }

    /// Increment and check that we have not exceeded node depth
    /// Returns true if we have exceeded the depth and also
    /// issues a single warning even on multiple guard failures
    /// unless the guard is reset
    fn spend_node_depth(&mut self) -> bool {
        if self.depth_guard.tick() {

            // Fire off a single warning
            if !self.depth_guard.exceeded {
                self.depth_guard.exceeded = true;
                self.warn(self.lexer.span(), ExceededNodeDepth);
            }

            false
        } else {
            true
        }
    }

    // Separators can be surrounded by whitespace
    // Sometimes we need to know if a separator was found
    fn skip_separator(&mut self, sep: Token) -> bool {
        self.skip_whitespace();
        let mut found = false;

        // peek for sep token and skip if found
        if let Some((next, _)) = self.peek() && *next == sep {
            found = true;
            self.take();
        }

        self.skip_whitespace();

        found
    }

    fn skip_separator_or_ws(&mut self, sep: Token) -> bool {
        let mut found = false;

        while let Some((tok, _)) = self.peek() {
            match tok {
                Whitespace => {
                    self.take();
                    found = true;
                }
                _ => break,
            }
        }

        // peek for sep token and skip if found
        if let Some((next, _)) = self.peek() && *next == sep {
            found = true;
            self.take();
        }

        self.skip_whitespace();

        found
    }

    /// Skip all whitespace and newlines
    fn skip_whitespace(&mut self) {
        while let Some((tok, _)) = self.peek() {
            match tok {
                Whitespace => {
                    self.take();
                }
                _ => break,
            }
        }
    }
}