use gtk4::gdk;
use gtk4::glib::Propagation;
use gtk4::prelude::*;
use gtk4::{Box, EventControllerKey, Label, Orientation};

const OUTER_MARGIN: i32 = 32;
const INNER_SPACING: i32 = 18;

pub fn build() -> Box {
	let container = Box::new(Orientation::Vertical, INNER_SPACING);
	container.set_margin_top(OUTER_MARGIN);
	container.set_margin_bottom(OUTER_MARGIN);
	container.set_margin_start(OUTER_MARGIN);
	container.set_margin_end(OUTER_MARGIN);
	container.set_hexpand(true);
	container.set_vexpand(true);
	container.set_focusable(true);
	container.set_can_focus(true);

	let title = Label::new(Some("Key Event Monitor"));
	title.add_css_class("title-3");
	title.set_xalign(0.0);
	container.append(&title);

	let info = Label::new(Some(
		"This view prints every key press and release to the terminal. Make sure the window is focused and start typing.",
	));
	info.set_xalign(0.0);
	info.set_wrap(true);
	container.append(&info);

	let controller = EventControllerKey::new();
	controller.connect_key_pressed(|_, key, keycode, state| {
		let name = readable_key_name(&key);
		println!(
			"Key press  -> {:<15} | code {:<5} | modifiers {:?}",
			name, keycode, state
		);
		Propagation::Proceed
	});
	controller.connect_key_released(|_, key, keycode, state| {
		let name = readable_key_name(&key);
		println!(
			"Key release -> {:<15} | code {:<5} | modifiers {:?}",
			name, keycode, state
		);
	});
	container.add_controller(controller);

	container
}

fn readable_key_name(key: &gdk::Key) -> String {
	key.name()
		.map(|n| n.to_string())
		.filter(|s| !s.is_empty())
		.unwrap_or_else(|| "Unknown".to_string())
}
