use ratatui::{
    style::Style,
    text::{Span, Text},
};

use crate::{document::DocumentBlock, text::Text as DText};

/// Heading One
/// ===========
///
/// ## Heading 2
///
/// ### Heading 3
///
/// ...
pub enum TitleLevel {
    Heading1,
    Heading2,
    Heading3,
    Heading4,
    Heading5,
    Heading6,
}

impl TitleLevel {
    fn as_u8(&self) -> u8 {
        match self {
            Self::Heading1 => 1,
            Self::Heading2 => 2,
            Self::Heading3 => 3,
            Self::Heading4 => 4,
            Self::Heading5 => 5,
            Self::Heading6 => 6,
        }
    }
}

pub struct Title {
    pub text: DText,
    pub level: TitleLevel,
}

impl DocumentBlock for Title {
    fn width_pressure(&self) -> usize {
        self.text.width_pressure()
    }
    fn max_width(&self) -> usize {
        self.text.max_width()
    }
    fn box_size_given_width(&self, width: usize) -> (usize, usize) {
        match self.level {
            // Heading 1 is under stricken by a line of equal signs
            TitleLevel::Heading1 => {
                let (x, y) = self.text.box_size_given_width(width);
                (x, y + 1)
            }
            // Heading 2 is prefixed in every line by "## "
            TitleLevel::Heading2 => self.text.box_size_given_width(width - 3),
            // Heading 3 is prefixed in every line by "### "
            TitleLevel::Heading3 => self.text.box_size_given_width(width - 4),
            TitleLevel::Heading4 => self.text.box_size_given_width(width - 5),
            TitleLevel::Heading5 => self.text.box_size_given_width(width - 6),
            TitleLevel::Heading6 => self.text.box_size_given_width(width - 7),
        }
    }
    fn get_text(&self, width: usize) -> ratatui::text::Text {
        let (underline, prefix) = match self.level {
            TitleLevel::Heading1 => (true, ""),
            TitleLevel::Heading2 => (false, "# "),
            TitleLevel::Heading3 => (false, "## "),
            TitleLevel::Heading4 => (false, "### "),
            TitleLevel::Heading5 => (false, "#### "),
            TitleLevel::Heading6 => (false, "##### "),
        };
        let mut lines: Vec<ratatui::text::Line<'_>> = self
            .text
            .get_text(width - prefix.len())
            .lines
            .into_iter()
            .map(|mut line| {
                line.spans.insert(0, Span::from(prefix));
                line
            })
            .collect();
        if underline {
            let real_width = lines.iter().map(|line| line.width()).max().unwrap_or(3);
            let bar: String = "=".repeat(real_width);
            let line_span = Span::from(bar);
            lines.push(ratatui::text::Line::from(line_span));
        }
        let t = Text::from(lines);
        t.patch_style(Style::default().fg(ratatui::style::Color::LightYellow))
    }
}
