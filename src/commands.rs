use pyo3::prelude::{PyModule, PyModuleMethods};
use pyo3::{pyclass, pymethods, wrap_pyfunction, Bound, PyResult};
use std::fmt;

pub fn init(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new_bound(parent_module.py(), "commands")?;

    child_module.add_class::<Command>()?;

    parent_module.add_submodule(&child_module)
}

// Product Information Commands
#[pyclass]
#[derive(Clone)]
pub struct ProtocolVersion;
#[pyclass]
#[derive(Clone)]
pub struct GetVersion;

// Session Commands
#[pyclass]
#[derive(Clone)]
struct Login {
    username: String,
    password: String,
    protocol_major_version: u8,
    protocol_minor_version: u8,
    protocol_revision: Option<u8>,
}

#[pyclass]
#[derive(Clone)]
pub struct Logout;
#[pyclass]
#[derive(Clone)]
pub struct Quit;

// General Resource Commands
#[pyclass]
#[derive(Clone)]
struct ResourceCreateFrontEnd {
    reg_incoming_ani: Option<String>,
    reg_incoming_dnis: Option<String>,
    reg_incoming_rdn: Option<String>,
    accepting: Option<bool>,
}
#[pyclass]
#[derive(Clone)]
struct ResourceCreatePlayer;
#[pyclass]
#[derive(Clone)]
struct ResourceCreateRecorder;
#[pyclass]
#[derive(Clone)]
struct ResourceCreateTransportChannel {
    transport_type: String,
}
#[pyclass]
#[derive(Clone)]
struct ResourceCreateRtpChannel {
    in_band_dtmf_enabled: Option<bool>,
}
#[pyclass]
#[derive(Clone)]
struct ResourceCreateSoundDevice {
    direction: String,
    device: Option<String>,
    buffers: Option<u8>,
}
#[pyclass]
#[derive(Clone)]
struct ResourceCreateFax;
#[pyclass]
#[derive(Clone)]
struct ResourceCreateDocument;
#[pyclass]
#[derive(Clone)]
struct ResourceDelete {
    resource_id: u32,
}
#[pyclass]
#[derive(Clone)]
struct ResourceGetStatus {
    resource_id: u32,
}

// Front-end Resource Commands
#[pyclass]
#[derive(Clone)]
struct CallMake {}
#[pyclass]
#[derive(Clone)]
struct CallAnswer {}
#[pyclass]
#[derive(Clone)]
struct CallClear {}
#[pyclass]
#[derive(Clone)]
struct CallTransferConsultation {}
#[pyclass]
#[derive(Clone)]
struct CallTransferBlind {}
#[pyclass]
#[derive(Clone)]
struct CallHold {}
#[pyclass]
#[derive(Clone)]
struct CallRetrieve {}
#[pyclass]
#[derive(Clone)]
struct CallSendDTMF {}
#[pyclass]
#[derive(Clone)]
struct CallStopActivity {}
#[pyclass]
#[derive(Clone)]
struct CallT38Relay {}
#[pyclass]
#[derive(Clone)]
struct CallsSetAlertingType {}
#[pyclass]
#[derive(Clone)]
struct CallsSetAccepting {}

// Player Resource Commands
#[pyclass]
#[derive(Clone)]
struct PlayFile {}
#[pyclass]
#[derive(Clone)]
struct PlayStream {}
#[pyclass]
#[derive(Clone)]
struct PlayTone {}
#[pyclass]
#[derive(Clone)]
struct PlayStop {}

// Recorder Resource Commands
#[pyclass]
#[derive(Clone)]
struct RecorderStartToFile {}
#[pyclass]
#[derive(Clone)]
struct RecorderStartToStream {}
#[pyclass]
#[derive(Clone)]
struct RecorderStop {}

// RTP Channel Resource Commands
#[pyclass]
#[derive(Clone)]
struct RtpChannelStartReceiving {}
#[pyclass]
#[derive(Clone)]
struct RtpChannelStartSending {}
#[pyclass]
#[derive(Clone)]
struct RtpChannelStop {}
#[pyclass]
#[derive(Clone)]
struct RtpChannelSendDTMF {}

