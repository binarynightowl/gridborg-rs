use crate::{audio_formats, constant_set, payload_types};
use paste::paste;
use pyo3::pyclass;
use crate::primitives::{Channels, SampleRate};

#[pyclass]
#[derive(Clone)]
enum ResourceType {
    FrontEnd,
    Player,
    Recorder,
    TransportChannel,
    RtpChannel,
    SoundDevice,
    Fax,
    Document,
}

#[pyclass]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AudioFormatType {
    pub name: &'static str,
    pub channels: Channels,
}

#[pyclass]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PayloadType {
    pub name: &'static str,
    pub type_code: Option<u8>,
    pub sample_rate: SampleRate,
}

#[pyclass]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConstantWithDescription {
    pub name: &'static str,
    pub description: &'static str,
}

audio_formats! {
    //  codec          wav raw vap  channels
    (ALAW        ,   Y , Y , Y , Channels::from_u8(2)),
    (MULAW       ,   Y , Y , Y , Channels::from_u8(2)),
    (PCM16       ,   Y , Y , N , Channels::from_u8(2)),
    (PCM8        ,   Y , Y , N , Channels::from_u8(2)),
    (PCM_S8      ,   N , Y , N , Channels::from_u8(1)),
    (Linear_16_Mono_16kHz , N , Y , N , Channels::from_u8(1)),
    (G726_40K    ,   N , Y , N , Channels::from_u8(1)),
    (G726_32K    ,   N , Y , N , Channels::from_u8(1)),
    (G726_24K    ,   N , Y , N , Channels::from_u8(1)),
    (G726_16K    ,   N , Y , N , Channels::from_u8(1)),
    (GSM610      ,   Y , Y , N , Channels::from_u8(1)),
    (MS_GSM      ,   Y , Y , N , Channels::from_u8(1)),
    (ILBC_13K3   ,   N , Y , N , Channels::from_u8(1)),
    (ILBC_15K2   ,   N , Y , N , Channels::from_u8(1)),
    (LPC10       ,   N , Y , N , Channels::from_u8(1)),
    (SpeexNB_5_95K , N , Y , N , Channels::from_u8(1)),
    (SpeexNB_8K  ,   N , Y , N , Channels::from_u8(1)),
    (SpeexNB_11K ,   N , Y , N , Channels::from_u8(1)),
    (SpeexNB_15K ,   N , Y , N , Channels::from_u8(1)),
    (SpeexNB_18_2K , N , Y , N , Channels::from_u8(1)),
    (SpeexNB_24_6K , N , Y , N , Channels::from_u8(1)),
    (SpeexWB_20_6K , N , Y , N , Channels::from_u8(1)),
    (SpeexWNarrow_8K , N , Y , N , Channels::from_u8(1)),
    (IMA_ADPCM   ,   Y , Y , N , Channels::from_u8(2)),
    (MSADPCM4    ,   Y , Y , N , Channels::from_u8(2)),
    (ADPCM4      ,   Y , Y , Y , Channels::from_u8(2)),
    (G728        ,   N , Y , N , Channels::from_u8(1)),
    (G729        ,   N , Y , N , Channels::from_u8(1)),
    (G729A       ,   N , Y , N , Channels::from_u8(1)),
    (G729B       ,   N , Y , N , Channels::from_u8(1)),
    (G729AB      ,   N , Y , N , Channels::from_u8(1)),
    (G7231       ,   N , Y , N , Channels::from_u8(1)),
    (G7231_5_3K  ,   N , Y , N , Channels::from_u8(1)),
    (G7231A_6_3K ,   N , Y , N , Channels::from_u8(1)),
    (G7231A_5_3K ,   N , Y , N , Channels::from_u8(1)),
}

