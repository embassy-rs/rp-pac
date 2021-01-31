use crate::generic::*;
#[doc = "Ring Oscillator control"]
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
    pub const LOW: Self = Self(0x0fa4);
    pub const MEDIUM: Self = Self(0x0fa5);
    pub const HIGH: Self = Self(0x0fa7);
    pub const TOOHIGH: Self = Self(0x0fa6);
}
#[doc = "For a detailed description see freqa register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Passwd(u16);
impl Passwd {
    pub const fn to_bits(&self) -> u16 {
        self.0
    }
    pub const fn from_bits(val: u16) -> Passwd {
        Passwd(val)
    }
    pub const PASS: Self = Self(0x9696);
}
#[doc = "Ring Oscillator control"]
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
#[doc = "Controls the output divider"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DivDiv(u16);
impl DivDiv {
    pub const fn to_bits(&self) -> u16 {
        self.0
    }
    pub const fn from_bits(val: u16) -> DivDiv {
        DivDiv(val)
    }
    pub const PASS: Self = Self(0x0aa0);
}
