use std::ops::Range;
use ariadne::{Color, Label, Report, ReportKind, Source};
use crate::parsing::parser::Parser;
use crate::strings::ParserWarnings;

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
            ParserWarningKind::EmptyInput => (
                ParserWarnings::MSG_EMPTY_INPUT,
                ParserWarnings::LABEL_EMPTY_INPUT,
                ParserWarnings::HELP_EMPTY_INPUT,
                ParserWarnings::FIX_EMPTY_INPUT,
            ),
            ParserWarningKind::ExpectedStyleValue => (
                ParserWarnings::MSG_EXPECTED_STYLE_VALUE,
                ParserWarnings::LABEL_EXPECTED_STYLE_VALUE,
                ParserWarnings::HELP_EXPECTED_STYLE_VALUE,
                ParserWarnings::FIX_EXPECTED_STYLE_VALUE,
            ),
            ParserWarningKind::UnclosedBoxContainer => (
                ParserWarnings::MSG_UNCLOSED_BOX_CONTAINER,
                ParserWarnings::LABEL_UNCLOSED_BOX_CONTAINER,
                ParserWarnings::HELP_UNCLOSED_BOX_CONTAINER,
                ParserWarnings::FIX_UNCLOSED_BOX_CONTAINER,
            ),
            ParserWarningKind::UnclosedStyleContainer => (
                ParserWarnings::MSG_UNCLOSED_STYLE_CONTAINER,
                ParserWarnings::LABEL_UNCLOSED_STYLE_CONTAINER,
                ParserWarnings::HELP_UNCLOSED_STYLE_CONTAINER,
                ParserWarnings::FIX_UNCLOSED_STYLE_CONTAINER,
            ),
            ParserWarningKind::UnclosedRawContainer => (
                ParserWarnings::MSG_UNCLOSED_RAW_CONTAINER,
                ParserWarnings::LABEL_UNCLOSED_RAW_CONTAINER,
                ParserWarnings::HELP_UNCLOSED_RAW_CONTAINER,
                ParserWarnings::FIX_UNCLOSED_RAW_CONTAINER,
            ),
            ParserWarningKind::StyleContainerNoStyles => (
                ParserWarnings::MSG_STYLE_CONTAINER_NO_STYLES,
                ParserWarnings::LABEL_STYLE_CONTAINER_NO_STYLES,
                ParserWarnings::HELP_STYLE_CONTAINER_NO_STYLES,
                ParserWarnings::FIX_STYLE_CONTAINER_NO_STYLES,
            ),
            ParserWarningKind::ExceededNodeCount => (
                ParserWarnings::MSG_EXCEEDED_NODE_COUNT,
                ParserWarnings::LABEL_EXCEEDED_NODE_COUNT,
                ParserWarnings::HELP_EXCEEDED_NODE_COUNT,
                ParserWarnings::FIX_EXCEEDED_NODE_COUNT,
            ),
            ParserWarningKind::ExceededNodeDepth => (
                ParserWarnings::MSG_EXCEEDED_NODE_DEPTH,
                ParserWarnings::LABEL_EXCEEDED_NODE_DEPTH,
                ParserWarnings::HELP_EXCEEDED_NODE_DEPTH,
                ParserWarnings::FIX_EXCEEDED_NODE_DEPTH,
            ),
            ParserWarningKind::UnexpectedToken => (
                ParserWarnings::MSG_UNEXPECTED_TOKEN,
                ParserWarnings::LABEL_UNEXPECTED_TOKEN,
                ParserWarnings::HELP_UNEXPECTED_TOKEN,
                ParserWarnings::FIX_UNEXPECTED_TOKEN,
            ),
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