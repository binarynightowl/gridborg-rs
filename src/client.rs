use std::net::IpAddr;
use std::str::FromStr;
use pyo3::prelude::*;

pub fn init(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new_bound(parent_module.py(), "client")?;

    child_module.add_class::<GridborgClient>()?;

    // child_module.add_function(wrap_pyfunction!(mult_as_string, &child_module)?)?;

    parent_module.add_submodule(&child_module)
}
#[pyclass]
struct GridborgClient {
    server: IpAddr,
    control_port: u16,
    transport_channel_port: u16,
}

#[pymethods]
impl GridborgClient {
    #[new]
    fn new(server: String, control_port: Option<u16>, transport_channel_port: Option<u16>) -> PyResult<Self> {
        let server = IpAddr::from_str(&server)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Invalid IP address: {}", e)))?;

        let control_port = control_port.unwrap_or(8000); // Example default value
        let transport_channel_port = transport_channel_port.unwrap_or(9000); // Example default value

        Ok(GridborgClient {
            server,
            control_port,
            transport_channel_port,
        })
    }
    fn print_details(&self) {
        println!(
            "GridborgClient(server: {}, control_port: {}, transport_channel_port: {})",
            self.server, self.control_port, self.transport_channel_port
        );
    }
}
