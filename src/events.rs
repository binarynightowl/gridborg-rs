use crate::constants::{
    DocumentPreparePaperSize, DocumentPrepareResolution, EStreamBufferStateNotification,
    FaxSendSpeed, PayloadType, RecorderStopReason,
};
use crate::primitives::{ResourceId, SessionId, ECM};
use pyo3::pyclass;

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
