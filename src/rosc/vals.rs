use crate::generic::*;
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct CtrlFreqRange(pub u16);
impl CtrlFreqRange {
    pub const LOW: Self = Self(0x0fa4);
    pub const MEDIUM: Self = Self(0x0fa5);
    pub const HIGH: Self = Self(0x0fa7);
    pub const TOOHIGH: Self = Self(0x0fa6);
}
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct CtrlEnable(pub u16);
impl CtrlEnable {
    pub const DISABLE: Self = Self(0x0d1e);
    pub const ENABLE: Self = Self(0x0fab);
}
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Passwd(pub u16);
impl Passwd {
    pub const PASS: Self = Self(0x9696);
}
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Div(pub u16);
impl Div {
    pub const PASS: Self = Self(0x0aa0);
}
