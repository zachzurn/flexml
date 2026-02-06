extern crate core;

mod styles;
mod strings;
pub mod layout;
pub mod document;

#[cfg(test)]
mod tests {
    use crate::document::parser::FlexmlDocument;
    use crate::layout::{FlexmlLayout, FlexmlLayoutContext};

    #[test]
    fn it_parses() {
        let input = "[bold+italic this is some text \r\n and some more on a new line ]";

        let document = FlexmlDocument::new(input)
            .parse();

        let _layout = FlexmlLayout::new(&document, FlexmlLayoutContext::default());
    }
}
