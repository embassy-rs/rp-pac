#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct StatusFreqRange(pub u8);
impl StatusFreqRange {
    pub const _1_15MHZ: Self = Self(0);
    pub const RESERVED_1: Self = Self(0x01);
    pub const RESERVED_2: Self = Self(0x02);
    pub const RESERVED_3: Self = Self(0x03);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Enable(pub u16);
impl Enable {
    pub const DISABLE: Self = Self(0x0d1e);
    pub const ENABLE: Self = Self(0x0fab);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CtrlFreqRange(pub u16);
impl CtrlFreqRange {
    pub const _1_15MHZ: Self = Self(0x0aa0);
    pub const RESERVED_1: Self = Self(0x0aa1);
    pub const RESERVED_2: Self = Self(0x0aa2);
    pub const RESERVED_3: Self = Self(0x0aa3);
}
