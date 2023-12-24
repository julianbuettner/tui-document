use std::{io::stdout, thread, time::Duration};

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::CrosstermBackend, Terminal};
use tui_document::{BulletPoint, Code, Document, DocumentWidget, Newlines, Text, Title};

fn build_widget() -> DocumentWidget {
    let doc = Document(vec![
        Box::new(Title {
            text: Text::plain(
                "Hello world! This is very much still the title of this document. \
                Make terminal smaller to see how it behaves",
            ),
            level: tui_document::TitleLevel::Heading1,
        }),
        Box::new(Text::plain("This is just some boring old text content.")),
        Box::new(Newlines::new(1)),
        Box::new(Title {
            text: Text::plain("Subtitle Level 2"),
            level: tui_document::TitleLevel::Heading2,
        }),
        Box::new(Title {
            text: Text::plain("Subtitle Level 3"),
            level: tui_document::TitleLevel::Heading3,
        }),
        Box::new(Newlines::new(1)),
        Box::new(Text::plain("This is just some boring old text content.")),
        Box::new(Newlines::new(2)),
        Box::new(BulletPoint::plain(0, "Bullet points are great")),
        Box::new(BulletPoint::plain(1, "Because there are subpoints")),
        Box::new(BulletPoint::plain(
            1,
            "This is a very long bullet point, so \
            you can resize your terminal and check how it's wrapped.",
        )),
        Box::new(BulletPoint::plain(1, "And because there are sub points")),
        Box::new(Newlines::new(2)),
        Box::new(Code::new(
            "rust",
            "// Here some code\npub struct Wow {\n  hi: u64,\n}",
        )),
    ]);
    DocumentWidget {
        document: doc,
        scroll_offset: 0,
    }
}

fn main() {
    enable_raw_mode().unwrap();
    stdout().execute(EnterAlternateScreen).unwrap();
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();

    let mut widget = build_widget();
    for i in 0..5 {
        widget.scroll_offset = i;
        terminal.draw(|f| {
            f.render_widget(&widget, f.size())
        }).unwrap();
        thread::sleep(Duration::from_secs(1));
    }

    disable_raw_mode().unwrap();
    stdout().execute(LeaveAlternateScreen).unwrap();
}