// Sound device Resource Commands
#[pyclass]
#[derive(Clone)]
struct SoundDeviceStart {}
#[pyclass]
#[derive(Clone)]
struct SoundDeviceStop {}

// Fax Resource Commands
#[pyclass]
#[derive(Clone)]
struct FaxReceive {}
#[pyclass]
#[derive(Clone)]
struct DocumentResourceId {}
#[pyclass]
#[derive(Clone)]
struct FaxSend {}
#[pyclass]
#[derive(Clone)]
struct FaxAbort {}

// Document Resource Commands
#[pyclass]
#[derive(Clone)]
struct DocumentAddFile {}
#[pyclass]
#[derive(Clone)]
struct DocumentPrepare {}
#[pyclass]
#[derive(Clone)]
struct DocumentSave {}
#[pyclass]
#[derive(Clone)]
struct DocumentClear {}

// Audio Routing and Audio Stream Monitoring Commands
#[pyclass]
#[derive(Clone)]
struct AudioSend {}
#[pyclass]
#[derive(Clone)]
struct AudioCancel {}
#[pyclass]
#[derive(Clone)]
struct AudioLevelNotificationSend {}
#[pyclass]
#[derive(Clone)]
struct AudioLevelNotificationCancel {}
#[pyclass]
#[derive(Clone)]
struct InBandSignalingDetectionEnable {}
#[pyclass]
#[derive(Clone)]
struct InBandSignalingDetectionDisable {}

// Miscellaneous Commands
#[pyclass]
#[derive(Clone)]
struct GetRtpStatistics {}

#[pyclass(str)]
#[derive(Clone)]
pub enum Command {
    ProtocolVersion(ProtocolVersion),
    GetVersion(GetVersion),
    Login(Login),
    Logout(Logout),
    Quit(Quit),
    ResourceCreateFrontEnd(ResourceCreateFrontEnd),
    ResourceCreatePlayer(ResourceCreatePlayer),
    ResourceCreateRecorder(ResourceCreateRecorder),
    ResourceCreateTransportChannel(ResourceCreateTransportChannel),
    ResourceCreateRtpChannel(ResourceCreateRtpChannel),
    ResourceCreateSoundDevice(ResourceCreateSoundDevice),
    ResourceCreateFax(ResourceCreateFax),
    ResourceCreateDocument(ResourceCreateDocument),
    ResourceDelete(ResourceDelete),
    ResourceGetStatus(ResourceGetStatus),
    CallMake(CallMake),
    CallAnswer(CallAnswer),
    CallClear(CallClear),
    CallTransferConsultation(CallTransferConsultation),
    CallTransferBlind(CallTransferBlind),
    CallHold(CallHold),
    CallRetrieve(CallRetrieve),
    CallSendDTMF(CallSendDTMF),
    CallStopActivity(CallStopActivity),
    CallT38Relay(CallT38Relay),
    CallsSetAlertingType(CallsSetAlertingType),
    CallsSetAccepting(CallsSetAccepting),
    PlayFile(PlayFile),
    PlayStream(PlayStream),
    PlayTone(PlayTone),
    PlayStop(PlayStop),
    RecorderStartToFile(RecorderStartToFile),
    RecorderStartToStream(RecorderStartToStream),
    RecorderStop(RecorderStop),
    RtpChannelStartReceiving(RtpChannelStartReceiving),
    RtpChannelStartSending(RtpChannelStartSending),
    RtpChannelStop(RtpChannelStop),
    RtpChannelSendDTMF(RtpChannelSendDTMF),
    SoundDeviceStart(SoundDeviceStart),
    SoundDeviceStop(SoundDeviceStop),
    FaxReceive(FaxReceive),
    DocumentResourceId(DocumentResourceId),
    FaxSend(FaxSend),
    FaxAbort(FaxAbort),
    DocumentAddFile(DocumentAddFile),
    DocumentPrepare(DocumentPrepare),
    DocumentSave(DocumentSave),
    DocumentClear(DocumentClear),
    AudioSend(AudioSend),
    AudioCancel(AudioCancel),
    AudioLevelNotificationSend(AudioLevelNotificationSend),
    AudioLevelNotificationCancel(AudioLevelNotificationCancel),
    InBandSignalingDetectionEnable(InBandSignalingDetectionEnable),
    InBandSignalingDetectionDisable(InBandSignalingDetectionDisable),
    GetRtpStatistics(GetRtpStatistics),
}

