use crate::constants::{
    DocumentPreparePaperSize, DocumentPrepareResolution, EStreamBufferStateNotification,
    FaxSendSpeed, PayloadType, RecorderStopReason,
};
use crate::primitives::{ResourceId, SessionId, ECM};
use pyo3::pyclass;
use serde::de::Visitor;
use serde::{de, Deserialize, Deserializer};
use std::{collections::HashMap, fmt, str::FromStr};

#[derive(thiserror::Error, Debug)]
pub enum ParseEventError {
    #[error("unknown event type '{0}'")]
    UnknownEvent(String),
    #[error("unexpected token count for {0}")]
    WrongArity(String),
    #[error("bad integer value in '{0}'")]
    BadInt(String),
    #[error("other: {0}")]
    Other(&'static str),
}

/// Try to convert the *required* positional token at `idx` into T.
fn parse_pos<T: FromStr>(tokens: &[&str], idx: usize, ev: &str) -> Result<T, ParseEventError> {
    tokens
        .get(idx)
        .ok_or(ParseEventError::WrongArity(ev.to_string()))?
        .parse::<T>()
        .map_err(|_| ParseEventError::BadInt(tokens[idx].to_owned()))
}

/// Collect leftover tokens → HashMap<name, value>
fn parse_opts(tokens: &[&str], start: usize) -> HashMap<String, String> {
    tokens[start..]
        .iter()
        .filter_map(|t| {
            let (k, v) = t.split_once('=')?;
            Some((k.to_ascii_lowercase(), v.to_string()))
        })
        .collect()
}

// Session, Resource and Notification Events
#[pyclass]
#[derive(Clone)]
pub struct SessionCreated {
    session_id: SessionId,
}
#[pyclass]
#[derive(Clone)]
pub struct SessionDeleted {
    session_id: SessionId,
}
#[pyclass]
#[derive(Clone)]
pub struct ResourceCreated {
    session_id: SessionId,
    resource_id: ResourceId,
}
#[pyclass]
#[derive(Clone)]
pub struct ResourceDeleted {
    session_id: SessionId,
    resource_id: ResourceId,
}
#[pyclass]
#[derive(Clone)]
pub struct AudioLevelNotification {
    session_id: SessionId,
    resource_id: ResourceId,
    in_talk: bool,
    energy_level: u8,
}
#[pyclass]
#[derive(Clone)]
pub struct StreamBufferStateNotification {
    session_id: SessionId,
    resource_id: ResourceId,
    state: EStreamBufferStateNotification,
}

// Front-end Events
#[pyclass]
#[derive(Clone)]
pub struct CallIncoming {
    session_id: SessionId,
    resource_id: ResourceId,
    call_identifier: String,
    ani: Option<String>,
    dnis: Option<String>,
    rdn: Option<String>,
    remote_name: Option<String>,
    remote_address: Option<String>,
}
#[pyclass]
#[derive(Clone)]
pub struct CallOutgoing {
    session_id: SessionId,
    resource_id: ResourceId,
    address: String,
    call_identifier: String,
}
#[pyclass]
#[derive(Clone)]
pub struct CallRemoteAlerting {
    session_id: SessionId,
    resource_id: ResourceId,
    user: Option<String>,
}
#[pyclass]
#[derive(Clone)]
pub struct CallConnectionEstablished {
    session_id: SessionId,
    resource_id: ResourceId,
}
#[pyclass]
#[derive(Clone)]
pub struct CallConnectionFailed {
    session_id: SessionId,
    resource_id: ResourceId,
    reason: String,
    protocol_specific_reason: Option<String>,
}
#[pyclass]
#[derive(Clone)]
pub struct CallCleared {
    session_id: SessionId,
    resource_id: ResourceId,
    reason: String,
    protocol_specific_reason: Option<String>,
}
#[pyclass]
#[derive(Clone)]
pub struct CallSendDTMFFinished {
    session_id: SessionId,
    resource_id: ResourceId,
}
#[pyclass]
#[derive(Clone)]
pub struct CallKeyPress {
    session_id: SessionId,
    resource_id: ResourceId,
    key: String,
    duration: Option<u16>,
}

// Player Resource Events
#[pyclass]
#[derive(Clone)]
pub struct PlayerStarted {
    session_id: SessionId,
    resource_id: ResourceId,
}
#[pyclass]
#[derive(Clone)]
pub struct PlayerStopped {
    session_id: SessionId,
    resource_id: ResourceId,
}
#[pyclass]
#[derive(Clone)]
pub struct PlayerError {
    session_id: SessionId,
    resource_id: ResourceId,
    error_text: String,
}

// Recorder Resource Events
#[pyclass]
#[derive(Clone)]
pub struct RecorderStarted {
    session_id: SessionId,
    resource_id: ResourceId,
}
#[pyclass]
#[derive(Clone)]
pub struct RecorderStopped {
    session_id: SessionId,
    resource_id: ResourceId,
    reason: RecorderStopReason,
}
#[pyclass]
#[derive(Clone)]
pub struct RecorderError {
    session_id: SessionId,
    resource_id: ResourceId,
    error_text: String,
}
#[pyclass]
#[derive(Clone)]
pub struct RecorderVoiceTrigger {
    session_id: SessionId,
    resource_id: ResourceId,
}

// RTP Channel Resource Events
#[pyclass]
#[derive(Clone)]
pub struct RtpChannelStartedReceiving {
    session_id: SessionId,
    resource_id: ResourceId,
    receiver_data_address: String,
    receiver_control_address: Option<String>,
    rtp_payload_type: Option<PayloadType>,
}
#[pyclass]
#[derive(Clone)]
pub struct RtpChannelStartedSending {
    session_id: SessionId,
    resource_id: ResourceId,
    sender_control_address: Option<String>,
    rtp_payload_type: Option<PayloadType>,
}
#[pyclass]
#[derive(Clone)]
pub struct RtpChannelSendDTMFFinished {
    session_id: SessionId,
    resource_id: ResourceId,
}
#[pyclass]
#[derive(Clone)]
pub struct RtpChannelReceivedDTMF {
    session_id: SessionId,
    resource_id: ResourceId,
    key: String,
    duration: Option<u16>,
}
#[pyclass]
#[derive(Clone)]
pub struct RtpChannelStopped {
    session_id: SessionId,
    resource_id: ResourceId,
}

// Sound Device Resource Events
#[pyclass]
#[derive(Clone)]
pub struct SoundDeviceStarted {
    session_id: SessionId,
    resource_id: ResourceId,
}
#[pyclass]
#[derive(Clone)]
pub struct SoundDeviceStopped {
    session_id: SessionId,
    resource_id: ResourceId,
}
#[pyclass]
#[derive(Clone)]
pub struct SoundDeviceError {
    session_id: SessionId,
    resource_id: ResourceId,
}

// Fax Resource Events
#[pyclass]
#[derive(Clone)]
pub struct ModeChangeT38 {
    session_id: SessionId,
    resource_id: ResourceId,
}
#[pyclass]
#[derive(Clone)]
pub struct ModeChangeT38Refused {
    session_id: SessionId,
    resource_id: ResourceId,
}
#[pyclass]
#[derive(Clone)]
pub struct FaxIncoming {
    session_id: SessionId,
    resource_id: ResourceId,
}
#[pyclass]
#[derive(Clone)]
pub struct FacsimilePageStarted {
    session_id: SessionId,
    resource_id: ResourceId,
    speed: FaxSendSpeed,
    paper_size: DocumentPreparePaperSize,
    resolution: DocumentPrepareResolution,
    ecm: ECM,
}
#[pyclass]
#[derive(Clone)]
pub struct FacsimilePageReceived {
    session_id: SessionId,
    resource_id: ResourceId,
}
#[pyclass]
#[derive(Clone)]
pub struct FacsimilePageSent {
    session_id: SessionId,
    resource_id: ResourceId,
}
#[pyclass]
#[derive(Clone)]
pub struct FaxOperationsStarted {
    session_id: SessionId,
    resource_id: ResourceId,
}
#[pyclass]
#[derive(Clone)]
pub struct FaxOperationFailed {
    session_id: SessionId,
    resource_id: ResourceId,
}
#[pyclass]
#[derive(Clone)]
pub struct FaxOperationFinished {
    session_id: SessionId,
    resource_id: ResourceId,
}
#[pyclass]
#[derive(Clone)]
pub struct FaxOperationAborted {
    session_id: SessionId,
    resource_id: ResourceId,
}

// Document Resource Events
#[pyclass]
#[derive(Clone)]
pub struct DocumentPrepared {
    session_id: SessionId,
    resource_id: ResourceId,
}
#[pyclass]
#[derive(Clone)]
pub struct DocumentNotPrepared {
    session_id: SessionId,
    resource_id: ResourceId,
    reason: String,
}
#[pyclass]
#[derive(Clone)]
pub struct DocumentSaved {
    session_id: SessionId,
    resource_id: ResourceId,
}
#[pyclass]
#[derive(Clone)]
pub struct DocumentNotSaved {
    session_id: SessionId,
    resource_id: ResourceId,
    reason: String,
}
#[pyclass]
#[derive(Clone)]
pub struct DocumentCleared {
    session_id: SessionId,
    resource_id: ResourceId,
}

#[pyclass]
#[derive(Clone)]
enum Event {
    // Session, Resource and Notification Events
    SessionCreated(SessionCreated),
    SessionDeleted(SessionDeleted),
    ResourceCreated(ResourceCreated),
    ResourceDeleted(ResourceDeleted),
    AudioLevelNotification(AudioLevelNotification),
    StreamBufferStateNotification(StreamBufferStateNotification),
    // Front-end Events
    CallIncoming(CallIncoming),
    CallOutgoing(CallOutgoing),
    CallRemoteAlerting(CallRemoteAlerting),
    CallConnectionEstablished(CallConnectionEstablished),
    CallConnectionFailed(CallConnectionFailed),
    CallCleared(CallCleared),
    CallSendDTMFFinished(CallSendDTMFFinished),
    CallKeyPress(CallKeyPress),
    // Player Resource Events
    PlayerStarted(PlayerStarted),
    PlayerStopped(PlayerStopped),
    PlayerError(PlayerError),
    // Recorder Resource Events
    RecorderStarted(RecorderStarted),
    RecorderStopped(RecorderStopped),
    RecorderError(RecorderError),
    RecorderVoiceTrigger(RecorderVoiceTrigger),
    // RTP Channel Resource Events
    RtpChannelStartedReceiving(RtpChannelStartedReceiving),
    RtpChannelStartedSending(RtpChannelStartedSending),
    RtpChannelSendDTMFFinished(RtpChannelSendDTMFFinished),
    RtpChannelReceivedDTMF(RtpChannelReceivedDTMF),
    RtpChannelStopped(RtpChannelStopped),
    // Sound Device Resource Events
    SoundDeviceStarted(SoundDeviceStarted),
    SoundDeviceStopped(SoundDeviceStopped),
    SoundDeviceError(SoundDeviceError),
    // Fax Resource Events
    ModeChangeT38(ModeChangeT38),
    ModeChangeT38Refused(ModeChangeT38Refused),
    FaxIncoming(FaxIncoming),
    FacsimilePageStarted(FacsimilePageStarted),
    FacsimilePageReceived(FacsimilePageReceived),
    FacsimilePageSent(FacsimilePageSent),
    FaxOperationsStarted(FaxOperationsStarted),
    FaxOperationFailed(FaxOperationFailed),
    FaxOperationFinished(FaxOperationFinished),
    FaxOperationAborted(FaxOperationAborted),
    // Document Resource Events
    DocumentPrepared(DocumentPrepared),
    DocumentNotPrepared(DocumentNotPrepared),
    DocumentSaved(DocumentSaved),
    DocumentNotSaved(DocumentNotSaved),
    DocumentCleared(DocumentCleared),
}

impl<'de> Deserialize<'de> for Event {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EventVisitor;
        impl<'de> Visitor<'de> for EventVisitor {
            type Value = Event;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a single Gridborg event line")
            }

