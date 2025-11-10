use pyo3::prelude::*;
use pyo3::types::PyList;
use std::sync::Once;

static INIT: Once = Once::new();

pub fn init_python_dqn() -> PyResult<()> {
    INIT.call_once(|| {
        pyo3::prepare_freethreaded_python();
    });

    Python::with_gil(|py| {
        let sys = PyModule::import(py, "sys")?;
        let path = sys.getattr("path")?;
        path.call_method1("insert", (0, "."))?;

        let py_mod = PyModule::import(py, "python_integration")?;
        if let Ok(init_fn) = py_mod.getattr("init_dqn") {
            init_fn.call0()?;
        }
        Ok(())
    })
}

pub fn get_dqn_action(state: Vec<f64>) -> PyResult<f64> {
    Python::with_gil(|py| {
        let py_mod = PyModule::import(py, "python_integration")?;
        let get_action = py_mod.getattr("get_dqn_action")?;

        // FIX: unwrap the PyList result before call1
        let py_list = PyList::new(py, &state);
        let result = get_action.call1((py_list,))?;
        result.extract::<f64>()
    })
}
