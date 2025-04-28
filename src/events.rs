use pyo3::pyclass;
use crate::constants::EStreamBufferStateNotification;
use crate::primitives::{ResourceId, SessionId};

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

enum Event {
    // Session, Resource and Notification Events
    SessionCreated(SessionCreated),
    SessionDeleted(SessionDeleted),
    ResourceCreated(ResourceCreated),
    ResourceDeleted(ResourceDeleted),
    AudioLevelNotification(AudioLevelNotification),
    StreamBufferStateNotification(StreamBufferStateNotification),
    // Front-end Events
}