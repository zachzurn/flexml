pub struct ValueHelp;

impl ValueHelp {
    pub const NUMBER: &'static [&'static str] = &["1", "100", "-100", "100%"];
    pub const POSITIVE_NUMBER: &'static [&'static str] = &["1", "100", "100%"];
    pub const PERCENT: &'static [&'static str] = &["1.5%", "100%", "200%"];
    pub const URL: &'static [&'static str] = &["http://www.google.com/image.png", "image.jpg", "../image.png"];
    pub const FATAL_MATCH: &'static [&'static str] = &["This atomic style is broken"];
    pub const COLOR: &'static [&'static str] = &["#FFFFFF", "#FF", "#FF0000FF"];
    pub const FLOAT: &'static [&'static str] = &["0", "1.0", "-1.0"];
}

pub struct ValueErrors;

impl ValueErrors {
    pub const NUMBER: &'static str = "Invalid number";
    pub const MATCH: &'static str = "Invalid value";
    pub const FATAL_MATCH: &'static str = "Fatal error when matching";
    pub const NEGATIVE_PERCENT: &'static str = "Invalid value, percent numbers can't be negative";
    pub const NEGATIVE_NUMBER: &'static str = "Invalid value, number can't be negative";
    pub const URL: &'static str = "Invalid URL";
    pub const COLOR: &'static str = "Invalid color";
    pub const FLOAT: &'static str = "Invalid decimal number";
}

pub struct Chars;

impl Chars {
    pub const PERCENT: &'static str = "%";
    pub const PX: &'static str = "px";
    pub const IN: &'static str = "in";
    pub const MM: &'static str = "mm";
    pub const EM: &'static str = "em";
    pub const REM: &'static str = "rem";
    pub const PT: &'static str = "pt";
    pub const HEX: &'static str = "#";
    pub const FORWARD: &'static str = ">";
}


pub struct ParserWarnings;

impl ParserWarnings {
        // Messages
        pub const MSG_EMPTY_INPUT: &'static str = "Input is empty.";
        pub const MSG_EXPECTED_STYLE_VALUE: &'static str = "Expected style value, but found nothing";
        pub const MSG_UNCLOSED_BOX_CONTAINER: &'static str = "Unclosed box container";
        pub const MSG_UNCLOSED_STYLE_CONTAINER: &'static str = "Unclosed style container";
        pub const MSG_UNCLOSED_RAW_CONTAINER: &'static str = "Unterminated raw container";
        pub const MSG_STYLE_CONTAINER_NO_STYLES: &'static str = "Style definition has no styles";
        pub const MSG_EXCEEDED_NODE_COUNT: &'static str = "Parser stopped due to max nodes exceeded";
        pub const MSG_EXCEEDED_NODE_DEPTH: &'static str = "Maximum nodes depth exceeded";
        pub const MSG_UNEXPECTED_TOKEN: &'static str = "There was an error while parsing a token";
        pub const MSG_OVERWROTE_USER_STYLE: &'static str = "You are overwriting a defined style here";
        pub const MSG_ATOMIC_STYLE: &'static str = "Attempting to define a custom style for a built-in style";

        // Labels
        pub const LABEL_EMPTY_INPUT: &'static str = "Provide some content";
        pub const LABEL_EXPECTED_STYLE_VALUE: &'static str = "Missing style value";
        pub const LABEL_UNCLOSED_BOX_CONTAINER: &'static str = "Box container isn't closed properly";
        pub const LABEL_UNCLOSED_STYLE_CONTAINER: &'static str = "Missing closing '}'";
        pub const LABEL_UNCLOSED_RAW_CONTAINER: &'static str = "Raw ended here with no closing tag";
        pub const LABEL_STYLE_CONTAINER_NO_STYLES: &'static str = "Missing styles";
        pub const LABEL_EXCEEDED_NODE_COUNT: &'static str = "Maximum nodes exceeded here. Everything after this was not parsed.";
        pub const LABEL_EXCEEDED_NODE_DEPTH: &'static str = "Some nodes were ignored";
        pub const LABEL_UNEXPECTED_TOKEN: &'static str = "Unexpected text";
        pub const LABEL_OVERWROTE_USER_STYLE: &'static str = "This style definition was defined earlier";
        pub const LABEL_ATOMIC_STYLE: &'static str = "Built in style name is being used";

        // Help
        pub const HELP_EMPTY_INPUT: &'static str = "Provide some content like [ This is my text ]";
        pub const HELP_EXPECTED_STYLE_VALUE: &'static str = "Try removing the : or adding a value";
        pub const HELP_UNCLOSED_BOX_CONTAINER: &'static str = "Make sure every -> [ has a matching -> ]";
        pub const HELP_UNCLOSED_STYLE_CONTAINER: &'static str = "Add '}' to close the style container";
        pub const HELP_UNCLOSED_RAW_CONTAINER: &'static str = "Try adding =| to close the raw container";
        pub const HELP_STYLE_CONTAINER_NO_STYLES: &'static str = "Add some styles like bold+italic";
        pub const HELP_EXCEEDED_NODE_COUNT: &'static str = "Increase the max nodes limit";
        pub const HELP_EXCEEDED_NODE_DEPTH: &'static str = "Increase the node depth limit";
        pub const HELP_UNEXPECTED_TOKEN: &'static str = "This should not be a problem but please file an issue";
        pub const HELP_OVERWROTE_USER_STYLE: &'static str = "Try renaming this style or the earlier one";
        pub const HELP_ATOMIC_STYLE: &'static str = "Try renaming this style (You can also use a different letter casing)";

        // Fixes
        pub const FIX_EMPTY_INPUT: &'static str = "[ Example Text ] [ Some more text ]";
        pub const FIX_EXPECTED_STYLE_VALUE: &'static str = "1";
        pub const FIX_UNCLOSED_BOX_CONTAINER: &'static str = "]";
        pub const FIX_UNCLOSED_STYLE_CONTAINER: &'static str = "}";
        pub const FIX_UNCLOSED_RAW_CONTAINER: &'static str = "=|";
        pub const FIX_STYLE_CONTAINER_NO_STYLES: &'static str = " bold+italic";
        pub const FIX_NONE: &'static str = "";
}