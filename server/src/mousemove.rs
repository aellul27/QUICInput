
use shared::MouseMove;

#[cfg(target_os = "linux")]
pub fn do_mouse_move(uinput: uinput::Device, mousemove: MouseMove) {
    device.position(&relative::X, mousemove.dx)?;  // dx = +10
    device.position(&relative::Y, mousemove.dy)?; // dy = -5
    device.synchronize()?;
}

#[cfg(not(target_os = "linux"))]
use rdev::EventType;
use crate::simulator::EventSimulator;
use mouse_position::mouse_position::{Mouse};

pub fn do_mouse_move(simulator: &EventSimulator, mousemove: MouseMove) {
    let position = Mouse::get_mouse_position();
        match position {
            Mouse::Position { x, y } => {
                let event: EventType = EventType::MouseMove { x: (x as f64 + mousemove.dx), y: (y as f64+ mousemove.dx) };
                simulator.enqueue(event);
            }
            Mouse::Error => println!("Error getting mouse position"),
    }
}