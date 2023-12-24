use std::{io::stdout, thread, time::Duration};

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::CrosstermBackend, Frame, Terminal};
use tui_document::{BulletPoint, Document, DocumentWidget, Newlines, Text, TextBlock, Title, Code};

fn ui(f: &mut Frame) {
    let doc = Document(vec![
        Box::new(Title {
            text: TextBlock {
                content: String::from(
                    "Hello world! This is very much still the title of this document. \
                Make terminal smaller to see how it behaves",
                ),
                style: Default::default(),
            }
            .space_separate(),
            level: tui_document::TitleLevel::Heading1,
        }),
        Box::new(Newlines::new(1)),
        Box::new(
            TextBlock {
                content: "This is just some boring old text content.".into(),
                style: Default::default(),
            }
            .space_separate(),
        ),
        Box::new(Newlines::new(2)),
        Box::new(BulletPoint::plain(0, "Bullet points are great")),
        Box::new(BulletPoint::plain(1, "Because there are subpoints")),
        Box::new(BulletPoint::plain(1, "And because there are sub points")),
        Box::new(Newlines::new(2)),
        Box::new(Code::new("rust", "pub struct Wow {\n  hi: u64,\n}"))
    ]);
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

    terminal.draw(ui).unwrap();
    thread::sleep(Duration::from_secs(2));

    disable_raw_mode().unwrap();
    stdout().execute(LeaveAlternateScreen).unwrap();
}