#[pymethods]
impl Command {
    // Product Information Commands
    #[staticmethod]
    pub fn protocol_version() -> Self {
        Command::ProtocolVersion(ProtocolVersion {})
    }

    #[staticmethod]
    pub fn get_version() -> Self {
        Command::GetVersion(GetVersion {})
    }

    // Session Commands
    #[staticmethod]
    pub fn login(
        username: String,
        password: String,
        major: Option<u8>,
        minor: Option<u8>,
        revision: Option<u8>,
    ) -> Self {
        let major = major.unwrap_or(2);
        let minor = minor.unwrap_or(3);
        Command::Login(Login {
            username,
            password,
            protocol_major_version: major,
            protocol_minor_version: minor,
            protocol_revision: revision,
        })
    }

    #[staticmethod]
    pub fn logout() -> Self {
        Command::Logout(Logout {})
    }

    #[staticmethod]
    pub fn quit() -> Self {
        Command::Quit(Quit {})
    }

    // General Resource Commands
    #[staticmethod]
    pub fn resource_create_frontend(
        reg_incoming_ani: Option<String>,
        reg_incoming_dnis: Option<String>,
        reg_incoming_rdn: Option<String>,
        accepting: Option<bool>,
    ) -> Self {
        Command::ResourceCreateFrontEnd(ResourceCreateFrontEnd {
            reg_incoming_ani,
            reg_incoming_dnis,
            reg_incoming_rdn,
            accepting,
        })
    }

    #[staticmethod]
    pub fn resource_create_player() -> Self {
        Command::ResourceCreatePlayer(ResourceCreatePlayer {})
    }

    #[staticmethod]
    pub fn resource_create_recorder() -> Self {
        Command::ResourceCreateRecorder(ResourceCreateRecorder {})
    }

    #[staticmethod]
    pub fn resource_create_transport_channel(transport_type: String) -> Self {
        Command::ResourceCreateTransportChannel(ResourceCreateTransportChannel { transport_type })
    }

    #[staticmethod]
    pub fn resource_create_rtp_channel(in_band_dtmf_enabled: Option<bool>) -> Self {
        Command::ResourceCreateRtpChannel(ResourceCreateRtpChannel {
            in_band_dtmf_enabled,
        })
    }

    #[staticmethod]
    pub fn resource_create_sound_device(
        direction: String,
        device: Option<String>,
        buffers: Option<u8>,
    ) -> Self {
        Command::ResourceCreateSoundDevice(ResourceCreateSoundDevice {
            direction,
            device,
            buffers,
        })
    }

    #[staticmethod]
    pub fn resource_create_fax() -> Self {
        Command::ResourceCreateFax(ResourceCreateFax {})
    }

    #[staticmethod]
    pub fn resource_create_document() -> Self {
        Command::ResourceCreateDocument(ResourceCreateDocument {})
    }

    #[staticmethod]
    pub fn resource_delete(resource_id: u32) -> Self {
        Command::ResourceDelete(ResourceDelete { resource_id })
    }