payload_types! {
    (G711_ALAW_64K , "G.711-ALaw-64k"           , 0  , 8000),
    (G711_ULAW_64K , "G.711-uLaw-64k"           , 8  , 8000),
    (PCM16         , "PCM-16"                   , _  , 8000),
    (LINEAR16_8K   , "Linear-16-Mono-8kHz"      , 11 , 8000),
    (PCM_S8        , "PCM-S8"                   , _  , 8000),
    (PCM_U8        , "PCM-U8"                   , _  , 8000),
    (LINEAR16_16K  , "Linear-16-Mono-16kHz"     , _  ,16000),
    (G726_40K      , "G.726-40k"                , _  , 8000),
    (G726_32K      , "G.726-32k"                , _  , 8000),
    (G726_24K      , "G.726-24k"                , _  , 8000),
    (G726_16K      , "G.726-16k"                , _  , 8000),
    (GSM_0610      , "GSM-06.10"                , 3  , 8000),
    (MS_GSM        , "MS-GSM"                   , _  , 8000),
    (ILBC_13K3     , "iLBC-13k3"                , _  , 8000),
    (ILBC_15K2     , "iLBC-15k2"                , _  , 8000),
    (LPC10         , "LPC-10"                   , _  , 8000),
    (SPEEX_5_95K   , "SpeexIETFNarrow-5.95k"    , _  , 8000),
    (SPEEX_8K      , "SpeexIETFNarrow-8k"       , _  , 8000),
    (SPEEX_11K     , "SpeexIETFNarrow-11k"      , _  , 8000),
    (SPEEX_15K     , "SpeexIETFNarrow-15k"      , _  , 8000),
    (SPEEX_18_2K   , "SpeexIETFNarrow-18.2k"    , _  , 8000),
    (SPEEX_24_6K   , "SpeexIETFNarrow-24.6k"    , _  , 8000),
    (SPEEXW_8K     , "SpeexWNarrow-8k"          , _  , 8000),
    (SPEEXW_20_6K  , "SpeexIETFWide-20.6k"      , _  ,16000),
    (SPEEXNB       , "SpeexNB"                  , _  , 8000),
    (SPEEXWB       , "SpeexWB"                  , _  ,16000),
    (IMA_ADPCM     , "IMA-ADPCM"                , _  , 8000),
    (MS_ADPCM      , "MS-ADPCM"                 , _  , 8000),
    (VOX_ADPCM     , "VOX-ADPCM"                , _  , 8000),
    (G728          , "G.728"                    , 15 , 8000),
    (G729          , "G.729"                    , 18 , 8000),
    (G729A         , "G.729A"                   , 18 , 8000),
    (G729B         , "G.729B"                   , 18 , 8000),
    (G729AB        , "G.729A/B"                 , 18 , 8000),
    (G7231         , "G.723.1"                  , 4  , 8000),
    (G7231_5_3K    , "G.723.1(5.3k)"            , 4  , 8000),
    (G7231A_6_3K   , "G.723.1A(6.3k)"           , 4  , 8000),
    (G7231A_5_3K   , "G.723.1A(5.3k)"           , 4  , 8000),
}

