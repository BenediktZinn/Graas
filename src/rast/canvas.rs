use crate::rast::blocks::Block;
use cairo::Context;
use gtk::ApplicationWindow;
use gtk::DrawingArea;
use gtk::prelude::*;

pub struct Canvas {
    drawing_area: DrawingArea,
}

impl Canvas {
    pub fn new(window: &ApplicationWindow) -> Self {
        let drawing_area = DrawingArea::new();

        drawing_area.set_draw_func(|_, ctx, width, height| {
            draw_func(ctx, width.try_into().unwrap(), height.try_into().unwrap());
        });

        window.set_child(Some(&drawing_area));
        Self { drawing_area }
    }
}

fn draw_func(ctx: &Context, _: u32, _: u32) {
    // Implement drawing logic here
    let block = Block::new(
        50.0,
        50.0,
        "this is a very long test\n with multiple lines\n and more".to_string(),
    );
    block.draw(ctx).unwrap();
}
