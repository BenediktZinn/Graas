use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, glib};
use ui::graphic::canvas::Canvas;

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
    let _canvas = Canvas::new(&window);

    window.present();
}
