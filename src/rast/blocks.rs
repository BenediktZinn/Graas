use cairo::Context;

pub struct Block {
    x: f64,
    y: f64,
    label: String,
}

impl Block {
    pub fn new(x: f64, y: f64, label: String) -> Self {
        Block { x, y, label }
    }

    pub fn draw(&self, cr: &Context) -> Result<(), cairo::Error> {
        let padding_x = 20.0;
        let padding_y = 20.0;
        let font_size = 18.0;

        cr.set_font_size(font_size);

        let extents = cr.text_extents(&self.label)?;
        let text_width = extents.width();
        let text_height = extents.height();

        // Total block dimensions with padding
        let block_width = text_width + padding_x * 2.0;
        let block_height = text_height + padding_y * 2.0;

        // Draw block background
        cr.set_source_rgb(0.2, 0.6, 0.8);
        cr.rectangle(self.x, self.y, block_width, block_height);
        cr.fill_preserve()?;
        cr.set_source_rgb(0.0, 0.0, 0.0);
        cr.set_line_width(2.0);
        cr.stroke()?;

        // Draw label (baseline is at y + padding + text_height)
        let text_x = self.x + padding_x;
        let text_y = self.y + padding_y + extents.height(); // baseline adjustment
        cr.move_to(text_x, text_y);
        cr.set_source_rgb(0.0, 0.0, 0.0);
        cr.show_text(&self.label)?;
        cr.stroke()?;

        Ok(())
    }
}
