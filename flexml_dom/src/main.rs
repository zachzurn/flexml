use crate::document::document::FlexmlDocument;

mod styles;
mod strings;
mod layout;
mod document;

fn main() {
    let input = "[bold+italic this is some text \r\n and some more on a new line ]";

    let document = FlexmlDocument::new(input)
        .with_page_size(1920, 1080)
        .parse();
}