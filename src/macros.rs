#[macro_export]
macro_rules! audio_formats {
    ( $( ($codec:ident , $wav:tt , $raw:tt , $vap:tt , $ch:expr) ),* $(,)? ) => {
        $(
            audio_formats!(@maybe_const WAV   $wav  $codec $ch);
            audio_formats!(@maybe_const RAW   $raw  $codec $ch);
            audio_formats!(@maybe_const VAP   $vap  $codec $ch);
        )*
    };

    // ── helper ── emit constant only if flag == Y
    (@maybe_const $prefix:ident Y $codec:ident $ch:expr) => {
        paste::paste! {
            #[allow(non_upper_case_globals)]
            pub const [<$prefix _ $codec>]: $crate::constants::AudioFormatType =
                $crate::constants::AudioFormatType {
                    name: concat!(stringify!($prefix), "_", stringify!($codec)),
                    channels: $ch,
                };
        }
    };
    (@maybe_const $prefix:ident N $codec:ident $ch:expr) => {};   // nothing
}

/// Declare payload-type constants.
///
/// * `$ident` – legal Rust identifier that will become the constant suffix.
/// * `$label` – the human-readable codec name (string literal).
/// * `$code`  – RTP payload-type code, or `_` if none.
/// * `$rate`  – sample-rate in Hertz.
///
/// The macro turns `(G711_ALAW_64K, "G.711-ALaw-64k", 0, 8000)` into
/// `pub const PT_G711_ALAW_64K: PayloadType = …`, and collects every
/// constant into `ALL_PAYLOAD_TYPES`.
#[macro_export]
macro_rules! payload_types {
    ( $( ($ident:ident , $label:literal , $code:tt , $rate:expr) ),* $(,)? ) => {
        paste! {
            $(
                pub const [<PayloadType_ $ident>]: $crate::constants::PayloadType = $crate::constants::PayloadType {
                    name: $label,
                    type_code: payload_types!(@code $code),
                    sample_rate: $rate,
                };
            )*

            pub const ALL_PAYLOAD_TYPES: &[$crate::constants::PayloadType] = &[
                $([<PayloadType_ $ident>]),*
            ];
        }
    };

    // helpers ────────────────────────────────────────
    (@code _)          => { None };
    (@code $n:literal) => { Some($n) };
}

#[macro_export]
macro_rules! constant_set {
    (
        type   = $ty:ident ,
        prefix = $prefix:ident ,
        slice  = $slice:ident ,
        $( ($ident:ident , $desc:literal) ),* $(,)?
    ) => {
        paste::paste! {
            $(
                pub const [<$prefix _ $ident>]: $crate::constants::$ty =
                    $crate::constants::$ty {
                        name: stringify!($ident),
                        description: $desc,
                    };
            )*

            pub const $slice: &[$crate::constants::$ty] = &[
                $([<$prefix _ $ident>]),*
            ];
        }
    };
}

#[macro_export]
macro_rules! play_tones {
    (
        $( ($ident:ident, $label:literal, $f1:literal, $f2:literal, $on:tt, $off:tt) ),* $(,)?
    ) => {
        paste::paste! {
            $(
                #[allow(non_upper_case_globals)]
                pub const [<PlayTone_ $ident>]: $crate::constants::ToneType =
                    $crate::constants::ToneType {
                        name: $label,
                        f1: $f1,
                        f2: $f2,
                        on_ms: play_tones!(@opt $on),
                        off_ms: play_tones!(@opt $off),
                    };
            )*

            pub const ALL_PLAY_TONES: &[$crate::constants::ToneType] = &[
                $( [<PlayTone_ $ident>], )*
            ];
        }
    };

    // helper to turn `_` into None, literal into Some(literal)
    (@opt _)          => { None };
    (@opt $n:literal) => { Some($n) };
}