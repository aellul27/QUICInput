use gtk4::prelude::*;
use gtk4::{Box, GestureClick, Label, Orientation};
use crate::key_monitor::start_global_key_monitor;

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

	let title = Label::new(Some("Event monitor"));
	title.add_css_class("title-3");
	title.set_xalign(0.0);
	container.append(&title);

	let info = Label::new(Some("Click here to start key capture."));
	info.set_xalign(0.0);
	info.set_wrap(true);
	let clicker = GestureClick::new();
	clicker.connect_pressed(|_, _, _, _| {
		start_global_key_monitor();
	});
	info.add_controller(clicker);
	container.append(&info);

	container
}
