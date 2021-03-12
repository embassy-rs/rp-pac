use crate::generic::*;
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
#[doc = "The FREQA & FREQB registers control the frequency by controlling the drive strength of each stage The drive strength has 4 levels determined by the number of bits set Increasing the number of bits set increases the drive strength and increases the oscillation frequency 0 bits set is the default drive strength 1 bit set doubles the drive strength 2 bits set triples drive strength 3 bits set quadruples drive strength"]
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
