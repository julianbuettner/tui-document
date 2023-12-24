use std::{
    iter::{once, repeat_with},
};

use ratatui::text::Span;

use crate::{DocumentBlock, Text};

static SPACES_PER_INDENT: usize = 4;

pub struct BulletPoint {
    pub indent: usize,
    pub point: String,
    pub content: Box<dyn DocumentBlock>,
}

impl BulletPoint {
    pub fn new(indent: usize, point: impl ToString, content: Box<dyn DocumentBlock>) -> Self {
        Self {
            indent,
            point: point.to_string(),
            content,
        }
    }
    pub fn plain(indent: usize, content: impl ToString) -> Self {
        Self::new(indent, "\u{2022}", Box::new(Text::plain(content)))
    }
    pub fn max_width(&self) -> usize {
        self.content.max_width()
    }
    pub fn indent_width(&self) -> usize {
        // Indent consists of:
        // - Spaces for indent greater 0
        // - The point itself, which might be wider than 1
        // - One space after the point
        self.indent * SPACES_PER_INDENT + Span::from(&self.point).width() + 1
    }
}

impl DocumentBlock for BulletPoint {
    fn width_pressure(&self) -> usize {
        self.content.width_pressure() + self.indent_width()
    }
    fn max_width(&self) -> usize {
        self.content.max_width() + self.indent_width()
    }
    fn box_size_given_width(&self, width: usize) -> (usize, usize) {
        let (x, y) = self.content.box_size_given_width(width - self.indent_width());
        (x + self.indent_width(), y)
    }
    fn get_text(&self, width: usize) -> ratatui::text::Text {
        let first_prefix: String = " ".repeat(self.indent * SPACES_PER_INDENT).chars()
            .chain(self.point.chars())
            .chain(once(' '))
            .collect();
        let empty_prefix = " ".repeat(self.indent_width());
        let line_prefixes = once(first_prefix).chain(repeat_with(|| empty_prefix.clone()));

        let mut result = Vec::new();
        let lines = self.content.get_text(width - self.indent_width()).lines.into_iter();
        for (pref, mut line) in line_prefixes.zip(lines) {
            line.spans.insert(0, Span::from(pref));
            result.push(line);
        }
        ratatui::text::Text { lines: result }
    }
}