            fn visit_str<E>(self, line: &str) -> Result<Event, E>
            where
                E: de::Error,
            {
                parse_event(line).map_err(|e| E::custom(e.to_string()))
            }
        }

        deserializer.deserialize_str(EventVisitor)
    }
}

fn parse_event(line: &str) -> Result<Event, ParseEventError> {
    let mut line = line.split('#').next().unwrap_or("").trim();
    if line.is_empty() {
        return Err(ParseEventError::Other("empty line"));
    }

    let tokens: Vec<&str> = line.split_whitespace().collect();
    let name = tokens[0];

    match name {
        // Session, Resource and Notification Events
        "ESessionCreated" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            Ok(Event::SessionCreated(SessionCreated { session_id }))
        }
        "ESessionDeleted" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            Ok(Event::SessionDeleted(SessionDeleted { session_id }))
        }
        "EResourceCreated" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            Ok(Event::ResourceCreated(ResourceCreated {
                session_id,
                resource_id,
            }))
        }
        "EResourceDeleted" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            Ok(Event::ResourceDeleted(ResourceDeleted {
                session_id,
                resource_id,
            }))
        }
        "EAudioLevelNotification" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            let in_talk = parse_pos::<bool>(&tokens, 3, name)?;
            let energy_level = parse_pos::<u8>(&tokens, 4, name)?;
            Ok(Event::AudioLevelNotification(AudioLevelNotification {
                session_id,
                resource_id,
                in_talk,
                energy_level,
            }))
        }
        "EStreamBufferStateNotification" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            let state = parse_pos::<EStreamBufferStateNotification>(&tokens, 3, name)?;
            Ok(Event::StreamBufferStateNotification(StreamBufferStateNotification {
                session_id,
                resource_id,
                state,
            }))
        }
        // Front-end Events
        "ECallIncoming" => {
            let session_id     = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id    = parse_pos::<ResourceId>(&tokens, 2, name)?;
            let call_identifier = tokens
                .get(3)
                .ok_or(ParseEventError::WrongArity(name.into()))?
                .to_string();

            let opts = parse_opts(&tokens, 4);

            let ani            = opts.get("ani").cloned();
            let dnis           = opts.get("dnis").cloned();
            let rdn            = opts.get("rdn").cloned();
            let remote_name    = opts.get("remotename").cloned();
            let remote_address = opts.get("remoteaddress").cloned();

            Ok(Event::CallIncoming(CallIncoming {
                session_id,
                resource_id,
                call_identifier,
                ani,
                dnis,
                rdn,
                remote_name,
                remote_address,
            }))
        }
        "ECallOutgoing" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            let address = tokens.get(3).ok_or(ParseEventError::WrongArity(name.into()))?.to_string();
            let call_identifier = tokens.get(4).ok_or(ParseEventError::WrongArity(name.into()))?.to_string();
            Ok(Event::CallOutgoing(CallOutgoing {
                session_id,
                resource_id,
                address,
                call_identifier,
            }))
        }
        "ECallRemoteAlerting" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            let opts = parse_opts(&tokens, 3);
            let user = opts.get("user").cloned();
            Ok(Event::CallRemoteAlerting(CallRemoteAlerting {
                session_id,
                resource_id,
                user,
            }))
        }
        "ECallConnectionEstablished" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            Ok(Event::CallConnectionEstablished(CallConnectionEstablished {
                session_id,
                resource_id,
            }))
        }
        "ECallConnectionFailed" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            let reason = tokens.get(3).ok_or(ParseEventError::WrongArity(name.into()))?.to_string();
            let opts = parse_opts(&tokens, 4);
            let protocol_specific_reason = opts.get("protocolspecificreason").cloned();
            Ok(Event::CallConnectionFailed(CallConnectionFailed {
                session_id,
                resource_id,
                reason,
                protocol_specific_reason,
            }))
        }
        "ECallCleared" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            let reason = tokens.get(3).ok_or(ParseEventError::WrongArity(name.into()))?.to_string();
            let opts = parse_opts(&tokens, 4);
            let protocol_specific_reason = opts.get("protocolspecificreason").cloned();
            Ok(Event::CallCleared(CallCleared {
                session_id,
                resource_id,
                reason,
                protocol_specific_reason,
            }))
        }
        "ECallSendDTMFFinished" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            Ok(Event::CallSendDTMFFinished(CallSendDTMFFinished {
                session_id,
                resource_id,
            }))
        }
        "ECallKeyPress" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            let key = tokens.get(3).ok_or(ParseEventError::WrongArity(name.into()))?.to_string();
            let opts = parse_opts(&tokens, 4);
            let duration = opts.get("duration").map(|v| v.parse().unwrap_or(0));
            Ok(Event::CallKeyPress(CallKeyPress {
                session_id,
                resource_id,
                key,
                duration,
            }))
        }
        // Player Resource Events
        "EPlayerStarted" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            Ok(Event::PlayerStarted(PlayerStarted {
                session_id,
                resource_id,
            }))
        }
        "EPlayerStopped" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            Ok(Event::PlayerStopped(PlayerStopped {
                session_id,
                resource_id,
            }))
        }
        "EPlayerError" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            let error_text = tokens.get(3).ok_or(ParseEventError::WrongArity(name.into()))?.to_string();
            Ok(Event::PlayerError(PlayerError {
                session_id,
                resource_id,
                error_text,
            }))
        }
        // Recorder Resource Events
        "ERecorderStarted" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            Ok(Event::RecorderStarted(RecorderStarted {
                session_id,
                resource_id,
            }))
        }
        "ERecorderStopped" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            let reason = parse_pos::<RecorderStopReason>(&tokens, 3, name)?;
            Ok(Event::RecorderStopped(RecorderStopped {
                session_id,
                resource_id,
                reason,
            }))
        }
        "ERecorderError" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            let error_text = tokens.get(3).ok_or(ParseEventError::WrongArity(name.into()))?.to_string();
            Ok(Event::RecorderError(RecorderError {
                session_id,
                resource_id,
                error_text,
            }))
        }
        "ERecorderVoiceTrigger" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            Ok(Event::RecorderVoiceTrigger(RecorderVoiceTrigger {
                session_id,
                resource_id,
            }))
        }
        // RTP Channel Resource Events
        "ERtpChannelStartedReceiving" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            let receiver_data_address = tokens.get(3).ok_or(ParseEventError::WrongArity(name.into()))?.to_string();
            let opts = parse_opts(&tokens, 4);
            let receiver_control_address = opts.get("receivercontroladdress").cloned();
            let rtp_payload_type = opts
                .get("rtppayloadtype")
                .and_then(|v| v.parse::<u8>().ok())
                .and_then(PayloadType::from_code);

            Ok(Event::RtpChannelStartedReceiving(RtpChannelStartedReceiving {
                session_id,
                resource_id,
                receiver_data_address,
                receiver_control_address,
                rtp_payload_type,
            }))
        }
        "ERtpChannelStartedSending" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            let opts = parse_opts(&tokens, 3);
            let sender_control_address = opts.get("sendercontroladdress").cloned();
            let rtp_payload_type = opts
                .get("rtppayloadtype")
                .and_then(|v| v.parse::<u8>().ok())
                .and_then(PayloadType::from_code);
            Ok(Event::RtpChannelStartedSending(RtpChannelStartedSending {
                session_id,
                resource_id,
                sender_control_address,
                rtp_payload_type,
            }))
        }
        "ERtpChannelSendDTMFFinished" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            Ok(Event::RtpChannelSendDTMFFinished(RtpChannelSendDTMFFinished {
                session_id,
                resource_id,
            }))
        }
        "ERtpChannelReceivedDTMF" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            let key = tokens.get(3).ok_or(ParseEventError::WrongArity(name.into()))?.to_string();
            let opts = parse_opts(&tokens, 4);
            let duration = opts.get("duration").map(|v| v.parse().unwrap());
            Ok(Event::RtpChannelReceivedDTMF(RtpChannelReceivedDTMF {
                session_id,
                resource_id,
                key,
                duration,
            }))
        }
        "ERtpChannelStopped" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            Ok(Event::RtpChannelStopped(RtpChannelStopped {
                session_id,
                resource_id,
            }))
        },
        // Sound Device Resource Events
        "ESoundDeviceStarted" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            Ok(Event::SoundDeviceStarted(SoundDeviceStarted {
                session_id,
                resource_id,
            }))
        }
        "ESoundDeviceStopped" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            Ok(Event::SoundDeviceStopped(SoundDeviceStopped {
                session_id,
                resource_id,
            }))
        }
        "ESoundDeviceError" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            Ok(Event::SoundDeviceError(SoundDeviceError {
                session_id,
                resource_id,
            }))
        }
        // Fax Resource Events
        "EModeChangeT38" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            Ok(Event::ModeChangeT38(ModeChangeT38 {
                session_id,
                resource_id,
            }))
        }
        "EModeChangeT38Refused" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            Ok(Event::ModeChangeT38Refused(ModeChangeT38Refused {
                session_id,
                resource_id,
            }))
        }
        "EFaxIncoming" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            Ok(Event::FaxIncoming(FaxIncoming {
                session_id,
                resource_id,
            }))
        }
        "EFacsimilePageStarted" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            let speed = parse_pos::<FaxSendSpeed>(&tokens, 3, name)?;
            let paper_size = parse_pos::<DocumentPreparePaperSize>(&tokens, 4, name)?;
            let resolution = parse_pos::<DocumentPrepareResolution>(&tokens, 5, name)?;
            let ecm = parse_pos::<ECM>(&tokens, 6, name)?;
            Ok(Event::FacsimilePageStarted(FacsimilePageStarted {
                session_id,
                resource_id,
                speed,
                paper_size,
                resolution,
                ecm,
            }))
        }
        "EFacsimilePageReceived" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            Ok(Event::FacsimilePageReceived(FacsimilePageReceived {
                session_id,
                resource_id,
            }))
        }
        "EFacsimilePageSent" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            Ok(Event::FacsimilePageSent(FacsimilePageSent {
                session_id,
                resource_id,
            }))
        }
        "EFaxOperationsStarted" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            Ok(Event::FaxOperationsStarted(FaxOperationsStarted {
                session_id,
                resource_id,
            }))
        }
        "EFaxOperationFailed" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            Ok(Event::FaxOperationFailed(FaxOperationFailed {
                session_id,
                resource_id,
            }))
        }
        "EFaxOperationFinished" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            Ok(Event::FaxOperationFinished(FaxOperationFinished {
                session_id,
                resource_id,
            }))
        }
        "EFaxOperationAborted" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            Ok(Event::FaxOperationAborted(FaxOperationAborted {
                session_id,
                resource_id,
            }))
        }
        // Document Resource Events
        "EDocumentPrepared" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            Ok(Event::DocumentPrepared(DocumentPrepared {
                session_id,
                resource_id,
            }))
        }
        "EDocumentNotPrepared" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            let reason = tokens.get(3).ok_or(ParseEventError::WrongArity(name.into()))?.to_string();
            Ok(Event::DocumentNotPrepared(DocumentNotPrepared {
                session_id,
                resource_id,
                reason,
            }))
        }
        "EDocumentSaved" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            Ok(Event::DocumentSaved(DocumentSaved {
                session_id,
                resource_id,
            }))
        }
        "EDocumentNotSaved" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            let reason = tokens.get(3).ok_or(ParseEventError::WrongArity(name.into()))?.to_string();
            Ok(Event::DocumentNotSaved(DocumentNotSaved {
                session_id,
                resource_id,
                reason,
            }))
        }
        "EDocumentCleared" => {
            let session_id = parse_pos::<SessionId>(&tokens, 1, name)?;
            let resource_id = parse_pos::<ResourceId>(&tokens, 2, name)?;
            Ok(Event::DocumentCleared(DocumentCleared {
                session_id,
                resource_id,
            }))
        }
        _ => Err(ParseEventError::UnknownEvent(name.into())),
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::FaxSendSpeed_V27At2400;
    use super::*;

    // Session, Resource and Notification Events
    #[test]
    fn parse_session_created() {
        let line = "ESessionCreated 1";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::SessionCreated(sc) => assert_eq!(sc.session_id, 1),
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_resource_created() {
        let line = "EResourceCreated 1 1";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::ResourceCreated(rc) => {
                assert_eq!(rc.session_id, 1);
                assert_eq!(rc.resource_id, 1);
            }
            _ => panic!("wrong variant"),
        }
    }

    // Front-end Events
    #[test]
    fn parse_call_incoming() {
        let line = "ECallIncoming 1 1 CALL123";

        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::CallIncoming(ci) => {
                assert_eq!(ci.session_id, 1);
                assert_eq!(ci.resource_id, 1);
                assert_eq!(ci.call_identifier, "CALL123");
                assert_eq!(ci.ani.as_deref(), None);
                assert_eq!(ci.dnis.as_deref(), None);
                assert_eq!(ci.remote_name.as_deref(), None);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_call_incoming_with_options() {
        let line = "\
        ECallIncoming 1 1 CALL123 \
        ANI=5551212 DNIS=1800 RDN=9988 \
        RemoteName=Bob RemoteAddress=bob@1.2.3.4:1720";

        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::CallIncoming(ci) => {
                assert_eq!(ci.session_id, 1);
                assert_eq!(ci.resource_id, 1);
                assert_eq!(ci.call_identifier, "CALL123");
                assert_eq!(ci.ani.as_deref(), Some("5551212"));
                assert_eq!(ci.dnis.as_deref(), Some("1800"));
                assert_eq!(ci.remote_name.as_deref(), Some("Bob"));
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_call_outgoing() {
        let line = "ECallOutgoing 1 2 address123 callid123";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::CallOutgoing(co) => {
                assert_eq!(co.session_id, 1);
                assert_eq!(co.resource_id, 2);
                assert_eq!(co.address, "address123");
                assert_eq!(co.call_identifier, "callid123");
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_call_remote_alerting() {
        let line = "ECallRemoteAlerting 1 2";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::CallRemoteAlerting(cra) => {
                assert_eq!(cra.session_id, 1);
                assert_eq!(cra.resource_id, 2);
                assert_eq!(cra.user, None);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_call_remote_alerting_with_options() {
        let line = "ECallRemoteAlerting 1 2 User=John";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::CallRemoteAlerting(cra) => {
                assert_eq!(cra.session_id, 1);
                assert_eq!(cra.resource_id, 2);
                assert_eq!(cra.user.as_deref(), Some("John"));
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_call_connection_established() {
        let line = "ECallConnectionEstablished 1 2";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::CallConnectionEstablished(cce) => {
                assert_eq!(cce.session_id, 1);
                assert_eq!(cce.resource_id, 2);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_call_connection_failed() {
        let line = "ECallConnectionFailed 1 2 ReasonText";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::CallConnectionFailed(ccf) => {
                assert_eq!(ccf.session_id, 1);
                assert_eq!(ccf.resource_id, 2);
                assert_eq!(ccf.reason, "ReasonText");
                assert_eq!(ccf.protocol_specific_reason, None);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_call_connection_failed_with_options() {
        let line = "ECallConnectionFailed 1 2 ReasonText ProtocolSpecificReason=SomeSpecific";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::CallConnectionFailed(ccf) => {
                assert_eq!(ccf.session_id, 1);
                assert_eq!(ccf.resource_id, 2);
                assert_eq!(ccf.reason, "ReasonText");
                assert_eq!(ccf.protocol_specific_reason.as_deref(), Some("SomeSpecific"));
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_call_cleared() {
        let line = "ECallCleared 1 2 ReasonText";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::CallCleared(cc) => {
                assert_eq!(cc.session_id, 1);
                assert_eq!(cc.resource_id, 2);
                assert_eq!(cc.reason, "ReasonText");
                assert_eq!(cc.protocol_specific_reason, None);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_call_cleared_with_options() {
        let line = "ECallCleared 1 2 ReasonText ProtocolSpecificReason=SomeSpecific";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::CallCleared(cc) => {
                assert_eq!(cc.session_id, 1);
                assert_eq!(cc.resource_id, 2);
                assert_eq!(cc.reason, "ReasonText");
                assert_eq!(cc.protocol_specific_reason.as_deref(), Some("SomeSpecific"));
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_call_send_dtmf_finished() {
        let line = "ECallSendDTMFFinished 1 2";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::CallSendDTMFFinished(csdf) => {
                assert_eq!(csdf.session_id, 1);
                assert_eq!(csdf.resource_id, 2);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_call_key_press() {
        let line = "ECallKeyPress 1 2 5";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::CallKeyPress(ckp) => {
                assert_eq!(ckp.session_id, 1);
                assert_eq!(ckp.resource_id, 2);
                assert_eq!(ckp.key, "5");
                assert_eq!(ckp.duration, None);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_call_key_press_with_options() {
        let line = "ECallKeyPress 1 2 5 Duration=150";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::CallKeyPress(ckp) => {
                assert_eq!(ckp.session_id, 1);
                assert_eq!(ckp.resource_id, 2);
                assert_eq!(ckp.key, "5");
                assert_eq!(ckp.duration, Some(150));
            }
            _ => panic!("wrong variant"),
        }
    }

    // --- Player Resource Events ---

    #[test]
    fn parse_player_started() {
        let line = "EPlayerStarted 1 2";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::PlayerStarted(ps) => {
                assert_eq!(ps.session_id, 1);
                assert_eq!(ps.resource_id, 2);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_player_stopped() {
        let line = "EPlayerStopped 1 2";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::PlayerStopped(ps) => {
                assert_eq!(ps.session_id, 1);
                assert_eq!(ps.resource_id, 2);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_player_error() {
        let line = "EPlayerError 1 2 ErrorText";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::PlayerError(pe) => {
                assert_eq!(pe.session_id, 1);
                assert_eq!(pe.resource_id, 2);
                assert_eq!(pe.error_text, "ErrorText");
            }
            _ => panic!("wrong variant"),
        }
    }

    // --- Recorder Resource Events ---

    #[test]
    fn parse_recorder_started() {
        let line = "ERecorderStarted 1 2";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::RecorderStarted(rs) => {
                assert_eq!(rs.session_id, 1);
                assert_eq!(rs.resource_id, 2);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_recorder_stopped() {
        let line = "ERecorderStopped 1 2 ExplicitRequest"; // Reason = ExplicitRequest (example value)
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::RecorderStopped(rs) => {
                assert_eq!(rs.session_id, 1);
                assert_eq!(rs.resource_id, 2);
                assert_eq!(rs.reason.name, "ExplicitRequest");
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_recorder_error() {
        let line = "ERecorderError 1 2 ErrorText";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::RecorderError(re) => {
                assert_eq!(re.session_id, 1);
                assert_eq!(re.resource_id, 2);
                assert_eq!(re.error_text, "ErrorText");
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_recorder_voice_trigger() {
        let line = "ERecorderVoiceTrigger 1 2";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::RecorderVoiceTrigger(rvt) => {
                assert_eq!(rvt.session_id, 1);
                assert_eq!(rvt.resource_id, 2);
            }
            _ => panic!("wrong variant"),
        }
    }

    // --- RTP Channel Resource Events ---

    #[test]
    fn parse_rtp_channel_started_receiving() {
        let line = "ERtpChannelStartedReceiving 1 2 receiver.data";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::RtpChannelStartedReceiving(rcsr) => {
                assert_eq!(rcsr.session_id, 1);
                assert_eq!(rcsr.resource_id, 2);
                assert_eq!(rcsr.receiver_data_address, "receiver.data");
                assert_eq!(rcsr.receiver_control_address, None);
                assert_eq!(rcsr.rtp_payload_type, None);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_rtp_channel_started_receiving_with_options() {
        let line = "ERtpChannelStartedReceiving 1 2 receiver.data ReceiverControlAddress=ctrl RtpPayloadType=8";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::RtpChannelStartedReceiving(rcsr) => {
                assert_eq!(rcsr.session_id, 1);
                assert_eq!(rcsr.resource_id, 2);
                assert_eq!(rcsr.receiver_data_address, "receiver.data");
                assert_eq!(rcsr.receiver_control_address.as_deref(), Some("ctrl"));
                assert_eq!(rcsr.rtp_payload_type.is_some(), true);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_rtp_channel_started_sending() {
        let line = "ERtpChannelStartedSending 1 2";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::RtpChannelStartedSending(rcss) => {
                assert_eq!(rcss.session_id, 1);
                assert_eq!(rcss.resource_id, 2);
                assert_eq!(rcss.sender_control_address, None);
                assert_eq!(rcss.rtp_payload_type, None);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_rtp_channel_started_sending_with_options() {
        let line = "ERtpChannelStartedSending 1 2 SenderControlAddress=ctrl RtpPayloadType=8";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::RtpChannelStartedSending(rcss) => {
                assert_eq!(rcss.session_id, 1);
                assert_eq!(rcss.resource_id, 2);
                assert_eq!(rcss.sender_control_address.as_deref(), Some("ctrl"));
                assert_eq!(rcss.rtp_payload_type.is_some(), true);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_rtp_channel_send_dtmf_finished() {
        let line = "ERtpChannelSendDTMFFinished 1 2";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::RtpChannelSendDTMFFinished(rcsdf) => {
                assert_eq!(rcsdf.session_id, 1);
                assert_eq!(rcsdf.resource_id, 2);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_rtp_channel_received_dtmf() {
        let line = "ERtpChannelReceivedDTMF 1 2 5";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::RtpChannelReceivedDTMF(rcrd) => {
                assert_eq!(rcrd.session_id, 1);
                assert_eq!(rcrd.resource_id, 2);
                assert_eq!(rcrd.key, "5");
                assert_eq!(rcrd.duration, None);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_rtp_channel_received_dtmf_with_options() {
        let line = "ERtpChannelReceivedDTMF 1 2 5 Duration=100";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::RtpChannelReceivedDTMF(rcrd) => {
                assert_eq!(rcrd.session_id, 1);
                assert_eq!(rcrd.resource_id, 2);
                assert_eq!(rcrd.key, "5");
                assert_eq!(rcrd.duration, Some(100));
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_rtp_channel_stopped() {
        let line = "ERtpChannelStopped 1 2";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::RtpChannelStopped(rcs) => {
                assert_eq!(rcs.session_id, 1);
                assert_eq!(rcs.resource_id, 2);
            }
            _ => panic!("wrong variant"),
        }
    }

    // --- Sound Device Resource Events ---

    #[test]
    fn parse_sound_device_started() {
        let line = "ESoundDeviceStarted 1 2";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::SoundDeviceStarted(sds) => {
                assert_eq!(sds.session_id, 1);
                assert_eq!(sds.resource_id, 2);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_sound_device_stopped() {
        let line = "ESoundDeviceStopped 1 2";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::SoundDeviceStopped(sds) => {
                assert_eq!(sds.session_id, 1);
                assert_eq!(sds.resource_id, 2);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_sound_device_error() {
        let line = "ESoundDeviceError 1 2";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::SoundDeviceError(sde) => {
                assert_eq!(sde.session_id, 1);
                assert_eq!(sde.resource_id, 2);
            }
            _ => panic!("wrong variant"),
        }
    }

    // --- Fax Resource Events ---

    #[test]
    fn parse_mode_change_t38() {
        let line = "EModeChangeT38 1 2";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::ModeChangeT38(mct) => {
                assert_eq!(mct.session_id, 1);
                assert_eq!(mct.resource_id, 2);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_mode_change_t38_refused() {
        let line = "EModeChangeT38Refused 1 2";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::ModeChangeT38Refused(mctr) => {
                assert_eq!(mctr.session_id, 1);
                assert_eq!(mctr.resource_id, 2);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_fax_incoming() {
        let line = "EFaxIncoming 1 2";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::FaxIncoming(fi) => {
                assert_eq!(fi.session_id, 1);
                assert_eq!(fi.resource_id, 2);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_facsimile_page_started() {
        let line = format!("EFacsimilePageStarted 1 2 V27At2400 Legal Low 64");
        let ev: Event = serde_plain::from_str(&*line).unwrap();
        match ev {
            Event::FacsimilePageStarted(fps) => {
                assert_eq!(fps.session_id, 1);
                assert_eq!(fps.resource_id, 2);
                assert_eq!(fps.speed, FaxSendSpeed_V27At2400);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_facsimile_page_received() {
        let line = "EFacsimilePageReceived 1 2";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::FacsimilePageReceived(fpr) => {
                assert_eq!(fpr.session_id, 1);
                assert_eq!(fpr.resource_id, 2);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_facsimile_page_sent() {
        let line = "EFacsimilePageSent 1 2";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::FacsimilePageSent(fps) => {
                assert_eq!(fps.session_id, 1);
                assert_eq!(fps.resource_id, 2);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_fax_operations_started() {
        let line = "EFaxOperationsStarted 1 2";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::FaxOperationsStarted(fos) => {
                assert_eq!(fos.session_id, 1);
                assert_eq!(fos.resource_id, 2);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_fax_operation_failed() {
        let line = "EFaxOperationFailed 1 2";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::FaxOperationFailed(fof) => {
                assert_eq!(fof.session_id, 1);
                assert_eq!(fof.resource_id, 2);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_fax_operation_finished() {
        let line = "EFaxOperationFinished 1 2";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::FaxOperationFinished(fof) => {
                assert_eq!(fof.session_id, 1);
                assert_eq!(fof.resource_id, 2);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_fax_operation_aborted() {
        let line = "EFaxOperationAborted 1 2";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::FaxOperationAborted(foa) => {
                assert_eq!(foa.session_id, 1);
                assert_eq!(foa.resource_id, 2);
            }
            _ => panic!("wrong variant"),
        }
    }

    // --- Document Resource Events ---

    #[test]
    fn parse_document_prepared() {
        let line = "EDocumentPrepared 1 2";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::DocumentPrepared(dp) => {
                assert_eq!(dp.session_id, 1);
                assert_eq!(dp.resource_id, 2);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_document_not_prepared() {
        let line = "EDocumentNotPrepared 1 2 ReasonText";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::DocumentNotPrepared(dnp) => {
                assert_eq!(dnp.session_id, 1);
                assert_eq!(dnp.resource_id, 2);
                assert_eq!(dnp.reason, "ReasonText");
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_document_saved() {
        let line = "EDocumentSaved 1 2";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::DocumentSaved(ds) => {
                assert_eq!(ds.session_id, 1);
                assert_eq!(ds.resource_id, 2);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_document_not_saved() {
        let line = "EDocumentNotSaved 1 2 ReasonText";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::DocumentNotSaved(dns) => {
                assert_eq!(dns.session_id, 1);
                assert_eq!(dns.resource_id, 2);
                assert_eq!(dns.reason, "ReasonText");
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_document_cleared() {
        let line = "EDocumentCleared 1 2";
        let ev: Event = serde_plain::from_str(line).unwrap();
        match ev {
            Event::DocumentCleared(dc) => {
                assert_eq!(dc.session_id, 1);
                assert_eq!(dc.resource_id, 2);
            }
            _ => panic!("wrong variant"),
        }
    }
}
