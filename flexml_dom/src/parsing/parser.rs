use super::nodes::{Node, Style};
use super::tokens::Token;
use super::tokens::Token::*;
pub(crate) use crate::parsing::warnings::ParserWarning;
use crate::parsing::warnings::ParserWarningKind::*;
use logos::{Lexer, Logos, Span};

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

pub struct Parser<'a> {
    lexer: Lexer<'a, Token>,
    pub(super) input: &'a str,
    peeked: Option<(Token, &'a str)>,
    header_parsed: bool,
    pub(super) warnings: Vec<ParserWarning>,
    max_depth: usize,
    max_nodes: usize,
    node_guard: Guard,
    depth_guard: Guard,
}

/// Parse flexml
///
/// ```
/// let input = "[bold+italic this is some text ]";
///
/// let mut parser = Parser::new(input);
///
/// while let Some(node) = parser.parse_next() {
///     println!("{:#?}", node);
/// }
/// ```
impl<'a> Parser<'a> {
    /// Create a one time use parser for parsing flexml
    pub fn new(input: &'a str) -> Self {
        let mut parser = Self {
            lexer: Token::lexer(input),
            input,
            peeked: None,
            header_parsed: false,
            warnings: Vec::new(),
            max_depth: 50,
            max_nodes: 10_000,
            node_guard: Guard::new(10_000),
            depth_guard: Guard::new(50),
        };

        if input.is_empty() {
            parser.warn(0..0, EmptyInput)
        }

        parser
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
}

impl<'a> Parser<'a> {
    /// Parse the next flexml node
    /// Call this method until None.
    pub fn parse_next(&mut self) -> Option<Node<'a>> {
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
                Whitespace | Newline => {
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
                        if let Some(warning) = self.warnings.last_mut() {
                            if matches!(warning.kind, ExceededNodeDepth) {
                                warning.span.end = self.lexer.span().end;
                            }
                        }
                        continue;
                    } else {
                        Some(self.parse_box_container())
                    };
                }

                // We don't care about starting newlines
                Newline => continue,

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
                // Newline always starts a new parse loop
                TagContainer | RawOpen | BoxContainerOpen | Newline | BoxContainerClose => {
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
    /// ```
    /// |= This is all raw text [ This is also raw ] [and this is raw ] {and so is this } \=| <- This is raw as well  =|
    /// ```
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
            Some((Named, _)) => {
                let (_, name) = self.take().unwrap(); // consume the name
                name
            }
            _ => {
                // No Named → fallback to text
                // This is valid and is not an error
                // We will consume as text
                return self.parse_text_run(start_span);
            }
        };

        // = Separator is optional
        self.skip_separator(StyleNameSeparator);

        // the + can come before { myStyle = +bold+italic}
        // mainly used for multiline styling
        self.skip_separator(StyleSeparator);

        let styles = self.parse_styles();

        if styles.is_empty() {
            self.warn(self.lexer.span(), StyleContainerNoStyles)
        }

        // Now peek for the closing '}'
        match self.peek() {
            Some((StyleContainerClose, _)) => {
                // Consume the closing tag
                self.take();
            }
            _ => self.warn(self.lexer.span(), UnclosedStyleContainer),
        }

        Node::StyleDefinition { name, styles }
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
        let styles = self.parse_styles();
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

        Node::BoxContainer { styles, children }
    }

    /// Styles always start with a named with alternating separators
    /// Styles always end on a named and consume any trailing whitespace
    /// or newlines
    fn parse_styles(&mut self) -> Vec<Style<'a>> {
        let mut styles = Vec::new();

        while let Some((tok, _)) = self.peek() {
            match tok {
                Named => {
                    let (_, name) = self.take().unwrap();
                    let mut value = None;

                    if self.skip_separator(StyleParamSeparator) {
                        if let Some((Named, arg_val)) = self.peek() {
                            value = Some(*arg_val);
                            self.take();
                        } else {
                            self.warn(self.lexer.span(), ExpectedStyleValue);
                        }
                    }

                    styles.push(Style { name, value });

                    if !self.skip_separator(StyleSeparator) {
                        break;
                    }
                }

                _ => {
                    break;
                }
            }
        }

        styles
    }
}

/// Utility type fn
impl<'a> Parser<'a> {
    /// Advances the lexer to the next valid token and returns it along with its matched input slice.
    /// Errors are ignored but sent off as warnings
    fn next_with_slice(&mut self) -> Option<(Token, &'a str)> {
        while let Some(result) = self.lexer.next() {
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
            if self.depth_guard.exceeded == false {
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
        if let Some((next, _)) = self.peek() {
            if *next == sep {
                found = true;
                self.take();
            }
        }

        self.skip_whitespace();

        found
    }

    /// Skip all whitespace and newlines
    fn skip_whitespace(&mut self) {
        while let Some((tok, _)) = self.peek() {
            match tok {
                Whitespace | Newline => {
                    self.take();
                }
                _ => break,
            }
        }
    }
}