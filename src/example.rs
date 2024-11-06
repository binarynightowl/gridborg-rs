use pyo3::prelude::*;

#[pyfunction]
fn mult_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a * b).to_string())
}

pub fn init(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new_bound(parent_module.py(), "example")?;
    child_module.add_function(wrap_pyfunction!(mult_as_string, &child_module)?)?;
    parent_module.add_submodule(&child_module)
}