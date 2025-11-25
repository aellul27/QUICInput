mod connect;
mod input;
mod menubar;
mod windowresolution;

use libadwaita::gio::SimpleAction;
use libadwaita::prelude::*;
use libadwaita::{glib, Application, ApplicationWindow, HeaderBar};
use gtk4::{Box, Orientation, Stack, StackTransitionType};

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
    // Adwaita's ApplicationWindow does not include a HeaderBar
    let header = HeaderBar::new();
    header.pack_end(&menubar::build(app));
    content.append(&header);

    let stack = build_stack();
    content.append(&stack);

    if app.lookup_action("test").is_none() {
        let connect_action = SimpleAction::new("test", None);
        connect_action.connect_activate(|_, _| {
            println!("Connect > test triggered");
        });
        app.add_action(&connect_action);
    }

    let (window_height, window_width) = windowresolution::find_window_size();

    // Create a window, set the title, and size it relative to the primary display
    let window = ApplicationWindow::builder()
        .application(app)
        .title("QUICinput")
        .default_height(window_height as i32)
        .default_width(window_width as i32)
        .content(&content)
        .build();
    
    // Present window
    window.present();
}

fn build_stack() -> Stack {
    let stack = Stack::builder()
        .hexpand(true)
        .vexpand(true)
        .transition_type(StackTransitionType::SlideLeftRight)
        .build();

    let input_view = input::build();
    stack.add_named(&input_view, Some("input"));

    let stack_for_connect = stack.clone();
    let input_view_for_connect = input_view.clone();
    let connect_view = connect::build(move |ip, port| {
        println!("Connecting to {}:{}", ip, port);
        stack_for_connect.set_visible_child_name("input");
        input_view_for_connect.grab_focus();
    });
    stack.add_named(&connect_view, Some("connect"));

    stack.set_visible_child_name("connect");
    stack
}