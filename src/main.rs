use gtk::{Application, ApplicationWindow, Orientation, glib};
use gtk::{Paned, prelude::*};
use ui::graphic::block_canvas::BlockCanvas;
use ui::text::text_canvas::TextCanvas;

pub mod ui;

const APP_ID: &str = "org.graas.MainUI";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}
fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Graas")
        .default_width(800)
        .default_height(600)
        .build();
    let block_canvas = BlockCanvas::new();
    let text_canvas = TextCanvas::new();

    let main_pane = Paned::new(Orientation::Horizontal);
    main_pane.set_start_child(Some(&text_canvas.source_view));
    main_pane.set_end_child(Some(&block_canvas.drawing_area));

    main_pane.set_position(400);
    window.set_child(Some(&main_pane));
    window.present();
}
