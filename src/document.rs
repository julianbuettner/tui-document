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
    // how width does the box actually have to be?
    // So it is usually called before
    // [Widget::render](https://docs.rs/ratatui/latest/ratatui/widgets/trait.Widget.html)
    fn box_size_given_width(&self, width: usize) -> (usize, usize);
}

pub struct Document(pub Vec<Box<dyn DocumentBlock>>);

pub struct DocumentWidget {
    pub document: Document,
    /// Basically how many lines to skip
    /// before rendering.
    pub scroll_offset: usize,
}

impl ratatui::widgets::Widget for DocumentWidget {

}
