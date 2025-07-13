use std::ops::Range;
use logos::{Logos, Lexer, Span};
use super::nodes::{Node, Style};
use super::tokens::Token;
use super::tokens::Token::*;


#[derive(Debug)]
pub struct ParserError {
    pub message: String,
    pub span: Range<usize>,
    pub label: String,
    pub help: String,
}

pub struct Parser<'a> {
    lexer: Lexer<'a, Token>,
    input: &'a str,
    peeked: Option<(Token, &'a str)>,
    styles_parsed: bool,
    pub errors: Vec<ParserError>,
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
        Self {
            lexer: Token::lexer(input),
            input,
            peeked: None,
            styles_parsed: false,
            errors: Vec::new(),
        }
    }

    /// Records a parser error.
    /// - `span`: The input range that caused the error.
    /// - `message`: A short error description.
    /// - `label`: A label for what the span highlights.
    /// - `help`: A suggestion for fixing the issue.
    fn warn(&mut self, span: Range<usize>, message: &str, label: &str, help: &str) {
        self.errors.push(ParserError {
            span,
            message: message.to_owned(),
            label: label.to_owned(),
            help: help.to_owned(),
        });
    }

    /// Advances the lexer to the next valid token and returns it along with its matched input slice.
    /// Errors are ignored but sent off as warnings
    fn next_with_slice(&mut self) -> Option<(Token, &'a str)> {
        while let Some(result) = self.lexer.next() {
            match result {
                Ok(tok) => {
                    let slice = self.lexer.slice();
                    return Some((tok, slice));
                }
                Err(()) => {
                    let span = self.lexer.span();
                    let slice = &self.input[span.start..span.end];

                    self.warn(
                        self.lexer.span(),
                        "There was an error while parsing a token",
                        &format!("unexpected token or text {}", slice),
                        "This should not be a problem but please file an issue",
                    );

                    // Errors should not happen but if they do
                    // we still forward it to a Text token to be
                    // collected as text
                    return Some((Token::Text, slice));
                }
            }
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

    /// Parse the next flexml node
    /// Call this method until None.
    pub fn parse_next(&mut self) -> Option<Node<'a>> {

        //Parse styles before anything else
        if !self.styles_parsed {
            while let Some((tok, _)) = self.peek() {
                match tok {
                    StyleContainerOpen => {
                        self.take();
                        return Some(self.parse_style_container());
                    }
                    Whitespace | Newline  => {
                        self.take();
                    },
                    _ => {
                        self.styles_parsed = true;
                        break;
                    },
                }
            }
        }

        while let Some((tok, slice)) = self.take() {
            return match tok {
                TagContainer => {
                    let name = &slice[1..slice.len() - 1];
                    Some(Node::Tag { name })
                }

                RawOpen => {
                    Some(self.parse_raw())
                }

                BoxContainerOpen => {
                    Some(self.parse_box_container())
                }

                // We don't care about starting newlines
                Newline => {
                    continue
                }

                // Anything else gets gathered as text
                // It's still possible to miss some content
                // See next_with_slice above
                _ => {
                    let current_span = self.lexer.span();
                    Some(self.parse_text_run(current_span))
                },
            }
        }

        None
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
                TagContainer | RawOpen | BoxContainerOpen | Newline | BoxContainerClose =>  {
                    //TODO maybe on box container close, we should ignore previous whitespace
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
            self.warn(
                start..end,
                "Unterminated raw container",
                "Raw ended here with no closing tag",
                "Try adding =| to close the raw container",
            );
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

        // We allow whitespace before the name
        self.skip_whitespace();

        // Style name
        let name = match self.peek() {
            Some((Token::Named, _)) => {
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
            self.warn(
                self.lexer.span(),
                "Style definition has no styles",
                "Missing styles",
                "Add some styles like bold+italic"
            )
        }

        // Now peek for the closing '}'
        match self.peek() {
            Some((Token::StyleContainerClose, _)) => {
                // Consume the closing tag
                self.take();
            }
            _ => {
                self.warn(
                    self.lexer.span(),
                    "Unclosed style container",
                    "Missing closing '}'",
                    "Add '}' to close the style container",
                );
            }
        }

        Node::StyleDefinition { name, styles }
    }

    /// Box containers can optionally have styles
    /// Box containers always have whitespace or newlines to start the children
    /// content parsing
    fn parse_box_container(&mut self) -> Node<'a> {
        let styles = self.parse_styles();
        let mut children = Vec::new();

        while let Some((tok, _)) = self.peek() {
            match tok {
                Token::BoxContainerClose => {
                    self.take();
                    break;
                }
                _ => {
                    if let Some(child) = self.parse_next() {
                        children.push(child);
                    } else {
                        break;
                    }
                }
            }
        }

        //Trim trailing text
        if let Some(Node::Text(last)) = children.last_mut() {
            *last = last.trim_end();
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
                            self.warn(
                                self.lexer.span(),
                                "Expected style value, but found nothing",
                                "Missing style value",
                                "Try removing the : or adding a value",
                            );
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