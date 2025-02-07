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
struct ResourceCreateFrontEnd {}
#[pyclass]
#[derive(Clone)]
struct ResourceCreatePlayer {}
#[pyclass]
#[derive(Clone)]
struct ResourceCreateRecorder {}
#[pyclass]
#[derive(Clone)]
struct ResourceCreateTransportChannel {}
#[pyclass]
#[derive(Clone)]
struct ResourceCreateRtpChannel {}
#[pyclass]
#[derive(Clone)]
struct ResourceCreateSoundDevice {}
#[pyclass]
#[derive(Clone)]
struct ResourceCreateFax {}
#[pyclass]
#[derive(Clone)]
struct ResourceCreateDocument {}
#[pyclass]
#[derive(Clone)]
struct ResourceDelete {}
#[pyclass]
#[derive(Clone)]
struct ResourceGetStatus {}

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
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::ProtocolVersion(_) => write!(f, "ProtocolVersion"),
            Command::GetVersion(_) => write!(f, "GetVersion"),
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
    fn get_version(&mut self) -> PyResult<()>;
    fn get_protocol_version(&mut self) -> PyResult<()>;
    fn login(&mut self) -> PyResult<()>;
    fn logout(&mut self) -> PyResult<()>;
    fn quit(&mut self) -> PyResult<()>;
}