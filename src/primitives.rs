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
