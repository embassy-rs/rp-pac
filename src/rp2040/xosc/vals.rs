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
impl core::fmt::Debug for CtrlFreqRange {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0aa0 => f.write_str("_1_15MHZ"),
            0x0aa1 => f.write_str("RESERVED_1"),
            0x0aa2 => f.write_str("RESERVED_2"),
            0x0aa3 => f.write_str("RESERVED_3"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CtrlFreqRange {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0aa0 => defmt::write!(f, "_1_15MHZ"),
            0x0aa1 => defmt::write!(f, "RESERVED_1"),
            0x0aa2 => defmt::write!(f, "RESERVED_2"),
            0x0aa3 => defmt::write!(f, "RESERVED_3"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
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
impl core::fmt::Debug for Dormant {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x636f_6d61 => f.write_str("DORMANT"),
            0x7761_6b65 => f.write_str("WAKE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dormant {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x636f_6d61 => defmt::write!(f, "DORMANT"),
            0x7761_6b65 => defmt::write!(f, "WAKE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
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
impl core::fmt::Debug for Enable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0d1e => f.write_str("DISABLE"),
            0x0fab => f.write_str("ENABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enable {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0d1e => defmt::write!(f, "DISABLE"),
            0x0fab => defmt::write!(f, "ENABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusFreqRange {
    _1_15MHZ = 0x0,
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
