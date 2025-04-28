use pyo3::prelude::*;
use std::io::{BufRead, BufReader, Write};
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::thread;

use crate::commands::{Command, CommandHandler};
use crate::constants::{
    AudioFormatType, DocumentAddFileTransformation, DocumentPreparePaperSize,
    DocumentPrepareResolution, DocumentSaveType, FaxReceiveMode, FaxSendSpeed, PayloadType,
    ToneType,
};
use crate::primitives::{Channels, ResourceId, SampleRate, ECM};

pub fn init(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new(parent_module.py(), "client")?;

    child_module.add_class::<GridborgClient>()?;

    // child_module.add_function(wrap_pyfunction!(sum_as_string, &child_module)?)?;

    parent_module.add_submodule(&child_module)
}

// #[pyfunction]
// fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }

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

    // General Resource Commands
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

    fn resource_delete(&mut self, resource_id: ResourceId) -> PyResult<()> {
        CommandHandler::resource_delete(self, resource_id)
    }

    fn resource_get_status(&mut self, resource_id: ResourceId) -> PyResult<()> {
        CommandHandler::resource_get_status(self, resource_id)
    }

    // Front-end Resource Commands
    fn call_make(
        &mut self,
        resource_id: ResourceId,
        address: String,
        timeout: Option<u32>,
        caller_number: Option<String>,
        caller_name: Option<String>,
        privacy: Option<u8>,
        screen: Option<u8>,
    ) -> PyResult<()> {
        CommandHandler::call_make(
            self,
            resource_id,
            address,
            timeout,
            caller_number,
            caller_name,
            privacy,
            screen,
        )
    }

    fn call_answer(&mut self, resource_id: ResourceId) -> PyResult<()> {
        CommandHandler::call_answer(self, resource_id)
    }

    fn call_clear(&mut self, resource_id: ResourceId, reason: Option<String>) -> PyResult<()> {
        CommandHandler::call_clear(self, resource_id, reason)
    }

    fn call_transfer_consultation(&mut self, resource_id1: u32, resource_id2: u32) -> PyResult<()> {
        CommandHandler::call_transfer_consultation(self, resource_id1, resource_id2)
    }

    fn call_transfer_blind(
        &mut self,
        resource_id: ResourceId,
        address: String,
        use_h450: Option<u8>,
    ) -> PyResult<()> {
        CommandHandler::call_transfer_blind(self, resource_id, address, use_h450)
    }

    fn call_hold(&mut self, resource_id: ResourceId) -> PyResult<()> {
        CommandHandler::call_hold(self, resource_id)
    }

    fn call_retrieve(&mut self, resource_id: ResourceId) -> PyResult<()> {
        CommandHandler::call_retrieve(self, resource_id)
    }

    fn call_send_dtmf(
        &mut self,
        resource_id: ResourceId,
        dtmf_string: String,
        duration: Option<u32>,
        delay: Option<u32>,
        pause_duration: Option<u32>,
    ) -> PyResult<()> {
        CommandHandler::call_send_dtmf(
            self,
            resource_id,
            dtmf_string,
            duration,
            delay,
            pause_duration,
        )
    }

    fn call_stop_activity(&mut self, resource_id: ResourceId) -> PyResult<()> {
        CommandHandler::call_stop_activity(self, resource_id)
    }

    fn call_t38_relay(&mut self, resource_id1: u32, resource_id2: u32) -> PyResult<()> {
        CommandHandler::call_t38_relay(self, resource_id1, resource_id2)
    }

    fn calls_set_alerting_type(
        &mut self,
        resource_id: ResourceId,
        alerting_type: String,
    ) -> PyResult<()> {
        CommandHandler::calls_set_alerting_type(self, resource_id, alerting_type)
    }

    fn calls_set_accepting(&mut self, resource_id: ResourceId, accepting: bool) -> PyResult<()> {
        CommandHandler::calls_set_accepting(self, resource_id, accepting)
    }

    // Player Resource Commands
    fn play_file(
        &mut self,
        resource_id: ResourceId,
        file_name: String,
        audio_type: Option<AudioFormatType>,
        sample_rate: Option<SampleRate>,
        channels: Option<Channels>,
        index: Option<u32>,
        skip_bytes: Option<i64>,
    ) -> PyResult<()> {
        CommandHandler::play_file(
            self,
            resource_id,
            file_name,
            audio_type,
            sample_rate,
            channels,
            index,
            skip_bytes,
        )
    }

    fn play_stream(
        &mut self,
        player_id: ResourceId,
        transport_channel_id: ResourceId,
        audio_type: Option<AudioFormatType>,
        sample_rate: Option<SampleRate>,
        buffer_optimum_size: Option<u32>,
    ) -> PyResult<()> {
        CommandHandler::play_stream(
            self,
            player_id,
            transport_channel_id,
            audio_type,
            sample_rate,
            buffer_optimum_size,
        )
    }

    fn play_tone(
        &mut self,
        resource_id: ResourceId,
        frequency: Option<u16>,
        frequency2: Option<u16>,
        tone: Option<ToneType>,
        volume: Option<u8>,
        duration: Option<u16>,
    ) -> PyResult<()> {
        CommandHandler::play_tone(
            self,
            resource_id,
            frequency,
            frequency2,
            tone,
            volume,
            duration,
        )
    }

    fn play_stop(&mut self, resource_id: ResourceId) -> PyResult<()> {
        CommandHandler::play_stop(self, resource_id)
    }

    // Recorder Resource Commands
    fn recorder_start_to_file(
        &mut self,
        resource_id: ResourceId,
        file_name: String,
        audio_type: Option<AudioFormatType>,
        sample_rate: Option<SampleRate>,
        channels: Option<Channels>,
        file_offset: Option<i64>,
        max_duration: Option<u32>,
        max_silence: Option<u32>,
        voice_trigger: Option<bool>,
        pause_if_empty: Option<bool>,
    ) -> PyResult<()> {
        CommandHandler::recorder_start_to_file(
            self,
            resource_id,
            file_name,
            audio_type,
            sample_rate,
            channels,
            file_offset,
            max_duration,
            max_silence,
            voice_trigger,
            pause_if_empty,
        )
    }

    fn recorder_start_to_stream(
        &mut self,
        recorder_id: ResourceId,
        transport_channel_id: ResourceId,
        audio_type: Option<AudioFormatType>,
        sample_rate: Option<SampleRate>,
        max_duration: Option<u32>,
        max_silence: Option<u32>,
        voice_trigger: Option<bool>,
        pause_if_empty: Option<bool>,
    ) -> PyResult<()> {
        CommandHandler::recorder_start_to_stream(
            self,
            recorder_id,
            transport_channel_id,
            audio_type,
            sample_rate,
            max_duration,
            max_silence,
            voice_trigger,
            pause_if_empty,
        )
    }

    fn recorder_stop(&mut self, resource_id: ResourceId) -> PyResult<()> {
        CommandHandler::recorder_stop(self, resource_id)
    }

    // RTP Channel Resource Commands
    fn rtp_channel_start_receiving(
        &mut self,
        resource_id: ResourceId,
        sender_control_address: Option<String>,
        receiver_data_address: Option<String>,
        receiver_control_address: Option<String>,
        payload_type: Option<PayloadType>,
        rfc2833_payload_type: Option<u8>,
        rtp_session_id: Option<u8>,
        jitter_buffer_length_min: Option<u16>,
        jitter_buffer_length_max: Option<u16>,
    ) -> PyResult<()> {
        CommandHandler::rtp_channel_start_receiving(
            self,
            resource_id,
            sender_control_address,
            receiver_data_address,
            receiver_control_address,
            payload_type,
            rfc2833_payload_type,
            rtp_session_id,
            jitter_buffer_length_min,
            jitter_buffer_length_max,
        )
    }

    fn rtp_channel_start_sending(
        &mut self,
        resource_id: ResourceId,
        receiver_data_address: String,
        receiver_control_address: Option<String>,
        sender_data_address: Option<String>,
        sender_control_address: Option<String>,
        payload_type: Option<PayloadType>,
        rfc2833_payload_type: Option<u8>,
        rtp_session_id: Option<u8>,
    ) -> PyResult<()> {
        CommandHandler::rtp_channel_start_sending(
            self,
            resource_id,
            receiver_data_address,
            receiver_control_address,
            sender_data_address,
            sender_control_address,
            payload_type,
            rfc2833_payload_type,
            rtp_session_id,
        )
    }

    fn rtp_channel_stop(&mut self, resource_id: ResourceId) -> PyResult<()> {
        CommandHandler::rtp_channel_stop(self, resource_id)
    }

    fn rtp_channel_send_dtmf(
        &mut self,
        resource_id: ResourceId,
        dtmf_string: String,
        duration: Option<u32>,
        delay: Option<u32>,
        pause_duration: Option<u32>,
    ) -> PyResult<()> {
        CommandHandler::rtp_channel_send_dtmf(
            self,
            resource_id,
            dtmf_string,
            duration,
            delay,
            pause_duration,
        )
    }

    // Sound device Resource Commands
    fn sound_device_start(&mut self, resource_id: ResourceId) -> PyResult<()> {
        CommandHandler::sound_device_start(self, resource_id)
    }

    fn sound_device_stop(&mut self, resource_id: ResourceId) -> PyResult<()> {
        CommandHandler::sound_device_stop(self, resource_id)
    }

    // Fax Resource Commands
    fn fax_receive(
        &mut self,
        fax_resource_id: ResourceId,
        frontend_resource_id: ResourceId,
        document_resource_id: ResourceId,
        fax_mode: Option<FaxReceiveMode>,
        use_ecm: Option<ECM>,
        csi: Option<String>,
    ) -> PyResult<()> {
        CommandHandler::fax_receive(self, fax_resource_id, frontend_resource_id, document_resource_id, fax_mode, use_ecm, csi)
    }

    fn fax_send(
        &mut self,
        fax_resource_id: ResourceId,
        frontend_resource_id: ResourceId,
        document_resource_id: ResourceId,
        speed: Option<FaxSendSpeed>,
        use_ecm: Option<ECM>,
        header: Option<String>,
        tsi: Option<String>,
    ) -> PyResult<()> {
        CommandHandler::fax_send(self, fax_resource_id, frontend_resource_id, document_resource_id, speed, use_ecm, header, tsi)
    }

    fn fax_abort(&mut self, resource_id: ResourceId) -> PyResult<()> {
        CommandHandler::fax_abort(self, resource_id)
    }

    // Document Resource Commands
    fn document_add_file(
        &mut self,
        resource_id: ResourceId,
        file_path: String,
        transformation: Option<DocumentAddFileTransformation>,
    ) -> PyResult<()> {
        CommandHandler::document_add_file(self, resource_id, file_path, transformation)
    }

    fn document_prepare(
        &mut self,
        resource_id: ResourceId,
        paper_size: Option<DocumentPreparePaperSize>,
        resolution: Option<DocumentPrepareResolution>,
    ) -> PyResult<()> {
        CommandHandler::document_prepare(self, resource_id, paper_size, resolution)
    }

    fn document_save(
        &mut self,
        resource_id: ResourceId,
        file_path: String,
        multipage: Option<bool>,
        document_type: Option<DocumentSaveType>,
    ) -> PyResult<()> {
        CommandHandler::document_save(self, resource_id, file_path, multipage, document_type)
    }

    fn document_clear(&mut self, resource_id: ResourceId) -> PyResult<()> {
        CommandHandler::document_clear(self, resource_id)
    }

    // Audio Routing and Audio Stream Monitoring Commands
    fn audio_send(
        &mut self,
        source_resource_id: ResourceId,
        sink_resource_id: ResourceId,
        source_channel: Option<u8>,
        sink_channel: Option<u8>,
        volume: Option<i16>,
        auto_gain: Option<bool>,
        auto_gain_resolution: Option<u16>,
        auto_gain_rise_time: Option<u16>,
        auto_gain_fall_time: Option<u16>,
        auto_gain_kill_time: Option<u16>,
    ) -> PyResult<()> {
        CommandHandler::audio_send(self, source_resource_id, sink_resource_id, source_channel, sink_channel, volume, auto_gain, auto_gain_resolution, auto_gain_rise_time, auto_gain_fall_time, auto_gain_kill_time)
    }

    fn audio_cancel(
        &mut self,
        source_resource_id: ResourceId,
        sink_resource_id: ResourceId,
    ) -> PyResult<()> {
        CommandHandler::audio_cancel(self, source_resource_id, sink_resource_id)
    }

    fn audio_level_notification_send(
        &mut self,
        resource_id: ResourceId,
        resolution: Option<u16>,
        voice_dead_band: Option<u16>,
        silence_dead_band: Option<u16>,
        adaptive_period: Option<u16>,
        voice_timer: Option<u16>,
        silence_timer: Option<u16>,
    ) -> PyResult<()> {
        CommandHandler::audio_level_notification_send(self, resource_id, resolution, voice_dead_band, silence_dead_band, adaptive_period, voice_timer, silence_timer)
    }

    fn audio_level_notification_cancel(&mut self, resource_id: ResourceId) -> PyResult<()> {
        CommandHandler::audio_level_notification_cancel(self, resource_id)
    }

    fn in_band_signaling_detection_enable(&mut self, resource_id: ResourceId) -> PyResult<()> {
        CommandHandler::in_band_signaling_detection_enable(self, resource_id)
    }

    fn in_band_signaling_detection_disable(&mut self, resource_id: ResourceId) -> PyResult<()> {
        CommandHandler::in_band_signaling_detection_disable(self, resource_id)
    }

    // Miscellaneous Commands
    fn get_rtp_statistics(&mut self, resource_id: ResourceId) -> PyResult<()> {
        CommandHandler::get_rtp_statistics(self, resource_id)
    }

    // Not Commands
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

    fn resource_delete(&mut self, resource_id: ResourceId) -> PyResult<()> {
        self.send_command(Command::resource_delete(resource_id))
            .expect("TODO: panic message");
        Ok(())
    }

    fn resource_get_status(&mut self, resource_id: ResourceId) -> PyResult<()> {
        self.send_command(Command::resource_get_status(resource_id))
            .expect("TODO: panic message");
        Ok(())
    }

    // Front-end Resource Commands
    fn call_make(
        &mut self,
        resource_id: ResourceId,
        address: String,
        timeout: Option<u32>,
        caller_number: Option<String>,
        caller_name: Option<String>,
        privacy: Option<u8>,
        screen: Option<u8>,
    ) -> PyResult<()> {
        self.send_command(Command::call_make(
            resource_id,
            address,
            timeout,
            caller_number,
            caller_name,
            privacy,
            screen,
        ))
        .expect("call_make failed");
        Ok(())
    }

    fn call_answer(&mut self, resource_id: ResourceId) -> PyResult<()> {
        self.send_command(Command::call_answer(resource_id))
            .expect("call_answer failed");
        Ok(())
    }

    fn call_clear(&mut self, resource_id: ResourceId, reason: Option<String>) -> PyResult<()> {
        self.send_command(Command::call_clear(resource_id, reason))
            .expect("call_clear failed");
        Ok(())
    }

    fn call_transfer_consultation(&mut self, resource_id1: u32, resource_id2: u32) -> PyResult<()> {
        self.send_command(Command::call_transfer_consultation(
            resource_id1,
            resource_id2,
        ))
        .expect("call_transfer_consultation failed");
        Ok(())
    }

    fn call_transfer_blind(
        &mut self,
        resource_id: ResourceId,
        address: String,
        use_h450: Option<u8>,
    ) -> PyResult<()> {
        self.send_command(Command::call_transfer_blind(resource_id, address, use_h450))
            .expect("call_transfer_blind failed");
        Ok(())
    }

    fn call_hold(&mut self, resource_id: ResourceId) -> PyResult<()> {
        self.send_command(Command::call_hold(resource_id))
            .expect("call_hold failed");
        Ok(())
    }

    fn call_retrieve(&mut self, resource_id: ResourceId) -> PyResult<()> {
        self.send_command(Command::call_retrieve(resource_id))
            .expect("call_retrieve failed");
        Ok(())
    }

    fn call_send_dtmf(
        &mut self,
        resource_id: ResourceId,
        dtmf_string: String,
        duration: Option<u32>,
        delay: Option<u32>,
        pause_duration: Option<u32>,
    ) -> PyResult<()> {
        self.send_command(Command::call_send_dtmf(
            resource_id,
            dtmf_string,
            duration,
            delay,
            pause_duration,
        ))
        .expect("call_send_dtmf failed");
        Ok(())
    }

    fn call_stop_activity(&mut self, resource_id: ResourceId) -> PyResult<()> {
        self.send_command(Command::call_stop_activity(resource_id))
            .expect("call_stop_activity failed");
        Ok(())
    }

    fn call_t38_relay(&mut self, resource_id1: u32, resource_id2: u32) -> PyResult<()> {
        self.send_command(Command::call_t38_relay(resource_id1, resource_id2))
            .expect("call_t38_relay failed");
        Ok(())
    }

    fn calls_set_alerting_type(
        &mut self,
        resource_id: ResourceId,
        alerting_type: String,
    ) -> PyResult<()> {
        self.send_command(Command::calls_set_alerting_type(resource_id, alerting_type))
            .expect("calls_set_alerting_type failed");
        Ok(())
    }

    fn calls_set_accepting(&mut self, resource_id: ResourceId, accepting: bool) -> PyResult<()> {
        self.send_command(Command::calls_set_accepting(resource_id, accepting))
            .expect("calls_set_accepting failed");
        Ok(())
    }

    // Player Resource Commands
    fn play_file(
        &mut self,
        resource_id: ResourceId,
        file_name: String,
        audio_type: Option<AudioFormatType>,
        sample_rate: Option<SampleRate>,
        channels: Option<Channels>,
        index: Option<u32>,
        skip_bytes: Option<i64>,
    ) -> PyResult<()> {
        self.send_command(Command::play_file(
            resource_id,
            file_name,
            audio_type,
            sample_rate,
            channels,
            index,
            skip_bytes,
        ))?;
        Ok(())
    }

    fn play_stream(
        &mut self,
        player_id: ResourceId,
        transport_channel_id: ResourceId,
        audio_type: Option<AudioFormatType>,
        sample_rate: Option<SampleRate>,
        buffer_optimum_size: Option<u32>,
    ) -> PyResult<()> {
        self.send_command(Command::play_stream(
            player_id,
            transport_channel_id,
            audio_type,
            sample_rate,
            buffer_optimum_size,
        ))?;
        Ok(())
    }

    fn play_tone(
        &mut self,
        resource_id: ResourceId,
        frequency: Option<u16>,
        frequency2: Option<u16>,
        tone: Option<ToneType>,
        volume: Option<u8>,
        duration: Option<u16>,
    ) -> PyResult<()> {
        self.send_command(Command::play_tone(
            resource_id,
            frequency,
            frequency2,
            tone,
            volume,
            duration,
        ))?;
        Ok(())
    }

    fn play_stop(&mut self, resource_id: ResourceId) -> PyResult<()> {
        self.send_command(Command::play_stop(resource_id))?;
        Ok(())
    }

    // Recorder Resource Commands
    fn recorder_start_to_file(
        &mut self,
        resource_id: ResourceId,
        file_name: String,
        audio_type: Option<AudioFormatType>,
        sample_rate: Option<SampleRate>,
        channels: Option<Channels>,
        file_offset: Option<i64>,
        max_duration: Option<u32>,
        max_silence: Option<u32>,
        voice_trigger: Option<bool>,
        pause_if_empty: Option<bool>,
    ) -> PyResult<()> {
        self.send_command(Command::recorder_start_to_file(
            resource_id,
            file_name,
            audio_type,
            sample_rate,
            channels,
            file_offset,
            max_duration,
            max_silence,
            voice_trigger,
            pause_if_empty,
        ))?;
        Ok(())
    }

    fn recorder_start_to_stream(
        &mut self,
        recorder_id: ResourceId,
        transport_channel_id: ResourceId,
        audio_type: Option<AudioFormatType>,
        sample_rate: Option<SampleRate>,
        max_duration: Option<u32>,
        max_silence: Option<u32>,
        voice_trigger: Option<bool>,
        pause_if_empty: Option<bool>,
    ) -> PyResult<()> {
        self.send_command(Command::recorder_start_to_stream(
            recorder_id,
            transport_channel_id,
            audio_type,
            sample_rate,
            max_duration,
            max_silence,
            voice_trigger,
            pause_if_empty,
        ))?;
        Ok(())
    }

    fn recorder_stop(&mut self, resource_id: ResourceId) -> PyResult<()> {
        self.send_command(Command::recorder_stop(resource_id))?;
        Ok(())
    }

    // RTP Channel Resource Commands
    fn rtp_channel_start_receiving(
        &mut self,
        resource_id: ResourceId,
        sender_control_address: Option<String>,
        receiver_data_address: Option<String>,
        receiver_control_address: Option<String>,
        payload_type: Option<PayloadType>,
        rfc2833_payload_type: Option<u8>,
        rtp_session_id: Option<u8>,
        jitter_buffer_length_min: Option<u16>,
        jitter_buffer_length_max: Option<u16>,
    ) -> PyResult<()> {
        self.send_command(Command::rtp_channel_start_receiving(
            resource_id,
            sender_control_address,
            receiver_data_address,
            receiver_control_address,
            payload_type,
            rfc2833_payload_type,
            rtp_session_id,
            jitter_buffer_length_min,
            jitter_buffer_length_max,
        ))?;
        Ok(())
    }

    fn rtp_channel_start_sending(
        &mut self,
        resource_id: ResourceId,
        receiver_data_address: String,
        receiver_control_address: Option<String>,
        sender_data_address: Option<String>,
        sender_control_address: Option<String>,
        payload_type: Option<PayloadType>,
        rfc2833_payload_type: Option<u8>,
        rtp_session_id: Option<u8>,
    ) -> PyResult<()> {
        self.send_command(Command::rtp_channel_start_sending(
            resource_id,
            receiver_data_address,
            receiver_control_address,
            sender_data_address,
            sender_control_address,
            payload_type,
            rfc2833_payload_type,
            rtp_session_id,
        ))?;
        Ok(())
    }

    fn rtp_channel_stop(&mut self, resource_id: ResourceId) -> PyResult<()> {
        self.send_command(Command::rtp_channel_stop(resource_id))?;
        Ok(())
    }

    fn rtp_channel_send_dtmf(
        &mut self,
        resource_id: ResourceId,
        dtmf_string: String,
        duration: Option<u32>,
        delay: Option<u32>,
        pause_duration: Option<u32>,
    ) -> PyResult<()> {
        self.send_command(Command::rtp_channel_send_dtmf(
            resource_id,
            dtmf_string,
            duration,
            delay,
            pause_duration,
        ))?;
        Ok(())
    }

    // Sound device Resource Commands
    fn sound_device_start(&mut self, resource_id: ResourceId) -> PyResult<()> {
        self.send_command(Command::sound_device_start(resource_id))?;
        Ok(())
    }

    fn sound_device_stop(&mut self, resource_id: ResourceId) -> PyResult<()> {
        self.send_command(Command::sound_device_stop(resource_id))?;
        Ok(())
    }

    // Fax Resource Commands
    fn fax_receive(
        &mut self,
        fax_resource_id: ResourceId,
        frontend_resource_id: ResourceId,
        document_resource_id: ResourceId,
        fax_mode: Option<FaxReceiveMode>,
        use_ecm: Option<ECM>,
        csi: Option<String>,
    ) -> PyResult<()> {
        self.send_command(Command::fax_receive(
            fax_resource_id,
            frontend_resource_id,
            document_resource_id,
            fax_mode,
            use_ecm,
            csi,
        ))?;
        Ok(())
    }

    fn fax_send(
        &mut self,
        fax_resource_id: ResourceId,
        frontend_resource_id: ResourceId,
        document_resource_id: ResourceId,
        speed: Option<FaxSendSpeed>,
        use_ecm: Option<ECM>,
        header: Option<String>,
        tsi: Option<String>,
    ) -> PyResult<()> {
        self.send_command(Command::fax_send(
            fax_resource_id,
            frontend_resource_id,
            document_resource_id,
            speed,
            use_ecm,
            header,
            tsi,
        ))?;
        Ok(())
    }

    fn fax_abort(&mut self, resource_id: ResourceId) -> PyResult<()> {
        self.send_command(Command::fax_abort(resource_id))?;
        Ok(())
    }

    // Document Resource Commands
    fn document_add_file(
        &mut self,
        resource_id: ResourceId,
        file_path: String,
        transformation: Option<DocumentAddFileTransformation>,
    ) -> PyResult<()> {
        self.send_command(Command::document_add_file(
            resource_id,
            file_path,
            transformation,
        ))?;
        Ok(())
    }

    fn document_prepare(
        &mut self,
        resource_id: ResourceId,
        paper_size: Option<DocumentPreparePaperSize>,
        resolution: Option<DocumentPrepareResolution>,
    ) -> PyResult<()> {
        self.send_command(Command::document_prepare(
            resource_id,
            paper_size,
            resolution,
        ))?;
        Ok(())
    }

    fn document_save(
        &mut self,
        resource_id: ResourceId,
        file_path: String,
        multipage: Option<bool>,
        document_type: Option<DocumentSaveType>,
    ) -> PyResult<()> {
        self.send_command(Command::document_save(
            resource_id,
            file_path,
            multipage,
            document_type,
        ))?;
        Ok(())
    }

    fn document_clear(&mut self, resource_id: ResourceId) -> PyResult<()> {
        self.send_command(Command::document_clear(resource_id))?;
        Ok(())
    }

    // Audio Routing and Audio Stream Monitoring Commands
    fn audio_send(
        &mut self,
        source_resource_id: ResourceId,
        sink_resource_id: ResourceId,
        source_channel: Option<u8>,
        sink_channel: Option<u8>,
        volume: Option<i16>,
        auto_gain: Option<bool>,
        auto_gain_resolution: Option<u16>,
        auto_gain_rise_time: Option<u16>,
        auto_gain_fall_time: Option<u16>,
        auto_gain_kill_time: Option<u16>,
    ) -> PyResult<()> {
        self.send_command(Command::audio_send(
            source_resource_id,
            sink_resource_id,
            source_channel,
            sink_channel,
            volume,
            auto_gain,
            auto_gain_resolution,
            auto_gain_rise_time,
            auto_gain_fall_time,
            auto_gain_kill_time,
        ))?;
        Ok(())
    }

    fn audio_cancel(
        &mut self,
        source_resource_id: ResourceId,
        sink_resource_id: ResourceId,
    ) -> PyResult<()> {
        self.send_command(Command::audio_cancel(source_resource_id, sink_resource_id))?;
        Ok(())
    }

    fn audio_level_notification_send(
        &mut self,
        resource_id: ResourceId,
        resolution: Option<u16>,
        voice_dead_band: Option<u16>,
        silence_dead_band: Option<u16>,
        adaptive_period: Option<u16>,
        voice_timer: Option<u16>,
        silence_timer: Option<u16>,
    ) -> PyResult<()> {
        self.send_command(Command::audio_level_notification_send(
            resource_id,
            resolution,
            voice_dead_band,
            silence_dead_band,
            adaptive_period,
            voice_timer,
            silence_timer,
        ))?;
        Ok(())
    }

    fn audio_level_notification_cancel(&mut self, resource_id: ResourceId) -> PyResult<()> {
        self.send_command(Command::audio_level_notification_cancel(resource_id))?;
        Ok(())
    }

    fn in_band_signaling_detection_enable(&mut self, resource_id: ResourceId) -> PyResult<()> {
        self.send_command(Command::in_band_signaling_detection_enable(resource_id))?;
        Ok(())
    }

    fn in_band_signaling_detection_disable(&mut self, resource_id: ResourceId) -> PyResult<()> {
        self.send_command(Command::in_band_signaling_detection_disable(resource_id))?;
        Ok(())
    }

    // Miscellaneous Commands
    fn get_rtp_statistics(&mut self, resource_id: ResourceId) -> PyResult<()> {
        self.send_command(Command::get_rtp_statistics(resource_id))?;
        Ok(())
    }
}
