use pyo3::prelude::*;
use std::io::{BufRead, BufReader, Write};
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::thread;

use crate::commands::{Command, CommandHandler, GetVersion, Logout, ProtocolVersion, Quit};

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
    #[pyo3(get)]
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

    fn send_command(&mut self, command: Command) -> PyResult<u64> {
        self.send_raw_command(command.into())
    }

    // Product Information Commands
    fn get_version(&mut self) -> PyResult<()> {
        CommandHandler::get_version(self)
    }

    fn get_protocol_version(&mut self) -> PyResult<()> {
        CommandHandler::get_protocol_version(self)
    }

    // Session Commands
    fn login(&mut self) -> PyResult<()> {
        CommandHandler::login(self)
    }

    fn logout(&mut self) -> PyResult<()> {
        CommandHandler::logout(self)
    }

    fn quit(&mut self) -> PyResult<()> {
        CommandHandler::quit(self)
    }

    fn resource_create_frontend(
        &mut self,
        reg_incoming_ani: Option<String>,
        reg_incoming_dnis: Option<String>,
        reg_incoming_rdn: Option<String>,
        accepting: Option<bool>,
    ) -> PyResult<()> {
        CommandHandler::resource_create_frontend(
            self,
            reg_incoming_ani,
            reg_incoming_dnis,
            reg_incoming_rdn,
            accepting,
        )
    }

    fn resource_create_player(&mut self) -> PyResult<()> {
        CommandHandler::resource_create_player(self)
    }

    fn resource_create_recorder(&mut self) -> PyResult<()> {
        CommandHandler::resource_create_recorder(self)
    }

    fn resource_create_transport_channel(&mut self, transport_type: String) -> PyResult<()> {
        CommandHandler::resource_create_transport_channel(self, transport_type)
    }

    fn resource_create_rtp_channel(&mut self, in_band_dtmf_enabled: Option<bool>) -> PyResult<()> {
        CommandHandler::resource_create_rtp_channel(self, in_band_dtmf_enabled)
    }

    fn resource_create_sound_device(
        &mut self,
        direction: String,
        device: Option<String>,
        buffers: Option<u8>,
    ) -> PyResult<()> {
        CommandHandler::resource_create_sound_device(self, direction, device, buffers)
    }

    fn resource_create_fax(&mut self) -> PyResult<()> {
        CommandHandler::resource_create_fax(self)
    }

    fn resource_create_document(&mut self) -> PyResult<()> {
        CommandHandler::resource_create_document(self)
    }

    fn resource_delete(&mut self, resource_id: u32) -> PyResult<()> {
        CommandHandler::resource_delete(self, resource_id)
    }

    fn resource_get_status(&mut self, resource_id: u32) -> PyResult<()> {
        CommandHandler::resource_get_status(self, resource_id)
    }

    fn print_details(&self) {
        println!(
            "GridborgClient(server: {}, control_port: {}, transport_channel_port: {}, username: {}, password: {})",
            self.server, self.control_port, self.transport_channel_port, self.username, self.password
        );
    }
}

impl CommandHandler for GridborgClient {
    // Product Information Commands
    fn get_version(&mut self) -> PyResult<()> {
        self.send_command(Command::get_version())
            .expect("TODO: panic message");
        Ok(())
    }

    fn get_protocol_version(&mut self) -> PyResult<()> {
        self.send_command(Command::protocol_version())
            .expect("TODO: panic message");
        Ok(())
    }

    // Session Commands
    fn login(&mut self) -> PyResult<()> {
        self.send_command(Command::login(
            self.username.clone(),
            self.password.clone(),
            None,
            None,
            None,
        ))
        .expect("TODO: panic message");
        Ok(())
    }

    fn logout(&mut self) -> PyResult<()> {
        self.send_command(Command::logout())
            .expect("TODO: panic message");
        Ok(())
    }

    fn quit(&mut self) -> PyResult<()> {
        self.send_command(Command::quit())
            .expect("TODO: panic message");
        Ok(())
    }

    // General Resource Commands
    fn resource_create_frontend(
        &mut self,
        reg_incoming_ani: Option<String>,
        reg_incoming_dnis: Option<String>,
        reg_incoming_rdn: Option<String>,
        accepting: Option<bool>,
    ) -> PyResult<()> {
        self.send_command(Command::resource_create_frontend(
            reg_incoming_ani,
            reg_incoming_dnis,
            reg_incoming_rdn,
            accepting,
        ))
        .expect("TODO: panic message");
        Ok(())
    }

    fn resource_create_player(&mut self) -> PyResult<()> {
        self.send_command(Command::resource_create_player())
            .expect("TODO: panic message");
        Ok(())
    }

    fn resource_create_recorder(&mut self) -> PyResult<()> {
        self.send_command(Command::resource_create_recorder())
            .expect("TODO: panic message");
        Ok(())
    }

    fn resource_create_transport_channel(&mut self, transport_type: String) -> PyResult<()> {
        self.send_command(Command::resource_create_transport_channel(transport_type))
            .expect("TODO: panic message");
        Ok(())
    }

    fn resource_create_rtp_channel(&mut self, in_band_dtmf_enabled: Option<bool>) -> PyResult<()> {
        self.send_command(Command::resource_create_rtp_channel(in_band_dtmf_enabled))
            .expect("TODO: panic message");
        Ok(())
    }

    fn resource_create_sound_device(
        &mut self,
        direction: String,
        device: Option<String>,
        buffers: Option<u8>,
    ) -> PyResult<()> {
        self.send_command(Command::resource_create_sound_device(
            direction, device, buffers,
        ))
        .expect("TODO: panic message");
        Ok(())
    }

    fn resource_create_fax(&mut self) -> PyResult<()> {
        self.send_command(Command::resource_create_fax())
            .expect("TODO: panic message");
        Ok(())
    }

    fn resource_create_document(&mut self) -> PyResult<()> {
        self.send_command(Command::resource_create_fax())
            .expect("TODO: panic message");
        Ok(())
    }

    fn resource_delete(&mut self, resource_id: u32) -> PyResult<()> {
        self.send_command(Command::resource_delete(resource_id))
            .expect("TODO: panic message");
        Ok(())
    }

    fn resource_get_status(&mut self, resource_id: u32) -> PyResult<()> {
        self.send_command(Command::resource_get_status(resource_id))
            .expect("TODO: panic message");
        Ok(())
    }
}
