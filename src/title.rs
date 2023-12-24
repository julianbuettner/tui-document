use crate::{document::DocumentBlock, text::Text, ScrollRect};

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

pub struct Title {
    pub text: Text,
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
    fn render_on_area(&self, area: ScrollRect, buf: &mut ratatui::prelude::Buffer) {
        todo!()
    }
}
