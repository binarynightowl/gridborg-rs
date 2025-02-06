use pyo3::prelude::*;
use std::io::{BufRead, BufReader, Write};
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::thread;

pub fn init(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new_bound(parent_module.py(), "client")?;

    child_module.add_class::<GridborgClient>()?;

    child_module.add_function(wrap_pyfunction!(sum_as_string, &child_module)?)?;

    parent_module.add_submodule(&child_module)
}

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyclass]
struct GridborgClient {
    server: IpAddr,
    control_port: u16,
    transport_channel_port: u16,
    username: String,
    password: String,
    socket: Option<TcpStream>,
    reader: Option<BufReader<TcpStream>>,
    command_tag: u64,
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
            reader: None,
            command_tag: 0,
        })
    }

    fn connect(&mut self) -> PyResult<()> {
        let addr = SocketAddr::new(self.server, self.control_port);
        match TcpStream::connect(addr) {
            Ok(stream) => {
                stream.set_read_timeout(None).ok();
                stream.set_write_timeout(None).ok();

                let reader = BufReader::new(stream.try_clone()?);
                self.socket = Some(stream);

                thread::spawn(move || {
                    let mut reader = reader;
                    let mut line = String::new();

                    while let Ok(bytes_read) = reader.read_line(&mut line) {
                        if bytes_read == 0 {
                            break;
                        }
                        println!("Received: {}", line.trim());
                        line.clear();
                    }
                    println!("Connection closed.");
                });

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

    fn send_raw_command(&mut self, message: String) -> PyResult<u64> {
        if let Some(ref mut stream) = self.socket {
            let msg = format!("{} COMMANDTAG={}\n", message, self.command_tag);
            let msg_bytes = msg.into_bytes();
            stream.write_all(&msg_bytes).map_err(|e| {
                pyo3::exceptions::PyIOError::new_err(format!("Failed to send message: {e}"))
            })?;
            let tag = self.command_tag;
            self.command_tag += 1;
            Ok(tag)
        } else {
            Err(pyo3::exceptions::PyRuntimeError::new_err(
                "No active connection to send message",
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
