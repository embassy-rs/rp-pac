#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CtrlFreqRange(pub u16);
impl CtrlFreqRange {
    pub const _1_15MHZ: Self = Self(0x0aa0);
    pub const _10_30MHZ: Self = Self(0x0aa1);
    pub const _25_60MHZ: Self = Self(0x0aa2);
    pub const _40_100MHZ: Self = Self(0x0aa3);
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
pub struct Dormant(pub u32);
impl Dormant {
    pub const DORMANT: Self = Self(0x636f_6d61);
    pub const WAKE: Self = Self(0x7761_6b65);
}
impl Dormant {
    pub const fn from_bits(val: u32) -> Dormant {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Dormant {
    #[inline(always)]
    fn from(val: u32) -> Dormant {
        Dormant::from_bits(val)
    }
}
impl From<Dormant> for u32 {
    #[inline(always)]
    fn from(val: Dormant) -> u32 {
        Dormant::to_bits(val)
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
    _10_30MHZ = 0x01,
    _25_60MHZ = 0x02,
    _40_100MHZ = 0x03,
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
