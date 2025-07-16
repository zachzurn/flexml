use std::ops::Range;
use ariadne::{Color, Label, Report, ReportKind, Source};
use crate::parsing::parser::Parser;

#[derive(Debug, Clone)]
pub struct ParserWarning {
    pub kind: ParserWarningKind,
    pub message: String,
    pub span: Range<usize>,
    pub label: String,
    pub help: String,
    pub fix: Option<String>
}

#[derive(Debug, Clone)]
pub enum ParserWarningKind {
    EmptyInput,
    ExpectedStyleValue,
    UnclosedRawContainer,
    UnclosedStyleContainer,
    UnclosedBoxContainer,
    ExceededNodeDepth,
    ExceededNodeCount,
    StyleContainerNoStyles,
    UnexpectedToken,
}

impl<'a> Parser<'a> {
    pub fn get_warnings(&self) -> Vec<ParserWarning> {
        self.warnings.to_vec()
    }

    pub fn print_warnings(&self, file_name: &str) {
        let mut buffer = Vec::new();

        for warning in &self.warnings {
            let start = warning.span.start;
            let end = warning.span.end;

            Report::build(ReportKind::Warning, (file_name, start..end))
                .with_message(&warning.message)
                .with_label(Label::new((file_name, start..end))
                    .with_message(&warning.label)
                    .with_color(Color::Green))
                .with_help(&warning.help)
                .finish()
                .write((file_name, Source::from(self.input)), &mut buffer)
                .unwrap();
        }

        println!("{}", String::from_utf8(buffer).unwrap());
    }

    /// Records a parser error.
    /// - `span`: The input range that caused the error.
    /// - `message`: A short error description.
    /// - `label`: A label for what the span highlights.
    /// - `help`: A suggestion for fixing the issue.
    pub(super) fn warn(&mut self, span: Range<usize>, kind: ParserWarningKind) {
        let (message, label, help, fix) = match kind {
            ParserWarningKind::EmptyInput => {(
              "Input is empty.",
              "Provide some content",
              "Provide some content like [ This is my text ]",
              "[ Example Text ] [ Some more text ]"
            )},
            ParserWarningKind::ExpectedStyleValue => {(
                "Expected style value, but found nothing",
                "Missing style value",
                "Try removing the : or adding a value",
                "1",
            )},
            ParserWarningKind::UnclosedBoxContainer => {(
                "Unclosed box container",
                "Box container isn't closed properly",
                "Make sure every -> [ has a matching -> ]",
                "]"
            )},
            ParserWarningKind::UnclosedStyleContainer => {(
                "Unclosed style container",
                "Missing closing '}'",
                "Add '}' to close the style container",
                "}"
            )},
            ParserWarningKind::UnclosedRawContainer => {(
                "Unterminated raw container",
                "Raw ended here with no closing tag",
                "Try adding =| to close the raw container",
                "=|"
            )},
            ParserWarningKind::StyleContainerNoStyles => {(
                "Style definition has no styles",
                "Missing styles",
                "Add some styles like bold+italic",
                " bold+italic"
            )},
            ParserWarningKind::ExceededNodeCount => {(
                "Parser stopped due to max nodes exceeded",
                "Maximum nodes exceeded here. Everything after this was not parsed.",
                "Increase the max nodes limit",
                ""
            )},
            ParserWarningKind::ExceededNodeDepth => {(
                "Maximum nodes depth exceeded",
                "Some nodes were ignored",
                "Increase the node depth limit",
                ""
            )},
            ParserWarningKind::UnexpectedToken => {(
                "There was an error while parsing a token",
                "Unexpected text",
                "This should not be a problem but please file an issue",
                ""
            )}
        };

        let fix = if fix.is_empty() { None } else { Some(fix.to_string()) };

        self.warn_detailed(span,kind,message,label,help, fix);
    }

    fn warn_detailed(&mut self, span: Range<usize>, kind: ParserWarningKind, message: &str, label: &str, help: &str, fix: Option<String> ) {
        self.warnings.push(ParserWarning{
            span, kind, fix,
            message: message.to_string(),
            label: label.to_string(),
            help: help.to_string(),
        })
    }
}