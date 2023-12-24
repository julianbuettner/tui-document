use std::{io::stdout, thread, time::Duration};

use ratatui::{Terminal, prelude::CrosstermBackend, Frame};
use tui_document::{Document, Text, TextBlock, Title, DocumentWidget};
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand,
};

fn ui(f: &mut Frame) {
    let doc = Document(vec![Box::new(Title {
        text: Text(vec![TextBlock {
            content: String::from("Hello world!"),
            style: Default::default(),
        }]),
        level: tui_document::TitleLevel::Heading1,
    })]);
    let widget = DocumentWidget {
        document: doc,
        scroll_offset: 0,
    };

    f.render_widget(&widget, f.size());
}

fn main() {
    enable_raw_mode().unwrap();
    stdout().execute(EnterAlternateScreen).unwrap();
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();

    terminal.draw(ui);
    thread::sleep(Duration::from_secs(2));

    disable_raw_mode().unwrap();
    stdout().execute(LeaveAlternateScreen).unwrap();
}
