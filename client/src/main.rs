use libadwaita as adw;

use adw::prelude::*;
use adw::{Application, ApplicationWindow};
use glib::ExitCode;
use gtk4::{Align, Label};

fn build_ui(app: &Application) {
	// Basic label-only window sufficient for demo
	let label = Label::builder()
		.label("Hello, world!")
		.margin_top(24)
		.margin_bottom(24)
		.margin_start(24)
		.margin_end(24)
		.halign(Align::Center)
		.valign(Align::Center)
		.build();

	let window = ApplicationWindow::builder()
		.application(app)
		.title("GTK + Libadwaita")
		.default_width(320)
		.default_height(120)
		.content(&label)
		.build();

	window.present();
}

fn main() -> ExitCode {
	adw::init().expect("Failed to initialize libadwaita");

	let app = Application::builder()
		.application_id("com.example.quicinput")
		.build();

	app.connect_activate(build_ui);

	ExitCode::from(app.run())
}
