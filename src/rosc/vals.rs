#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Div(pub u16);
impl Div {
    pub const PASS: Self = Self(0x0aa0);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct FreqRange(pub u16);
impl FreqRange {
    pub const LOW: Self = Self(0x0fa4);
    pub const MEDIUM: Self = Self(0x0fa5);
    pub const TOOHIGH: Self = Self(0x0fa6);
    pub const HIGH: Self = Self(0x0fa7);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Passwd(pub u16);
impl Passwd {
    pub const PASS: Self = Self(0x9696);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Enable(pub u16);
impl Enable {
    pub const DISABLE: Self = Self(0x0d1e);
    pub const ENABLE: Self = Self(0x0fab);
}
