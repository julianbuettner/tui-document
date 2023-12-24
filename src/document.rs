use ratatui::{style::Style, prelude::{Rect, Buffer}};

use crate::ScrollRect;

pub trait DocumentBlock {
    // If the block can be rendered on a
    // screen which is infinitely wide,
    // how wide would it be?
    fn max_width(&self) -> usize;

    // A measurement about how much it
    // would like to have to look good.
    // It is a kind of soft min_width.
    // A long word of 30 chars could
    // just be broken inbetween, but
    // it would be much nicer if it would not.
    // So the width_pressure should be probably 30.
    // With time, these values will be tuned.
    fn width_pressure(&self) -> usize;

    // How many lines are needed, given
    // a defined width. This value will determine
    // the render box size.
    // When rendered across multiple lines,
    // how wide does the box actually have to be?
    // So it is usually called before
    // [Widget::render](https://docs.rs/ratatui/latest/ratatui/widgets/trait.Widget.html)
    fn box_size_given_width(&self, width: usize) -> (usize, usize);

    fn render_on_area(&self, area: ScrollRect, buf: &mut Buffer);
}

pub struct Document(pub Vec<Box<dyn DocumentBlock>>);

pub struct DocumentWidget {
    pub document: Document,
    /// Basically how many lines to skip
    /// before rendering.
    pub scroll_offset: usize,
}


impl ratatui::widgets::Widget for &DocumentWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let mut rect: ScrollRect = area.into();
        rect.scroll_y(self.scroll_offset as i32);
        let current_line = self.scroll_offset as i64 * -1;
        for block in self.document.0.iter() {
            let (width, height) = block.box_size_given_width(area.width as usize);
        }
    }
}
