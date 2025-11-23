use libadwaita::prelude::*;
use libadwaita::{glib, Application, ApplicationWindow, HeaderBar};
use gtk4::{Box, Orientation};

const APP_ID: &str = "com.aellul27.quicinput.client";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {

    // Combine the content in a box
    let content = Box::new(Orientation::Vertical, 0);
    // Adwaitas' ApplicationWindow does not include a HeaderBar
    content.append(&HeaderBar::new());
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("QUICinput")
        .content(&content)
        .build();

    // Present window
    window.present();
}