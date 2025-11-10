use dioxus::prelude::*;

pub mod python_integration {
    use pyo3::prelude::*;
    use pyo3::types::PyTuple;

    /// Try to initialize the Python side: fix sys.path and call `init_agent()` if present.
    pub fn init_python_dqn() -> Result<(), String> {
        Python::attach(|py| {
            // Determine a project root candidate based on the running executable
            let exe = std::env::current_exe().map_err(|e| format!("current_exe failed: {}", e))?;
            let mut path = exe.parent().ok_or("no parent for executable")?.to_path_buf();

            // Walk up three levels like your previous logs did, to find project root
            for _ in 0..3 {
                if let Some(p) = path.parent() {
                    path = p.to_path_buf();
                }
            }

            // Append the Q_Layered_Network directory (adjust if your layout differs)
            let mut q_layered = path.clone();
            q_layered.push("Q_Layered_Network");

            // Add to sys.path if it exists
            let sys = py.import("sys").map_err(|e| format!("failed to import sys: {}", e))?;
            let sys_path = sys.getattr("path").map_err(|e| format!("sys.path getattr failed: {}", e))?;

            if q_layered.exists() {
                let q_path_str = q_layered.to_string_lossy().to_string();
                // Insert at front so our module resolves first
                sys_path
                    .call_method1("insert", (0, q_path_str.clone()))
                    .map_err(|e| format!("failed to insert into sys.path: {}", e))?;
                println!("Added to Python path: {}", q_path_str);
            } else {
                println!(
                    "Warning: expected python project dir not found: {} (continuing)",
                    q_layered.display()
                );
            }

            // Try to import your python dqn module
            // Change module name here if your file name is different (e.g., dqn_integration.py)
            let py_mod_name = "dqn_integration";
            match py.import(py_mod_name) {
                Ok(module) => {
                    // If Python defines init_agent(), call it to let the Python side load ONNX session once
                    if let Ok(init_fn) = module.getattr("init_agent") {
                        match init_fn.call0() {
                            Ok(_) => {
                                println!("Python init_agent() called successfully.");
                                Ok(())
                            }
                            Err(e) => Err(format!(
                                "Python init_agent() raised an exception: {:?}",
                                e
                            )),
                        }
                    } else {
                        // No init function; import succeeded — that's fine
                        println!(
                            "Python module '{}' imported successfully (no init_agent detected).",
                            py_mod_name
                        );
                        Ok(())
                    }
                }
                Err(e) => {
                    Err(format!("Failed to import Python module '{}': {:?}", py_mod_name, e))
                }
            }
        })
    }

    /// Call Python's get_dqn_action(state: list[float]) -> int
    pub fn get_dqn_action(state: Vec<f32>) -> Result<i32, String> {
        Python::attach(|py| {
            // Import the module (module should be findable because init_python_dqn inserted path)
            let py_mod_name = "dqn_integration";
            let module = py
                .import(py_mod_name)
                .map_err(|e| format!("Failed to import '{}': {:?}", py_mod_name, e))?;

            // Get the function
            let func = module
                .getattr("get_dqn_action")
                .map_err(|e| format!("Module '{}' has no 'get_dqn_action': {:?}", py_mod_name, e))?;

            // Convert the Rust Vec<f32> into a Python tuple/list
            // Here we pass a tuple, but the Python side should accept list/tuple/numpy-compatible data.
            let py_tuple = PyTuple::new(py, state.iter().map(|f| *f));
            let py_tuple_bound = py_tuple.map_err(|e| format!("Failed to create py_tuple: {:?}", e))?;
            let result = func
                .call1((py_tuple_bound,))
                .map_err(|e| format!("Python call failed: {:?}", e))?;

            // Try to extract an integer as action
            let action: i32 = result
                .extract()
                .map_err(|e| format!("Failed to extract action int from Python result: {:?}", e))?;

            Ok(action)
        })
    }
}


#[component]
pub fn App() -> Element {
    // create a reactive signal
    let status = use_signal(|| "Python DQN not initialized.".to_string());

    // run once asynchronously when the component mounts
    use_future(move || {
        let mut status = status.clone();
        async move {
            match python_integration::init_python_dqn() {
                Ok(_) => status.set("✅ Python DQN initialized successfully.".to_string()),
                Err(e) => status.set(format!("❌ Python initialization failed: {:?}", e)),
            }
        }
    });

    rsx! {
        div {
            style { "padding:20px; color:white; background:#0d0d0d; height:100vh; font-family:sans-serif;" }
            h1 { "Deadlock DQN Integration" }
            p { "{status}" }
            button {
                onclick: move |_| {
                    let mut status = status.clone();
                    spawn(async move {
                        let dummy_state = vec![0.1_f32, 0.2, 0.3];
                        match python_integration::get_dqn_action(dummy_state) {
                            Ok(action) => status.set(format!("🧠 DQN Action: {}", action)),
                            Err(e) => status.set(format!("⚠️ Python error: {:?}", e)),
                        }
                    });
                },
                "Run DQN Action"
            }
        }
    }
}
