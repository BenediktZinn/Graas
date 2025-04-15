use crate::ui::graphic::blocks::Block;
use cairo::Context;
use gtk::DrawingArea;
use gtk::prelude::*;

pub struct BlockCanvas {
    pub drawing_area: DrawingArea,
}

impl BlockCanvas {
    pub fn new() -> Self {
        let drawing_area = DrawingArea::new();

        drawing_area.set_draw_func(|_, ctx, width, height| {
            draw_func(ctx, width.try_into().unwrap(), height.try_into().unwrap());
        });

        Self { drawing_area }
    }
}

fn draw_func(ctx: &Context, _: u32, _: u32) {
    // Implement drawing logic here
    let mut block = Block::new(
        50.0,
        50.0,
        "this is a very long test\n with multiple lines\n and more".to_string(),
    );
    block.draw(ctx).unwrap();
}
