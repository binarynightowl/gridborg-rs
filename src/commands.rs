use pyo3::prelude::{PyModule, PyModuleMethods};
use pyo3::{pyclass, pymethods, Bound, PyResult};
use std::fmt;
use crate::constants::AudioFormatType;
use crate::primitives::{ResourceId, SampleRate};

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
pub struct Login {
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
pub struct ResourceCreateFrontEnd {
    reg_incoming_ani: Option<String>,
    reg_incoming_dnis: Option<String>,
    reg_incoming_rdn: Option<String>,
    accepting: Option<bool>,
}
#[pyclass]
#[derive(Clone)]
pub struct ResourceCreatePlayer;
#[pyclass]
#[derive(Clone)]
pub struct ResourceCreateRecorder;
#[pyclass]
#[derive(Clone)]
pub struct ResourceCreateTransportChannel {
    transport_type: String,
}
#[pyclass]
#[derive(Clone)]
pub struct ResourceCreateRtpChannel {
    in_band_dtmf_enabled: Option<bool>,
}
#[pyclass]
#[derive(Clone)]
pub struct ResourceCreateSoundDevice {
    direction: String,
    device: Option<String>,
    buffers: Option<u8>,
}
#[pyclass]
#[derive(Clone)]
pub struct ResourceCreateFax;
#[pyclass]
#[derive(Clone)]
pub struct ResourceCreateDocument;
#[pyclass]
#[derive(Clone)]
pub struct ResourceDelete {
    resource_id: ResourceId,
}
#[pyclass]
#[derive(Clone)]
pub struct ResourceGetStatus {
    resource_id: ResourceId,
}

// Front-end Resource Commands
#[pyclass]
#[derive(Clone)]
pub struct CallMake {
    resource_id: ResourceId,
    address: String,
    timeout: Option<u32>, // Default: 30000 ms
    caller_number: Option<String>,
    caller_name: Option<String>,
    privacy: Option<u8>, // Default: 0
    screen: Option<u8>,  // Default: 1
}
#[pyclass]
#[derive(Clone)]
pub struct CallAnswer {
    resource_id: ResourceId,
}
#[pyclass]
#[derive(Clone)]
pub struct CallClear {
    resource_id: ResourceId,
    reason: Option<String>, // Optional reason string
}
#[pyclass]
#[derive(Clone)]
pub struct CallTransferConsultation {
    resource_id1: ResourceId,
    resource_id2: ResourceId,
}

#[pyclass]
#[derive(Clone)]
pub struct CallTransferBlind {
    resource_id: ResourceId,
    address: String,
    use_h450: Option<u8>, // Default: 1
}
#[pyclass]
#[derive(Clone)]
pub struct CallHold {
    resource_id: ResourceId,
}
#[pyclass]
#[derive(Clone)]
pub struct CallRetrieve {
    resource_id: ResourceId,
}
#[pyclass]
#[derive(Clone)]
pub struct CallSendDTMF {
    resource_id: ResourceId,
    dtmf_string: String,
    duration: Option<u32>,       // Default: 300 ms
    delay: Option<u32>,          // Default: 200 ms
    pause_duration: Option<u32>, // Default: 2000 ms
}
#[pyclass]
#[derive(Clone)]
pub struct CallStopActivity {
    resource_id: ResourceId,
}
#[pyclass]
#[derive(Clone)]
pub struct CallT38Relay {
    resource_id1: ResourceId,
    resource_id2: ResourceId,
}
#[pyclass]
#[derive(Clone)]
pub struct CallsSetAlertingType {
    resource_id: ResourceId,
    alerting_type: String,
}
#[pyclass]
#[derive(Clone)]
pub struct CallsSetAccepting {
    resource_id: ResourceId,
    accepting: bool,
}

// Player Resource Commands
#[pyclass]
#[derive(Clone)]
pub struct PlayFile {
    resource_id: ResourceId,
    file_name: String,
    audio_type: Option<AudioFormatType>,
    sample_rate: Option<SampleRate>,
}
#[pyclass]
#[derive(Clone)]
pub struct PlayStream {}
#[pyclass]
#[derive(Clone)]
pub struct PlayTone {}
#[pyclass]
#[derive(Clone)]
pub struct PlayStop {}

// Recorder Resource Commands
#[pyclass]
#[derive(Clone)]
pub struct RecorderStartToFile {}
#[pyclass]
#[derive(Clone)]
pub struct RecorderStartToStream {}
#[pyclass]
#[derive(Clone)]
pub struct RecorderStop {}

// RTP Channel Resource Commands
#[pyclass]
#[derive(Clone)]
pub struct RtpChannelStartReceiving {}
#[pyclass]
#[derive(Clone)]
pub struct RtpChannelStartSending {}
#[pyclass]
#[derive(Clone)]
pub struct RtpChannelStop {}
#[pyclass]
#[derive(Clone)]
pub struct RtpChannelSendDTMF {}

// Sound device Resource Commands
#[pyclass]
#[derive(Clone)]
pub struct SoundDeviceStart {}
#[pyclass]
#[derive(Clone)]
pub struct SoundDeviceStop {}

// Fax Resource Commands
#[pyclass]
#[derive(Clone)]
pub struct FaxReceive {}
#[pyclass]
#[derive(Clone)]
pub struct DocumentResourceId {}
#[pyclass]
#[derive(Clone)]
pub struct FaxSend {}
#[pyclass]
#[derive(Clone)]
pub struct FaxAbort {}

// Document Resource Commands
#[pyclass]
#[derive(Clone)]
pub struct DocumentAddFile {}
#[pyclass]
#[derive(Clone)]
pub struct DocumentPrepare {}
#[pyclass]
#[derive(Clone)]
pub struct DocumentSave {}
#[pyclass]
#[derive(Clone)]
pub struct DocumentClear {}

// Audio Routing and Audio Stream Monitoring Commands
#[pyclass]
#[derive(Clone)]
pub struct AudioSend {}
#[pyclass]
#[derive(Clone)]
pub struct AudioCancel {}
#[pyclass]
#[derive(Clone)]
pub struct AudioLevelNotificationSend {}
#[pyclass]
#[derive(Clone)]
pub struct AudioLevelNotificationCancel {}
#[pyclass]
#[derive(Clone)]
pub struct InBandSignalingDetectionEnable {}
#[pyclass]
#[derive(Clone)]
pub struct InBandSignalingDetectionDisable {}

// Miscellaneous Commands
#[pyclass]
#[derive(Clone)]
pub struct GetRtpStatistics {}

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
    pub fn resource_delete(resource_id: ResourceId) -> Self {
        Command::ResourceDelete(ResourceDelete { resource_id })
    }

