use std::cmp::{min, max};

use crate::document::DocumentBlock;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone)]
pub struct TextBlockStyle {
    pub underlined: bool,
    pub stricken_through: bool,
    pub bold: bool,
    pub italic: bool,
}

impl Default for TextBlockStyle {
    fn default() -> Self {
        Self {
            underlined: false,
            stricken_through: false,
            bold: false,
            italic: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TextBlock {
    pub content: String,
    pub style: TextBlockStyle,
}

impl TextBlock {
    pub fn space_separate(&self) -> Text {
        Text(self.content
            .split(" ")
            .into_iter()
            .map(|word| TextBlock {
                content: format!("{} ", word),
                style: self.style.clone(),
            })
            .collect())
    }
}

pub struct Text(pub Vec<TextBlock>);

impl DocumentBlock for TextBlock {
    fn max_width(&self) -> usize {
        self.content.graphemes(true).count()
    }
    fn width_pressure(&self) -> usize {
        self.max_width()
    }
    fn box_size_given_width(&self, width: usize) -> (usize, usize) {
        // Expect to contain no space. This TextBlock should
        // be one word in a space separated Text.
        (min(width, self.max_width()), self.max_width() / width + 1)
    }
}

impl DocumentBlock for Text {
    fn max_width(&self) -> usize {
        self.0.iter().map(|x| x.max_width()).sum()
    }
    fn width_pressure(&self) -> usize {
        self.0.iter().map(|x| x.max_width()).max().unwrap_or(0)
    }
    fn box_size_given_width(&self, width: usize) -> (usize, usize) {
        let mut line_count = 1;
        let mut max_width = 0;
        let mut current_line_length = 0;
        for word in self.0.iter() {
            let word_length = word.max_width();
            let overlength = word_length > width;

            match (current_line_length, word_length, overlength) {
                (0, n, true) => {
                    line_count += n / width + 1;
                }
                (_, n, true) => {
                    line_count += 1;
                    current_line_length = 0;
                    line_count += n / width + 1;
                }
                (c, n, false) if c + n > width => {
                    line_count += 1;
                    current_line_length = n;
                }
                (c, n, false) if c + n <= width => {
                    current_line_length += n;
                }
                (_, _, false) => unreachable!(),

            }
            max_width = max(max_width, current_line_length);
        }
        max_width = min(max_width, width);
        (max_width, line_count)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn text_block_box_size_given_width() {
        let text = TextBlock {
            content: "This-is-some-dummy-text".to_string(),
            style: Default::default(),
        };
        assert_eq!(text.content.len(), 23);

        let (x, y) = text.box_size_given_width(10);
        // Will fill the entire box and need three lines.
        assert_eq!((x, y), (10, 3));
    }

    #[test]
    fn text_box_size_given_width() {

        let text = TextBlock {
            content: "This is some dummy text".to_string(),
            style: Default::default(),
        };
        let blocks = text.space_separate();

        // Given a width of 12 will will get
        // 123456789ABCDEF
        // This is |
        // some dummy |
        // text |
        //
        // With max length of 11

        let (x, y) = blocks.box_size_given_width(12);
        assert_eq!((x, y), (11, 3));

    }
}
