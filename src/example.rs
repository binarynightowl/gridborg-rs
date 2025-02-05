use pyo3::prelude::*;
use std::sync::mpsc;
use std::thread;

#[pyfunction]
fn mult_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a * b).to_string())
}

#[pyfunction]
fn foo() -> PyResult<String> {
    // Create a channel for communication
    let (tx, rx) = mpsc::channel();

    // Spawn a new thread
    thread::spawn(move || {
        let message = "Hello from thread!".to_string();
        let _ = tx.send(message); // Send the message
    });

    // Receive the message
    let received_message = rx.recv().map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Channel receive error: {}", e))
    })?;

    Ok(received_message)
}

pub fn init(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new_bound(parent_module.py(), "example")?;
    child_module.add_function(wrap_pyfunction!(mult_as_string, &child_module)?)?;
    child_module.add_function(wrap_pyfunction!(foo, &child_module)?)?;
    parent_module.add_submodule(&child_module)
}
