use gtk4::prelude::*;
use gtk4::{Box, GestureClick, Label, Orientation};
use quinn::{Connection, Endpoint};
use crate::key_monitor::start_global_key_monitor;
use glib::{object::ObjectType, WeakRef};
use std::{cell::RefCell, thread::LocalKey};

thread_local! {
    static ROOT_CONTAINER: RefCell<Option<WeakRef<Box>>> = RefCell::new(None);
	static INFO_LABEL:  RefCell<Option<WeakRef<Label>>> = RefCell::new(None);
	static CONNECTION: RefCell<Option<(Endpoint, Connection)>> = RefCell::new(None);
}


const OUTER_MARGIN: i32 = 32;
const INNER_SPACING: i32 = 18;

pub fn build() -> Box {
	let container = Box::new(Orientation::Vertical, INNER_SPACING);
	store_widget(&ROOT_CONTAINER, &container);
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

	let info: Label = Label::new(Some("Click here to start capture."));
	store_widget(&INFO_LABEL, &info);
	info.set_xalign(0.0);
	info.set_wrap(true);
	let clicker = GestureClick::new();
	clicker.connect_pressed(move |_, _, _, _| {
		with_connection(|endpoint, connection| {
			start_global_key_monitor(endpoint, connection);
			with_widget(&ROOT_CONTAINER, |container: Box| {
				container.set_cursor_from_name(Some("none"));
			});
			with_widget(&INFO_LABEL, |label: Label| {
				label.set_label("Type CTRL-ALT-0 to ungrab and stop capture.");
			});
		});
	});
	container.add_controller(clicker);
	container.append(&info);

	container
}

pub fn set_connection(endpoint: Endpoint, connection: Connection) {
	CONNECTION.with(|cell| {
		*cell.borrow_mut() = Some((endpoint, connection));
	});
}

fn with_widget<T, F>(storage: &'static LocalKey<RefCell<Option<WeakRef<T>>>>, action: F)
where
	T: Clone + ObjectType,
	F: FnOnce(T),
{
	storage.with(|cell| {
		if let Some(widget) = cell
			.borrow()
			.as_ref()
			.and_then(|weak| weak.upgrade())
		{
			action(widget);
		}
	});
}

fn store_widget<T>(storage: &'static LocalKey<RefCell<Option<WeakRef<T>>>>, widget: &T)
where
	T: Clone + ObjectType,
{
	storage.with(|cell| {
		*cell.borrow_mut() = Some(widget.downgrade());
	});
}

fn with_connection<F>(action: F)
where
	F: FnOnce(Endpoint, Connection),
{
	CONNECTION.with(|cell| {
		if let Some((endpoint, connection)) = cell.borrow().as_ref() {
			action(endpoint.clone(), connection.clone());
		}
	});
}

pub fn input_ungrabbed() {
	with_widget(&ROOT_CONTAINER, |container: Box| {
		container.set_cursor_from_name(None);
	});
	with_widget(&INFO_LABEL, |label: Label| {
		label.set_label("Click here to start capture.");
	});
}
