#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CtrlFreqRange(pub u16);
impl CtrlFreqRange {
    pub const _1_15MHZ: Self = Self(0x0aa0);
    pub const RESERVED_1: Self = Self(0x0aa1);
    pub const RESERVED_2: Self = Self(0x0aa2);
    pub const RESERVED_3: Self = Self(0x0aa3);
}
impl CtrlFreqRange {
    pub const fn from_bits(val: u16) -> CtrlFreqRange {
        Self(val & 0x0fff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl From<u16> for CtrlFreqRange {
    #[inline(always)]
    fn from(val: u16) -> CtrlFreqRange {
        CtrlFreqRange::from_bits(val)
    }
}
impl From<CtrlFreqRange> for u16 {
    #[inline(always)]
    fn from(val: CtrlFreqRange) -> u16 {
        CtrlFreqRange::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Enable(pub u16);
impl Enable {
    pub const DISABLE: Self = Self(0x0d1e);
    pub const ENABLE: Self = Self(0x0fab);
}
impl Enable {
    pub const fn from_bits(val: u16) -> Enable {
        Self(val & 0x0fff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl From<u16> for Enable {
    #[inline(always)]
    fn from(val: u16) -> Enable {
        Enable::from_bits(val)
    }
}
impl From<Enable> for u16 {
    #[inline(always)]
    fn from(val: Enable) -> u16 {
        Enable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum StatusFreqRange {
    _1_15MHZ = 0,
    RESERVED_1 = 0x01,
    RESERVED_2 = 0x02,
    RESERVED_3 = 0x03,
}
impl StatusFreqRange {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusFreqRange {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusFreqRange {
    #[inline(always)]
    fn from(val: u8) -> StatusFreqRange {
        StatusFreqRange::from_bits(val)
    }
}
impl From<StatusFreqRange> for u8 {
    #[inline(always)]
    fn from(val: StatusFreqRange) -> u8 {
        StatusFreqRange::to_bits(val)
    }
}
