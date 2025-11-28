use std::{
    env,
    error::Error,
    net::{SocketAddr},
    sync::Arc,
};

#[cfg(target_os = "linux")]
use std::sync::Mutex;

mod simulator;
mod mousemove;
mod server;
mod loadconfig;
mod config;

use crate::{config::QUICInputConfig, simulator::EventSimulator};
use crate::server::{run_server, DeviceInput, Simulators};

#[cfg(target_os = "linux")]
use crate::mousemove::create_virtual_mouse;
#[cfg(target_os = "linux")]
use crate::server::ensure_uinput_available;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let args: Vec<String> = env::args().collect();
    let quicconfig = if let Some(config_file) = args.get(1) {
        println!("Config File: {}", config_file);
        loadconfig::load_config(config_file)
    } else {
        println!("No config file! Using defaults");
        QUICInputConfig::default()
    };
    let addr = SocketAddr::new(quicconfig.broadcastip, quicconfig.port);
    let simulators: Simulators = Arc::new([EventSimulator::new(), EventSimulator::new()]);

    #[cfg(target_os = "linux")]
    let device_input = {
        ensure_uinput_available();
        match create_virtual_mouse() {
            Ok(device) => Arc::new(Mutex::new(Some(device))),
            Err(err) => {
                eprintln!("[server] failed to create virtual mouse: {err}");
                Arc::new(Mutex::new(None))
            }
        }
    };

    #[cfg(not(target_os = "linux"))]
    let device_input: DeviceInput = ();

    run_server(addr, quicconfig.max_connections, simulators, device_input).await
}
