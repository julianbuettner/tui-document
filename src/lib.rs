// TODO: remove dead code
#![allow(dead_code)]

mod document;
mod text;
mod title;

pub use text::{Text, TextBlock, TextBlockStyle};
pub use title::{Title, TitleLevel};
pub use document::{Document, DocumentBlock};
