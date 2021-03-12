use crate::generic::*;
#[doc = "Crystal Oscillator Control"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct CtrlFreqRange(u16);
impl CtrlFreqRange {
    pub const fn to_bits(&self) -> u16 {
        self.0
    }
    pub const fn from_bits(val: u16) -> CtrlFreqRange {
        CtrlFreqRange(val)
    }
    pub const _1_15MHZ: Self = Self(0x0aa0);
    pub const RESERVED_1: Self = Self(0x0aa1);
    pub const RESERVED_2: Self = Self(0x0aa2);
    pub const RESERVED_3: Self = Self(0x0aa3);
}
#[doc = "Crystal Oscillator Control"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct CtrlEnable(u16);
impl CtrlEnable {
    pub const fn to_bits(&self) -> u16 {
        self.0
    }
    pub const fn from_bits(val: u16) -> CtrlEnable {
        CtrlEnable(val)
    }
    pub const DISABLE: Self = Self(0x0d1e);
    pub const ENABLE: Self = Self(0x0fab);
}
#[doc = "Crystal Oscillator Status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct StatusFreqRange(u8);
impl StatusFreqRange {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> StatusFreqRange {
        StatusFreqRange(val)
    }
    pub const _1_15MHZ: Self = Self(0);
    pub const RESERVED_1: Self = Self(0x01);
    pub const RESERVED_2: Self = Self(0x02);
    pub const RESERVED_3: Self = Self(0x03);
}
