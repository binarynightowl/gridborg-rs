pub type SessionId = u32;
pub type ResourceId = u32;
pub type SampleRate = u16;
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Channels {
    Mono = 1,
    Stereo = 2,
}

impl Channels {
    pub const fn from_u8(value: u8) -> Self {
        match value {
            1 => Channels::Mono,
            2 => Channels::Stereo,
            _ => panic!("Invalid value for Channels"),
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ECM {
    No = 0,
    ECM64 = 64,
    ECM128 = 120,
}

impl ECM {
    pub const fn from_u8(value: u8) -> Self {
        match value {
            0 => ECM::No,
            64 => ECM::ECM64,
            128 => ECM::ECM128,
            _ => panic!("Invalid value for ECM"),
        }
    }
}