    #[staticmethod]
    pub fn resource_get_status(resource_id: ResourceId) -> Self {
        Command::ResourceGetStatus(ResourceGetStatus { resource_id })
    }

    // Front-end Resource Commands
    #[staticmethod]
    pub fn call_make(
        resource_id: ResourceId,
        address: String,
        timeout: Option<u32>,
        caller_number: Option<String>,
        caller_name: Option<String>,
        privacy: Option<u8>,
        screen: Option<u8>,
    ) -> Self {
        Command::CallMake(CallMake {
            resource_id,
            address,
            timeout: timeout.or(Some(30000)),
            caller_number,
            caller_name,
            privacy: privacy.or(Some(0)),
            screen: screen.or(Some(1)),
        })
    }

    #[staticmethod]
    pub fn call_answer(resource_id: ResourceId) -> Self {
        Command::CallAnswer(CallAnswer { resource_id })
    }

    #[staticmethod]
    pub fn call_clear(resource_id: ResourceId, reason: Option<String>) -> Self {
        Command::CallClear(CallClear {
            resource_id,
            reason,
        })
    }

    #[staticmethod]
    pub fn call_transfer_consultation(resource_id1: u32, resource_id2: u32) -> Self {
        Command::CallTransferConsultation(CallTransferConsultation {
            resource_id1,
            resource_id2,
        })
    }

    #[staticmethod]
    pub fn call_transfer_blind(resource_id: ResourceId, address: String, use_h450: Option<u8>) -> Self {
        Command::CallTransferBlind(CallTransferBlind {
            resource_id,
            address,
            use_h450: use_h450.or(Some(1)),
        })
    }

    #[staticmethod]
    pub fn call_hold(resource_id: ResourceId) -> Self {
        Command::CallHold(CallHold { resource_id })
    }

    #[staticmethod]
    pub fn call_retrieve(resource_id: ResourceId) -> Self {
        Command::CallRetrieve(CallRetrieve { resource_id })
    }

    #[staticmethod]
    pub fn call_send_dtmf(
        resource_id: ResourceId,
        dtmf_string: String,
        duration: Option<u32>,
        delay: Option<u32>,
        pause_duration: Option<u32>,
    ) -> Self {
        Command::CallSendDTMF(CallSendDTMF {
            resource_id,
            dtmf_string,
            duration: duration.or(Some(300)),
            delay: delay.or(Some(200)),
            pause_duration: pause_duration.or(Some(2000)),
        })
    }

    #[staticmethod]
    pub fn call_stop_activity(resource_id: ResourceId) -> Self {
        Command::CallStopActivity(CallStopActivity { resource_id })
    }

    #[staticmethod]
    pub fn call_t38_relay(resource_id1: u32, resource_id2: u32) -> Self {
        Command::CallT38Relay(CallT38Relay {
            resource_id1,
            resource_id2,
        })
    }

    #[staticmethod]
    pub fn calls_set_alerting_type(resource_id: ResourceId, alerting_type: String) -> Self {
        Command::CallsSetAlertingType(CallsSetAlertingType {
            resource_id,
            alerting_type,
        })
    }

