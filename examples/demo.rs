use tui_document::{Document, Text, TextBlock, Title};

fn main() {
    let doc = Document(vec![Box::new(Title {
        text: Text(vec![TextBlock {
            content: String::from("Hello world!"),
            style: Default::default(),
        }]),
        level: tui_document::TitleLevel::Heading1,
    })]);
}
