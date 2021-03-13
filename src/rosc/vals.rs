use crate::generic::*;
#[doc = "Ring Oscillator control"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct CtrlEnable(pub u16);
impl CtrlEnable {
    pub const DISABLE: Self = Self(0x0d1e);
    pub const ENABLE: Self = Self(0x0fab);
}
#[doc = "The FREQA & FREQB registers control the frequency by controlling the drive strength of each stage The drive strength has 4 levels determined by the number of bits set Increasing the number of bits set increases the drive strength and increases the oscillation frequency 0 bits set is the default drive strength 1 bit set doubles the drive strength 2 bits set triples drive strength 3 bits set quadruples drive strength"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Passwd(pub u16);
impl Passwd {
    pub const PASS: Self = Self(0x9696);
}
#[doc = "Ring Oscillator control"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct CtrlFreqRange(pub u16);
impl CtrlFreqRange {
    pub const LOW: Self = Self(0x0fa4);
    pub const MEDIUM: Self = Self(0x0fa5);
    pub const HIGH: Self = Self(0x0fa7);
    pub const TOOHIGH: Self = Self(0x0fa6);
}
#[doc = "Controls the output divider"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Div(pub u16);
impl Div {
    pub const PASS: Self = Self(0x0aa0);
}
