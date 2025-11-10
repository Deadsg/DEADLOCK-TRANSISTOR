//! lib.rs - Updated for Dioxus 0.5+ and PyO3 0.22+
//! Compatible with your ONNX DQN integration

use dioxus::prelude::*;
use std::thread;

pub mod python_integration {
    use pyo3::prelude::*;
    use pyo3::types::PyTuple;

    /// Initialize the Python DQN environment and load the ONNX model
    pub fn init_python_dqn() -> Result<(), String> {
        Python::attach(|py| {
            // Locate project root and Q_Layered_Network directory
            let exe = std::env::current_exe().map_err(|e| format!("current_exe failed: {}", e))?;
            let mut path = exe.parent().ok_or("no parent for executable")?.to_path_buf();

            // Step up three levels to reach project root
            for _ in 0..3 {
                if let Some(p) = path.parent() {
                    path = p.to_path_buf();
                }
            }

            let q_layered = path.join("Q_Layered_Network");

            let sys = py.import("sys").map_err(|e| format!("failed to import sys: {}", e))?;
            let sys_path = sys.getattr("path").map_err(|e| format!("sys.path getattr failed: {}", e))?;

            if q_layered.exists() {
                let q_path_str = q_layered.to_string_lossy().to_string();
                sys_path
                    .call_method1("insert", (0, q_path_str.clone()))
                    .map_err(|e| format!("failed to insert into sys.path: {}", e))?;
                println!("✅ Added Python path: {}", q_path_str);
            } else {
                println!("⚠️ Warning: Q_Layered_Network folder not found: {}", q_layered.display());
            }

            let py_mod_name = "dqn_integration";
            let module = py.import(py_mod_name)
                .map_err(|e| format!("Failed to import Python module '{}': {:?}", py_mod_name, e))?;

            if let Ok(init_fn) = module.getattr("init_agent") {
                if let Err(e) = init_fn.call0() {
                    return
