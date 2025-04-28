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
    DocumentNotSaved(DocumentCleared),
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
        _ => Err(ParseEventError::UnknownEvent(name.into())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
