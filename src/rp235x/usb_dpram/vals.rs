#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EpBufferControlDoubleBufferIsoOffset {
    _128 = 0,
    _256 = 0x01,
    _512 = 0x02,
    _1024 = 0x03,
}
impl EpBufferControlDoubleBufferIsoOffset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EpBufferControlDoubleBufferIsoOffset {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EpBufferControlDoubleBufferIsoOffset {
    #[inline(always)]
    fn from(val: u8) -> EpBufferControlDoubleBufferIsoOffset {
        EpBufferControlDoubleBufferIsoOffset::from_bits(val)
    }
}
impl From<EpBufferControlDoubleBufferIsoOffset> for u8 {
    #[inline(always)]
    fn from(val: EpBufferControlDoubleBufferIsoOffset) -> u8 {
        EpBufferControlDoubleBufferIsoOffset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EpControlEndpointType {
    CONTROL = 0,
    ISOCHRONOUS = 0x01,
    BULK = 0x02,
    INTERRUPT = 0x03,
}
impl EpControlEndpointType {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EpControlEndpointType {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EpControlEndpointType {
    #[inline(always)]
    fn from(val: u8) -> EpControlEndpointType {
        EpControlEndpointType::from_bits(val)
    }
}
impl From<EpControlEndpointType> for u8 {
    #[inline(always)]
    fn from(val: EpControlEndpointType) -> u8 {
        EpControlEndpointType::to_bits(val)
    }
}
