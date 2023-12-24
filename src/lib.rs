// TODO: remove dead code
#![allow(dead_code)]

mod atoms;
mod document;
mod text;
mod title;
mod bulletpoint;
mod code;

pub use atoms::Newlines;
pub use document::{Document, DocumentBlock, DocumentWidget};
use ratatui::prelude::Rect;
pub use text::{Text, TextBlock};
pub use title::{Title, TitleLevel};
pub use bulletpoint::BulletPoint;
pub use code::Code;

/// Similiar to Rect, but allows for negative offset.
/// Symbols in negative x or y coordinates are out of
/// screen and not rendered. Useful for scrolling widgets.
pub struct ScrollRect {
    pub x: i32,
    pub y: i32,
    pub width: usize,
    pub height: usize,
}

impl From<Rect> for ScrollRect {
    fn from(value: Rect) -> Self {
        Self {
            x: value.x.into(),
            y: value.y.into(),
            width: value.width.into(),
            height: value.height.into(),
        }
    }
}

impl ScrollRect {
    fn scroll_y(&mut self, diff: i32) {
        self.y += diff;
    }
}