constant_set! {
    type   = ConstantWithDescription,
    prefix = CallEndReason,
    slice  = ALL_CALL_END_REASONS,

    (EndedByLocalUser, "Local endpoint application cleared call"),
    (EndedByNoAccept, "Local endpoint did not accept call"),
    (EndedByAnswerDenied, "Local endpoint declined to answer call"),
    (EndedByRemoteUser, "Remote endpoint application cleared call"),
    (EndedByRefusal, "Remote endpoint refused call"),
    (EndedByNoAnswer, "Remote endpoint did not answer in required time"),
    (EndedByCallerAbort, "Remote endpoint stopped calling"),
    (EndedByTransportFail, "Transport error cleared call"),
    (EndedByConnectFail, "Transport connection failed to establish call"),
    (EndedByGatekeeper, "Gatekeeper has cleared call"),
    (EndedByNoUser, "Call failed as could not find user (in GK)"),
    (EndedByNoBandwidth, "Call failed as could not get enough bandwidth"),
    (EndedByCapabilityExchange, "Could not find common capabilities"),
    (EndedByCallForwarded, "Call was forwarded using FACILITY message"),
    (EndedBySecurityDenial, "Call failed a security check and was ended"),
    (EndedByLocalBusy, "Local endpoint busy"),
    (EndedByLocalCongestion, "Local endpoint congested"),
    (EndedByRemoteBusy, "Remote endpoint busy"),
    (EndedByRemoteCongestion, "Remote endpoint congested"),
    (EndedByUnreachable, "Could not reach the remote party"),
    (EndedByNoEndPoint, "The remote party is not running an endpoint"),
    (EndedByHostOffline, "The remote party host off line"),
    (EndedByTemporaryFailure, "The remote failed temporarily app may retry"),
    (EndedByQ931Cause, "The remote ended the call with unmapped Q.931 cause code"),
    (EndedByDurationLimit, "Call cleared due to an enforced duration limit"),
    (EndedByInvalidConferenceID, "Call cleared due to invalid conference ID"),
}

constant_set! {
    type   = ConstantWithDescription,
    prefix = AlertingType,
    slice  = ALL_ALERTING_TYPES,

    (Deferred, "Sending of the Alerting message is deferred until the call is answered."),
    (Normal, "An Alerting message is sent to the caller immediately."),
    (WithMedia, "An 'with media' Alerting message is sent to the caller and media channels are started."),
}

constant_set! {
    type   = ConstantWithDescription,
    prefix = RecorderStopReason,
    slice  = ALL_RECORDER_STOP_REASONS,

    (ExplicitRequest, "RecorderStop was called"),
    (MaxDurationExceeded, "Maximum recording time expired"),
    (MaxSilenceDetected, "Maximum silence time was detected"),
}

constant_set! {
    type   = ConstantWithDescription,
    prefix = EStreamBufferStateNotification,
    slice  = ALL_ESTREAM_BUFFER_STATE_NOTIFICATIONS,

    (Underrun, "Buffer has been emptied, no data is available"),
    (Optimum, "Buffer is working optimally"),
    (Overrun, "Buffer is full, no room for new data"),
}

constant_set! {
    type   = ConstantWithDescription,
    prefix = FaxSendSpeed,
    slice  = ALL_FAX_SEND_SPEEDS,

    (V27At2400, "ITU-T V.27 ter at 2400 bps"),
    (V27At4800, "ITU-T V.27 ter at 4800 bps"),
    (V29At7200, "ITU-T V.29 at 7200 bps"),
    (V29At9600, "ITU-T V.29 at 9600 bps"),
    (V17At12000, "ITU-T V.17 at 12000 bps"),
    (V17At14400, "ITU-T V.17 at 14400 bps"),
    (V17At7200, "ITU-T V.17 at 7200 bps"),
    (V17At9600, "ITU-T V.17 at 9600 bps"),
}

constant_set! {
    type   = ConstantWithDescription,
    prefix = FaxReceiveMode,
    slice  = ALL_FAX_RECEIVE_MODES,

    (V27, "ITU-T V.27 ter"),
    (V29, "ITU-T V.29"),
    (V27V29, "ITU-T V.29 and ITU-T V.27 ter"),
    (V17, "ITU-T V.17 and ITU-T V.29 and ITU-T V.27 ter"),
}

constant_set! {
    type   = ConstantWithDescription,
    prefix = DocumentPreparePaperSize,
    slice  = ALL_DOCUMENT_PREPARE_PAPER_SIZES,

    (A4, "ISO A4"),
    (A3, "ISO A3"),
    (B4, "ISO B4"),
    (Legal, "North American Legal"),
    (Letter, "North American Letter"),
}

