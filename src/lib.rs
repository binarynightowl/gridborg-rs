mod client;
mod commands;
mod primitives;
mod constants;
mod macros;
mod events;

use pyo3::prelude::*;

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
pub fn gridborg_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?);
    client::init(m)?;
    commands::init(m)?;
    Ok(())
}
