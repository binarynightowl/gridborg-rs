use pyo3::prelude::*;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::time::Duration;

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
    username: String,
    password: String,
    socket: Option<TcpStream>,
}

#[pymethods]
impl GridborgClient {
    #[new]
    fn new(
        server: String,
        control_port: Option<u16>,
        transport_channel_port: Option<u16>,
        username: Option<String>,
        password: Option<String>,
    ) -> PyResult<Self> {
        let server = IpAddr::from_str(&server).map_err(|_| {
            pyo3::exceptions::PyValueError::new_err(format!("Invalid IP address: {}", server))
        })?;

        let control_port = control_port.unwrap_or(1234);
        let transport_channel_port = transport_channel_port.unwrap_or(1235);
        let username = username.unwrap_or("user1".to_string());
        let password = password.unwrap_or("abc".to_string());

        Ok(GridborgClient {
            server,
            control_port,
            transport_channel_port,
            username,
            password,
            socket: None,
        })
    }

    fn connect(&mut self) -> PyResult<()> {
        let addr = SocketAddr::new(self.server, self.control_port);
        match TcpStream::connect(addr) {
            Ok(stream) => {
                stream.set_read_timeout(Some(Duration::from_secs(5))).ok();
                stream.set_write_timeout(Some(Duration::from_secs(5))).ok();
                self.socket = Some(stream);
                Ok(())
            }
            Err(e) => Err(pyo3::exceptions::PyIOError::new_err(format!(
                "Failed to connect: {e}"
            ))),
        }
    }

    fn disconnect(&mut self) -> PyResult<()> {
        if self.socket.is_some() {
            self.socket = None;
            Ok(())
        } else {
            Err(pyo3::exceptions::PyRuntimeError::new_err(
                "No active connection to disconnect",
            ))
        }
    }

    fn print_details(&self) {
        println!(
            "GridborgClient(server: {}, control_port: {}, transport_channel_port: {}, username: {}, password: {})",
            self.server, self.control_port, self.transport_channel_port, self.username, self.password
        );
    }
}
