//! main.rs - launcher for the Dioxus desktop app
//! Place in: dioxus-app/src/main.rs

use dioxus_desktop::{launch_cfg, ConfigBuilder};
use std::process;
use log::{info, error};

fn main() {
    // Initialize simple logger - replace with your preferred logger
    // Be sure to add env_logger or similar to Cargo.toml if you use this
    let _ = env_logger::try_init();

    info!("Application started.");

    // Initialize Python + ONNX DQN. main will log errors to console but will continue launching UI.
    match crate::python_integration::init_python_dqn() {
        Ok(_) => info!("Python DQN initialized OK."),
        Err(err) => {
            error!("Failed to initialize Python DQN: {}", err);
            // Continue launching the UI: you can still debug via console. If you want to abort, uncomment below:
            // eprintln!("Aborting due to Python init failure.");
            // process::exit(1);
        }
    }

    // Minimal desktop config - adjust window size/title as desired
    let cfg = ConfigBuilder::new().with_title("Deadlock-net Dioxus + DQN").build();

    // Launch the Dioxus app. App is defined in lib.rs
    launch_cfg(dioxus_app::App, cfg);
}
