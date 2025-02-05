// Product Information Commands
struct ProtocolVersion {}
struct GetVersion {}

// Session Commands
struct Login {}
struct Logout {}
struct Quit {}

// General Resource Commands
struct ResourceCreateFrontEnd {}
struct ResourceCreatePlayer {}
struct ResourceCreateRecorder {}
struct ResourceCreateTransportChannel {}
struct ResourceCreateRtpChannel {}
struct ResourceCreateSoundDevice {}
struct ResourceCreateFax {}
struct ResourceCreateDocument {}
struct ResourceDelete {}
struct ResourceGetStatus {}

// Front-end Resource Commands
struct CallMake {}
struct CallAnswer {}
struct CallClear {}
struct CallTransferConsultation {}
struct CallTransferBlind {}
struct CallHold {}
struct CallRetrieve {}
struct CallSendDTMF {}
struct CallStopActivity {}
struct CallT38Relay {}
struct CallsSetAlertingType {}
struct CallsSetAccepting {}

// Player Resource Commands
struct PlayFile {}
struct PlayStream {}
struct PlayTone {}
struct PlayStop {}

// Recorder Resource Commands
struct RecorderStartToFile {}
struct RecorderStartToStream {}
struct RecorderStop {}

// RTP Channel Resource Commands
struct RtpChannelStartReceiving {}
struct RtpChannelStartSending {}
struct RtpChannelStop {}
struct RtpChannelSendDTMF {}

// Sound device Resource Commands
struct SoundDeviceStart {}
struct SoundDeviceStop {}

// Fax Resource Commands
struct FaxReceive {}
struct DocumentResourceId {}
struct FaxSend {}
struct FaxAbort {}

// Document Resource Commands
struct DocumentAddFile {}
struct DocumentPrepare {}
struct DocumentSave {}
struct DocumentClear {}

// Audio Routing and Audio Stream Monitoring Commands
struct AudioSend {}
struct AudioCancel {}
struct AudioLevelNotificationSend {}
struct AudioLevelNotificationCancel {}
struct InBandSignalingDetectionEnable {}
struct InBandSignalingDetectionDisable {}

// Miscellaneous Commands
struct GetRtpStatistics {}

enum Command {
    ProtocolVersion,
    GetVersion,
    Login,
    Logout,
    Quit,
    ResourceCreateFrontEnd,
    ResourceCreatePlayer,
    ResourceCreateRecorder,
    ResourceCreateTransportChannel,
    ResourceCreateRtpChannel,
    ResourceCreateSoundDevice,
    ResourceCreateFax,
    ResourceCreateDocument,
    ResourceDelete,
    ResourceGetStatus,
    CallMake,
    CallAnswer,
    CallClear,
    CallTransferConsultation,
    CallTransferBlind,
    CallHold,
    CallRetrieve,
    CallSendDTMF,
    CallStopActivity,
    CallT38Relay,
    CallsSetAlertingType,
    CallsSetAccepting,
    PlayFile,
    PlayStream,
    PlayTone,
    PlayStop,
    RecorderStartToFile,
    RecorderStartToStream,
    RecorderStop,
    RtpChannelStartReceiving,
    RtpChannelStartSending,
    RtpChannelStop,
    RtpChannelSendDTMF,
    SoundDeviceStart,
    SoundDeviceStop,
    FaxReceive,
    DocumentResourceId,
    FaxSend,
    FaxAbort,
    DocumentAddFile,
    DocumentPrepare,
    DocumentSave,
    DocumentClear,
    AudioSend,
    AudioCancel,
    AudioLevelNotificationSend,
    AudioLevelNotificationCancel,
    InBandSignalingDetectionEnable,
    InBandSignalingDetectionDisable,
    GetRtpStatistics,
}
