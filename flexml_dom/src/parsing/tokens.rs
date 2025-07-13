use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(subpattern WS = r"[ \t\f\r\n]*")]
pub enum Token {
    #[regex(r"[ \t\f]+", priority = 5)]
    Whitespace,

    #[regex(r"\r\n|\r|\n", priority = 5)]
    Newline,

    #[regex(r"[A-Za-z0-9]+", priority = 5)]
    Named,

    #[token("=", priority = 5)]
    StyleNameSeparator,

    #[token("+", priority = 5)]
    StyleSeparator,

    #[token(":", priority = 5)]
    StyleParamSeparator,

    #[token("[", priority = 5)]
    BoxContainerOpen,

    #[token("]", priority = 5)]
    BoxContainerClose,

    #[token("{", priority = 5)]
    StyleContainerOpen,

    #[token("}", priority = 5)]
    StyleContainerClose,

    #[regex(r"<[A-Za-z0-9_-]+>", priority = 5)]
    TagContainer,

    #[token("\\|=", priority = 5)]
    RawOpenEscaped,

    #[token("\\=|", priority = 5)]
    RawCloseEscaped,

    #[token("|=", priority = 5)]
    RawOpen,

    #[token("=|", priority = 5)]
    RawClose,

    #[token("|")]
    #[regex(r"[^{}\[\]\+{}:|=\s\r\n\t\f]+")]
    Text,
}
