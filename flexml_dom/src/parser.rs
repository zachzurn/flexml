mod json;
mod util;
mod macro_parser;

use logos::{Logos};
use crate::document::*;
use util::ElementTreeBuilder;

#[derive(Debug, PartialEq, Logos)]
enum Token<'source> {
    #[regex(r"([ \t]+)?\/\/.*\n?", priority = 200)]
    Comment,

    #[regex(r"[ \t]{2,}", priority = 199)]
    WhitespaceBlock,

    #[regex(r"\r\n|\r|\n", priority = 198)]
    Newline,

    #[regex(r"[a-zA-Z0-9.]+[{]", |lex| lex.slice(), priority = 198)]
    Element(&'source str),

    #[regex(r"[A-Z]{3}:([a-zA-Z0-9.]+)?[{]", |lex| lex.slice(), priority = 198)]
    MacroElement(&'source str),

    #[token("{", priority = 197)]
    Open,

    #[token("}", priority = 196)]
    Close,

    Content(&'source str),
}

pub(crate) fn parse_document(src: &str) -> Document {
    let mut document = Document::new();
    let mut root = parse_element(&src, &mut document.context);
    root.variant = ElementVariant::Root;
    document.root = Some(root);
    document
}

pub(crate) fn parse_element(src: &str, context: &mut Context) -> Element{
    let mut brackets_waiting_close = 0;
    let mut src_line = 0;
    let mut src_col = 0;
    let mut awaiting_macro = false;
    let mut tree = ElementTreeBuilder::new();

    let mut lexer = Token::lexer(src);
    while let Some(result) = lexer.next() {
        let token = match result {
            Ok(t) => t,
            //Errors are unmatched tokens, we add those as content
            Err(_e) => Token::Content(lexer.slice()),
        };

        //We use this for errors
        if token == Token::Newline || token == Token::Comment {
            src_line += 1;
            src_col = 0;
        } else {
            src_col += 1;
        };

        //Macros always grab the raw text content and then can do
        //their own processing before being added to the Element tree
        if awaiting_macro {
            let macro_element = tree.current();
            match token {
                Token::Open | Token::Element(_) | Token::MacroElement(_) => {
                    //Open brackets need to be tracked so that we can
                    //only close the element on an unterminated end bracket
                    brackets_waiting_close += 1;
                    macro_element.content.push_str(lexer.slice());
                }
                Token::Close => {
                    //We have unterminated empty open brackets
                    if brackets_waiting_close > 0 {
                        brackets_waiting_close -= 1;
                        macro_element.content.push_str(lexer.slice());
                    }
                    //No unterminated empty brackets
                    //Element has been closed and can be removed from the stack
                    //Elements are then added to the previous element in the stack (parent)
                    else {
                        //Macro element its self will never be directly 
                        //added as a child, so we remove it from the tree
                        let element = tree.pop();

                        match macro_parser::parse(&element, context ) {
                            Ok(maybe_element) => {
                                if let Some(generated_element) = maybe_element {
                                    tree.push_child(generated_element);
                                };
                            }
                            Err(err) => {
                                context.errors.push(err);
                            }
                        }

                        //Resume normal parsing
                        awaiting_macro = false;
                    }
                }
                Token::Newline => {
                    macro_element.content.push_str(lexer.slice());
                },
                _ => {
                    //All unmatched content is added as it raw data
                    macro_element.content.push_str(lexer.slice());
                }
            }

            continue;
        }

        //Build the element tree
        match token {
            Token::Element(name) => {
                tree.push(Element::from_raw_name(name));
            },
            Token::MacroElement(name) => {
                tree.push(Element::from_raw_macro_name(name));
                awaiting_macro = true;
            }
            Token::Open => {
                //Open brackets need to be tracked so that we can
                //only close the element on an unterminated end bracket
                tree.push_text(lexer.slice());
                brackets_waiting_close += 1;
            }
            Token::Close => {
                //We have unterminated empty open brackets
                if brackets_waiting_close > 0 {
                    tree.push_text(lexer.slice());
                    brackets_waiting_close -= 1;
                }
                //No unterminated empty brackets
                //Element has been closed and can be removed from the stack
                //Elements are then added to the previous element in the stack (parent)
                else {
                    if !tree.can_fold() {
                        context.errors.push(ParseError::unclosed_element(src_line, src_col));
                        continue;
                    }
                    tree.fold();
                }
            }
            Token::Content(str) => {
                //Gather contents into string. Would be nice if the lexer
                //gave use the whole string instead of character by character
                tree.push_text(str);
            }
            Token::Comment => {
                tree.push_child(Element::comment_variant(lexer.slice()));
            }
            Token::Newline => {
                tree.fold_text();
            }
            _ => {}
        }

    };

    tree.root()
}