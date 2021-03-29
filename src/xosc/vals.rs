use crate::generic::*;
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct StatusFreqRange(pub u8);
impl StatusFreqRange {
    pub const _1_15MHZ: Self = Self(0);
    pub const RESERVED_1: Self = Self(0x01);
    pub const RESERVED_2: Self = Self(0x02);
    pub const RESERVED_3: Self = Self(0x03);
}
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct CtrlFreqRange(pub u16);
impl CtrlFreqRange {
    pub const _1_15MHZ: Self = Self(0x0aa0);
    pub const RESERVED_1: Self = Self(0x0aa1);
    pub const RESERVED_2: Self = Self(0x0aa2);
    pub const RESERVED_3: Self = Self(0x0aa3);
}
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct CtrlEnable(pub u16);
impl CtrlEnable {
    pub const DISABLE: Self = Self(0x0d1e);
    pub const ENABLE: Self = Self(0x0fab);
}
