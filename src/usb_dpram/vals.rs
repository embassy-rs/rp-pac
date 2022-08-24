#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EpControlEndpointType(pub u8);
impl EpControlEndpointType {
    pub const CONTROL: Self = Self(0);
    pub const ISOCHRONOUS: Self = Self(0x01);
    pub const BULK: Self = Self(0x02);
    pub const INTERRUPT: Self = Self(0x03);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EpBufferControlDoubleBufferIsoOffset(pub u8);
impl EpBufferControlDoubleBufferIsoOffset {
    pub const _128: Self = Self(0);
    pub const _256: Self = Self(0x01);
    pub const _512: Self = Self(0x02);
    pub const _1024: Self = Self(0x03);
}