    #[staticmethod]
    pub fn calls_set_accepting(resource_id: ResourceId, accepting: bool) -> Self {
        Command::CallsSetAccepting(CallsSetAccepting {
            resource_id,
            accepting,
        })
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

            // Front-end Resource Commands
            Command::CallMake(cmd) => {
                write!(f, "CallMake {} {}", cmd.resource_id, cmd.address)?;
                if let Some(timeout) = cmd.timeout {
                    write!(f, " TimeOut={}", timeout)?;
                }
                if let Some(ref num) = cmd.caller_number {
                    write!(f, " CallerNumber={}", num)?;
                }
                if let Some(ref name) = cmd.caller_name {
                    write!(f, " CallerName={}", name)?;
                }
                if let Some(privacy) = cmd.privacy {
                    write!(f, " Privacy={}", privacy)?;
                }
                if let Some(screen) = cmd.screen {
                    write!(f, " Screen={}", screen)?;
                }
                Ok(())
            }
            Command::CallAnswer(cmd) => write!(f, "CallAnswer {}", cmd.resource_id),
            Command::CallClear(cmd) => {
                write!(f, "CallClear {}", cmd.resource_id)?;
                if let Some(ref reason) = cmd.reason {
                    write!(f, " Reason={}", reason)?;
                }
                Ok(())
            }
            Command::CallTransferConsultation(cmd) => {
                write!(
                    f,
                    "CallTransferConsultation {} {}",
                    cmd.resource_id1, cmd.resource_id2
                )
            }
            Command::CallTransferBlind(cmd) => {
                write!(f, "CallTransferBlind {} {}", cmd.resource_id, cmd.address)?;
                if let Some(use_h450) = cmd.use_h450 {
                    write!(f, " UseH450={}", use_h450)?;
                }
                Ok(())
            }
            Command::CallHold(cmd) => write!(f, "CallHold {}", cmd.resource_id),
            Command::CallRetrieve(cmd) => write!(f, "CallRetrieve {}", cmd.resource_id),
            Command::CallSendDTMF(cmd) => {
                write!(f, "CallSendDTMF {} {}", cmd.resource_id, cmd.dtmf_string)?;
                if let Some(duration) = cmd.duration {
                    write!(f, " Duration={}", duration)?;
                }
                if let Some(delay) = cmd.delay {
                    write!(f, " Delay={}", delay)?;
                }
                if let Some(pause) = cmd.pause_duration {
                    write!(f, " PauseDuration={}", pause)?;
                }
                Ok(())
            }
            Command::CallStopActivity(cmd) => write!(f, "CallStopActivity {}", cmd.resource_id),
            Command::CallT38Relay(cmd) => {
                write!(f, "CallT38Relay {} {}", cmd.resource_id1, cmd.resource_id2)
            }
            Command::CallsSetAlertingType(cmd) => {
                write!(
                    f,
                    "CallsSetAlertingType {} {}",
                    cmd.resource_id, cmd.alerting_type
                )
            }
            Command::CallsSetAccepting(cmd) => {
                write!(
                    f,
                    "CallsSetAccepting {} {}",
                    cmd.resource_id,
                    if cmd.accepting { 1 } else { 0 }
                )
            }
            _ => write!(f, "Unimplemented Command"),
        }
    }
}

impl From<Command> for String {
    fn from(val: Command) -> Self {
        val.to_string()
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

    fn resource_delete(&mut self, resource_id: ResourceId) -> PyResult<()>;
    fn resource_get_status(&mut self, resource_id: ResourceId) -> PyResult<()>;

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
    ) -> PyResult<()>;
    fn call_answer(&mut self, resource_id: ResourceId) -> PyResult<()>;
    fn call_clear(&mut self, resource_id: ResourceId, reason: Option<String>) -> PyResult<()>;
    fn call_transfer_consultation(&mut self, resource_id1: u32, resource_id2: u32) -> PyResult<()>;
    fn call_transfer_blind(
        &mut self,
        resource_id: ResourceId,
        address: String,
        use_h450: Option<u8>,
    ) -> PyResult<()>;
    fn call_hold(&mut self, resource_id: ResourceId) -> PyResult<()>;
    fn call_retrieve(&mut self, resource_id: ResourceId) -> PyResult<()>;
    fn call_send_dtmf(
        &mut self,
        resource_id: ResourceId,
        dtmf_string: String,
        duration: Option<u32>,
        delay: Option<u32>,
        pause_duration: Option<u32>,
    ) -> PyResult<()>;
    fn call_stop_activity(&mut self, resource_id: ResourceId) -> PyResult<()>;
    fn call_t38_relay(&mut self, resource_id1: u32, resource_id2: u32) -> PyResult<()>;
    fn calls_set_alerting_type(&mut self, resource_id: ResourceId, alerting_type: String) -> PyResult<()>;
    fn calls_set_accepting(&mut self, resource_id: ResourceId, accepting: bool) -> PyResult<()>;
}
