#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Div(pub u16);
impl Div {
    pub const PASS: Self = Self(0xaa00);
}
impl Div {
    pub const fn from_bits(val: u16) -> Div {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl From<u16> for Div {
    #[inline(always)]
    fn from(val: u16) -> Div {
        Div::from_bits(val)
    }
}
impl From<Div> for u16 {
    #[inline(always)]
    fn from(val: Div) -> u16 {
        Div::to_bits(val)
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct FreqRange(pub u16);
impl FreqRange {
    pub const LOW: Self = Self(0x0fa4);
    pub const MEDIUM: Self = Self(0x0fa5);
    pub const TOOHIGH: Self = Self(0x0fa6);
    pub const HIGH: Self = Self(0x0fa7);
}
impl FreqRange {
    pub const fn from_bits(val: u16) -> FreqRange {
        Self(val & 0x0fff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl From<u16> for FreqRange {
    #[inline(always)]
    fn from(val: u16) -> FreqRange {
        FreqRange::from_bits(val)
    }
}
impl From<FreqRange> for u16 {
    #[inline(always)]
    fn from(val: FreqRange) -> u16 {
        FreqRange::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Passwd(pub u16);
impl Passwd {
    pub const PASS: Self = Self(0x9696);
}
impl Passwd {
    pub const fn from_bits(val: u16) -> Passwd {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl From<u16> for Passwd {
    #[inline(always)]
    fn from(val: u16) -> Passwd {
        Passwd::from_bits(val)
    }
}
impl From<Passwd> for u16 {
    #[inline(always)]
    fn from(val: Passwd) -> u16 {
        Passwd::to_bits(val)
    }
}
