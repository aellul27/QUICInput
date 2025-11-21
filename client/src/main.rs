use winit::{
    event::{Event, WindowEvent, ElementState, MouseButton, DeviceEvent},
    event_loop::EventLoop,
    keyboard::Key,
    window::{WindowBuilder, CursorGrabMode},
};

pub fn main() {
    // Create winit event loop & window
    let event_loop = EventLoop::new().expect("event loop");
    let window = WindowBuilder::new()
        .with_title("Pointer Lock Demo")
        .build(&event_loop)
        .expect("create window");

    println!("Click (left) to lock pointer. Unlock with Ctrl+Alt+0.\nLogging mouse & keyboard events...");

    let mut locked = false;
    let mut modifiers = winit::keyboard::ModifiersState::empty();

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    elwt.exit();
                }
                WindowEvent::ModifiersChanged(m) => {
                    modifiers = m.state();
                    println!("Modifiers: {:?}", modifiers);
                }
                WindowEvent::CursorMoved { position, .. } => {
                    if !locked { println!("CursorMoved: {:?}", position); }
                }
                WindowEvent::CursorEntered { .. } => println!("CursorEntered"),
                WindowEvent::CursorLeft { .. } => println!("CursorLeft"),
                WindowEvent::MouseWheel { delta, .. } => println!("MouseWheel: {:?}", delta),
                WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. } => {
                    if !locked {
                        window.set_cursor_visible(false);
                        let grab = window.set_cursor_grab(CursorGrabMode::Locked)
                            .or_else(|_| window.set_cursor_grab(CursorGrabMode::Confined));
                        match grab {
                            Ok(_) => { locked = true; println!("Locked pointer."); }
                            Err(e) => println!("Grab failed: {e}"),
                        }
                    } else {
                        println!("Left click (already locked)");
                    }
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    println!("MouseInput: {:?} {:?}", button, state);
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    let k = &event.logical_key;
                    println!("Key: {:?} state={:?} mods={:?}", k, event.state, modifiers);
                    if locked && event.state == ElementState::Pressed && modifiers.control_key() && modifiers.alt_key() {
                        if let Key::Character(s) = k { if s == "0" { unlock(&window, &mut locked); } }
                    }
                }
                _ => {}
            },
            Event::DeviceEvent { event, .. } => {
                if locked {
                    match event {
                        DeviceEvent::MouseMotion { delta } => println!("MouseMotion delta: {:?}", delta),
                        DeviceEvent::Button { button, state } => println!("Raw Button {} state={:?}", button, state),
                        DeviceEvent::MouseWheel { delta } => println!("Raw Wheel delta: {:?}", delta),
                        _ => {}
                    }
                }
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        }
    }).unwrap();
}

fn unlock(window: &winit::window::Window, locked: &mut bool) {
    if let Err(e) = window.set_cursor_grab(CursorGrabMode::None) { println!("Release failed: {e}"); }
    window.set_cursor_visible(true);
    *locked = false;
    println!("Unlocked pointer.");
}