    #[staticmethod]
    pub fn resource_get_status(resource_id: u32) -> Self {
        Command::ResourceGetStatus(ResourceGetStatus { resource_id })
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // Product Information Commands
            Command::ProtocolVersion(_) => write!(f, "ProtocolVersion"),
            Command::GetVersion(_) => write!(f, "GetVersion"),

            // Session Commands
            Command::Login(login) => {
                if let Some(revision) = login.protocol_revision {
                    write!(
                        f,
                        "Login {} {} {} {} {}",
                        login.username,
                        login.password,
                        login.protocol_major_version,
                        login.protocol_minor_version,
                        revision
                    )
                } else {
                    write!(
                        f,
                        "Login {} {} {} {}",
                        login.username,
                        login.password,
                        login.protocol_major_version,
                        login.protocol_minor_version
                    )
                }
            }
            Command::Logout(_) => write!(f, "Logout"),
            Command::Quit(_) => write!(f, "Quit"),

            // General Resource Commands
            Command::ResourceCreateFrontEnd(cmd) => {
                write!(f, "ResourceCreateFrontEnd")?;
                if let Some(ani) = &cmd.reg_incoming_ani {
                    write!(f, " RegIncomingANI={}", ani)?;
                }
                if let Some(dnis) = &cmd.reg_incoming_dnis {
                    write!(f, " RegIncomingDNIS={}", dnis)?;
                }
                if let Some(rdn) = &cmd.reg_incoming_rdn {
                    write!(f, " RegIncomingRDN={}", rdn)?;
                }
                if let Some(accepting) = cmd.accepting {
                    write!(f, " Accepting={}", if accepting { 1 } else { 0 })?;
                }
                Ok(())
            }
            Command::ResourceCreatePlayer(_) => write!(f, "ResourceCreatePlayer"),
            Command::ResourceCreateRecorder(_) => write!(f, "ResourceCreateRecorder"),
            Command::ResourceCreateTransportChannel(cmd) => {
                write!(f, "ResourceCreateTransportChannel {}", cmd.transport_type)
            }
            Command::ResourceCreateRtpChannel(cmd) => {
                write!(f, "ResourceCreateRtpChannel")?;
                if let Some(enabled) = cmd.in_band_dtmf_enabled {
                    write!(f, " InBandDTMFEnabled={}", if enabled { 1 } else { 0 })?;
                }
                Ok(())
            }
            Command::ResourceCreateSoundDevice(cmd) => {
                write!(f, "ResourceCreateSoundDevice Direction={}", cmd.direction)?;
                if let Some(device) = &cmd.device {
                    write!(f, " Device={}", device)?;
                }
                if let Some(buffers) = cmd.buffers {
                    write!(f, " Buffers={}", buffers)?;
                }
                Ok(())
            }
            Command::ResourceCreateFax(_) => write!(f, "ResourceCreateFax"),
            Command::ResourceCreateDocument(_) => write!(f, "ResourceCreateDocument"),
            Command::ResourceDelete(cmd) => write!(f, "ResourceDelete {}", cmd.resource_id),
            Command::ResourceGetStatus(cmd) => write!(f, "ResourceGetStatus {}", cmd.resource_id),

            _ => write!(f, "Unimplemented Command"),
        }
    }
}

impl Into<String> for Command {
    fn into(self) -> String {
        self.to_string()
    }
}

pub trait CommandHandler: Send + Sync {
    // Product Information Commands
    fn get_version(&mut self) -> PyResult<()>;
    fn get_protocol_version(&mut self) -> PyResult<()>;
    fn login(&mut self) -> PyResult<()>;
    // fn login(&mut self, username: String, password: String, major: Option<u8>, minor: Option<u8>, revision: Option<u8>) -> PyResult<()>;
    fn logout(&mut self) -> PyResult<()>;
    fn quit(&mut self) -> PyResult<()>;

    // General Resource Commands
    fn resource_create_frontend(
        &mut self,
        reg_incoming_ani: Option<String>,
        reg_incoming_dnis: Option<String>,
        reg_incoming_rdn: Option<String>,
        accepting: Option<bool>,
    ) -> PyResult<()>;

    fn resource_create_player(&mut self) -> PyResult<()>;
    fn resource_create_recorder(&mut self) -> PyResult<()>;
    fn resource_create_transport_channel(&mut self, transport_type: String) -> PyResult<()>;

    fn resource_create_rtp_channel(&mut self, in_band_dtmf_enabled: Option<bool>) -> PyResult<()>;

    fn resource_create_sound_device(
        &mut self,
        direction: String,
        device: Option<String>,
        buffers: Option<u8>,
    ) -> PyResult<()>;

    fn resource_create_fax(&mut self) -> PyResult<()>;
    fn resource_create_document(&mut self) -> PyResult<()>;

    fn resource_delete(&mut self, resource_id: u32) -> PyResult<()>;
    fn resource_get_status(&mut self, resource_id: u32) -> PyResult<()>;
}

