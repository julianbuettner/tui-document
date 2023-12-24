use std::cmp::max;

use ansi_to_tui::IntoText;
use syntect::{
    easy::HighlightLines,
    highlighting::ThemeSet,
    parsing::SyntaxSet,
    util::{as_24_bit_terminal_escaped, LinesWithEndings},
};

use crate::DocumentBlock;

pub struct Code {
    pub language: String,
    pub content: String,
}

fn box_code(code: String) -> String {
    let mut result = String::new();
    let width = code.lines().map(|l| l.len()).max().unwrap_or(0);
    for line in code.lines() {
        result += line;
        result += " ".repeat(width - line.len()).as_str();
        result += "\n";
    }
    result
}

impl Code {
    pub fn new(language: impl ToString, content: impl ToString) -> Self {
        Self {
            language: language.to_string(),
            content: box_code(content.to_string()),
        }
    }
}

impl DocumentBlock for Code {
    fn get_text(&self, _width: usize) -> ratatui::text::Text {
        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let syntax = ps
            .find_syntax_by_token(&self.language)
            .unwrap_or(ps.find_syntax_plain_text());
        let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
        let mut formatted_text = String::new();
        for line in LinesWithEndings::from(&self.content) {
            let ranges: Vec<(syntect::highlighting::Style, &str)> =
                h.highlight_line(line, &ps).unwrap();
            let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
            formatted_text += &escaped;
        }
        formatted_text.into_text().unwrap()
    }

    fn box_size_given_width(&self, _width: usize) -> (usize, usize) {
        let (mut x, mut y) = (0, 0);
        for line in self.content.lines() {
            x = max(x, line.len());
            y += 1
        }
        (x, y)
    }

    fn max_width(&self) -> usize {
        self.content
            .lines()
            .map(|l| l.len())
            .max()
            .unwrap_or(0)
    }

    fn width_pressure(&self) -> usize {
        self.max_width()
    }
}