constant_set! {
    type   = ConstantWithDescription,
    prefix = DocumentPrepareResolution,
    slice  = ALL_DOCUMENT_PREPARE_RESOLUTIONS,

    (Low, "Low resolution 200x100 dpi."),
    (High, "High resolution 200x200 dpi."),
}

constant_set! {
    type   = ConstantWithDescription,
    prefix = DocumentAddFileTransformation,
    slice  = ALL_DOCUMENT_ADD_FILE_TRANSFORMATIONS,

    (Crop, "Crop the image if larger."),
    (Scale, "Scale the image, preserving aspect ratio."),
    (ScaleToFit, "Scale the image to fit the page."),
}

constant_set! {
    type   = ConstantWithDescription,
    prefix = DocumentSaveType,
    slice  = ALL_DOCUMENT_SAVE_TYPES,

    (Auto, "Auto detect the format from the filename. Default."),
    (JPEG, "JPEG image format."),
    (TIFF, "TIFF image format."),
    (PNG, "PNG image format."),
    (BMP, "MS Windows image format."),
    (GIF, "GIF image format."),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payload_type_constants_individual() {
        assert_eq!(PayloadType_G711_ALAW_64K.name, "G.711-ALaw-64k");
        assert_eq!(PayloadType_G711_ALAW_64K.type_code, Some(0));
        assert_eq!(PayloadType_G711_ALAW_64K.sample_rate, 8000);

        assert_eq!(PayloadType_G711_ULAW_64K.name, "G.711-uLaw-64k");
        assert_eq!(PayloadType_G711_ULAW_64K.type_code, Some(8));
        assert_eq!(PayloadType_G711_ULAW_64K.sample_rate, 8000);
    }

    #[test]
    fn payload_type_constants_all() {
        assert!(ALL_PAYLOAD_TYPES.contains(&PayloadType_G711_ALAW_64K));
        assert!(ALL_PAYLOAD_TYPES.contains(&PayloadType_G711_ULAW_64K));
    }

    #[test]
    fn call_end_reason_constants_individual() {
        assert_eq!(CallEndReason_EndedByLocalUser.name, "EndedByLocalUser");
        assert_eq!(
            CallEndReason_EndedByLocalUser.description,
            "Local endpoint application cleared call"
        );
    }

    #[test]
    fn call_end_reason_constants_all() {
        assert!(ALL_CALL_END_REASONS.contains(&CallEndReason_EndedByLocalUser));
        assert!(ALL_CALL_END_REASONS.contains(&CallEndReason_EndedByRemoteUser));
    }

    #[test]
    fn alerting_type_constants_individual() {
        assert_eq!(AlertingType_Deferred.name, "Deferred");
        assert_eq!(
            AlertingType_Deferred.description,
            "Sending of the Alerting message is deferred until the call is answered."
        );
    }

    #[test]
    fn alerting_type_constants_all() {
        assert!(ALL_ALERTING_TYPES.contains(&AlertingType_Deferred));
        assert!(ALL_ALERTING_TYPES.contains(&AlertingType_Normal));
    }

    #[test]
    fn recorder_stop_reason_constants_individual() {
        assert_eq!(RecorderStopReason_ExplicitRequest.name, "ExplicitRequest");
        assert_eq!(
            RecorderStopReason_ExplicitRequest.description,
            "RecorderStop was called"
        );
    }

    #[test]
    fn recorder_stop_reason_constants_all() {
        assert!(ALL_RECORDER_STOP_REASONS.contains(&RecorderStopReason_ExplicitRequest));
        assert!(ALL_RECORDER_STOP_REASONS.contains(&RecorderStopReason_MaxDurationExceeded));
    }

    #[test]
    fn estream_buffer_state_notification_constants_individual() {
        assert_eq!(EStreamBufferStateNotification_Underrun.name, "Underrun");
        assert_eq!(
            EStreamBufferStateNotification_Underrun.description,
            "Buffer has been emptied, no data is available"
        );
    }

    #[test]
    fn estream_buffer_state_notification_constants_all() {
        assert!(ALL_ESTREAM_BUFFER_STATE_NOTIFICATIONS
            .contains(&EStreamBufferStateNotification_Underrun));
        assert!(ALL_ESTREAM_BUFFER_STATE_NOTIFICATIONS
            .contains(&EStreamBufferStateNotification_Overrun));
    }

    #[test]
    fn fax_send_speed_constants_individual() {
        assert_eq!(FaxSendSpeed_V27At2400.name, "V27At2400");
        assert_eq!(
            FaxSendSpeed_V27At2400.description,
            "ITU-T V.27 ter at 2400 bps"
        );
    }

    #[test]
    fn fax_send_speed_constants_all() {
        assert!(ALL_FAX_SEND_SPEEDS.contains(&FaxSendSpeed_V27At2400));
        assert!(ALL_FAX_SEND_SPEEDS.contains(&FaxSendSpeed_V17At14400));
    }

    #[test]
    fn fax_receive_mode_constants_individual() {
        assert_eq!(FaxReceiveMode_V27.name, "V27");
        assert_eq!(FaxReceiveMode_V27.description, "ITU-T V.27 ter");
    }

    #[test]
    fn fax_receive_mode_constants_all() {
        assert!(ALL_FAX_RECEIVE_MODES.contains(&FaxReceiveMode_V27));
        assert!(ALL_FAX_RECEIVE_MODES.contains(&FaxReceiveMode_V17));
    }

    #[test]
    fn document_prepare_paper_size_constants_individual() {
        assert_eq!(DocumentPreparePaperSize_A4.name, "A4");
        assert_eq!(DocumentPreparePaperSize_A4.description, "ISO A4");
    }

    #[test]
    fn document_prepare_paper_size_constants_all() {
        assert!(ALL_DOCUMENT_PREPARE_PAPER_SIZES.contains(&DocumentPreparePaperSize_A4));
        assert!(ALL_DOCUMENT_PREPARE_PAPER_SIZES.contains(&DocumentPreparePaperSize_Legal));
    }

    #[test]
    fn document_prepare_resolution_constants_individual() {
        assert_eq!(DocumentPrepareResolution_Low.name, "Low");
        assert_eq!(
            DocumentPrepareResolution_Low.description,
            "Low resolution 200x100 dpi."
        );
    }

    #[test]
    fn document_prepare_resolution_constants_all() {
        assert!(ALL_DOCUMENT_PREPARE_RESOLUTIONS.contains(&DocumentPrepareResolution_Low));
        assert!(ALL_DOCUMENT_PREPARE_RESOLUTIONS.contains(&DocumentPrepareResolution_High));
    }

    #[test]
    fn document_add_file_transformation_constants_individual() {
        assert_eq!(DocumentAddFileTransformation_Crop.name, "Crop");
        assert_eq!(
            DocumentAddFileTransformation_Crop.description,
            "Crop the image if larger."
        );
    }

    #[test]
    fn document_add_file_transformation_constants_all() {
        assert!(ALL_DOCUMENT_ADD_FILE_TRANSFORMATIONS.contains(&DocumentAddFileTransformation_Crop));
        assert!(ALL_DOCUMENT_ADD_FILE_TRANSFORMATIONS
            .contains(&DocumentAddFileTransformation_ScaleToFit));
    }

    #[test]
    fn document_save_type_constants_individual() {
        assert_eq!(DocumentSaveType_Auto.name, "Auto");
        assert_eq!(
            DocumentSaveType_Auto.description,
            "Auto detect the format from the filename. Default."
        );
    }

    #[test]
    fn document_save_type_constants_all() {
        assert!(ALL_DOCUMENT_SAVE_TYPES.contains(&DocumentSaveType_Auto));
        assert!(ALL_DOCUMENT_SAVE_TYPES.contains(&DocumentSaveType_JPEG));
    }
}
