#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum AddrWidth {
    #[doc = "Single width"]
    S = 0x0,
    #[doc = "Dual width"]
    D = 0x01,
    #[doc = "Quad width"]
    Q = 0x02,
    _RESERVED_3 = 0x03,
}
impl AddrWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AddrWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AddrWidth {
    #[inline(always)]
    fn from(val: u8) -> AddrWidth {
        AddrWidth::from_bits(val)
    }
}
impl From<AddrWidth> for u8 {
    #[inline(always)]
    fn from(val: AddrWidth) -> u8 {
        AddrWidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DataWidth {
    #[doc = "Single width"]
    S = 0x0,
    #[doc = "Dual width"]
    D = 0x01,
    #[doc = "Quad width"]
    Q = 0x02,
    _RESERVED_3 = 0x03,
}
impl DataWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DataWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DataWidth {
    #[inline(always)]
    fn from(val: u8) -> DataWidth {
        DataWidth::from_bits(val)
    }
}
impl From<DataWidth> for u8 {
    #[inline(always)]
    fn from(val: DataWidth) -> u8 {
        DataWidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DummyLen {
    #[doc = "No dummy phase"]
    NONE = 0x0,
    #[doc = "4 dummy bits"]
    _4 = 0x01,
    #[doc = "8 dummy bits"]
    _8 = 0x02,
    #[doc = "12 dummy bits"]
    _12 = 0x03,
    #[doc = "16 dummy bits"]
    _16 = 0x04,
    #[doc = "20 dummy bits"]
    _20 = 0x05,
    #[doc = "24 dummy bits"]
    _24 = 0x06,
    #[doc = "28 dummy bits"]
    _28 = 0x07,
}
impl DummyLen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DummyLen {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DummyLen {
    #[inline(always)]
    fn from(val: u8) -> DummyLen {
        DummyLen::from_bits(val)
    }
}
impl From<DummyLen> for u8 {
    #[inline(always)]
    fn from(val: DummyLen) -> u8 {
        DummyLen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DummyWidth {
    #[doc = "Single width"]
    S = 0x0,
    #[doc = "Dual width"]
    D = 0x01,
    #[doc = "Quad width"]
    Q = 0x02,
    _RESERVED_3 = 0x03,
}
impl DummyWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DummyWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DummyWidth {
    #[inline(always)]
    fn from(val: u8) -> DummyWidth {
        DummyWidth::from_bits(val)
    }
}
impl From<DummyWidth> for u8 {
    #[inline(always)]
    fn from(val: DummyWidth) -> u8 {
        DummyWidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Iwidth {
    #[doc = "Single width"]
    S = 0x0,
    #[doc = "Dual width"]
    D = 0x01,
    #[doc = "Quad width"]
    Q = 0x02,
    _RESERVED_3 = 0x03,
}
impl Iwidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iwidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iwidth {
    #[inline(always)]
    fn from(val: u8) -> Iwidth {
        Iwidth::from_bits(val)
    }
}
impl From<Iwidth> for u8 {
    #[inline(always)]
    fn from(val: Iwidth) -> u8 {
        Iwidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pagebreak {
    #[doc = "No page boundary is enforced"]
    NONE = 0x0,
    #[doc = "Break bursts crossing a 256-byte page boundary"]
    _256 = 0x01,
    #[doc = "Break bursts crossing a 1024-byte quad-page boundary"]
    _1024 = 0x02,
    #[doc = "Break bursts crossing a 4096-byte sector boundary"]
    _4096 = 0x03,
}
impl Pagebreak {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pagebreak {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pagebreak {
    #[inline(always)]
    fn from(val: u8) -> Pagebreak {
        Pagebreak::from_bits(val)
    }
}
impl From<Pagebreak> for u8 {
    #[inline(always)]
    fn from(val: Pagebreak) -> u8 {
        Pagebreak::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PrefixLen {
    #[doc = "No prefix"]
    NONE = 0x0,
    #[doc = "8-bit prefix"]
    _8 = 0x01,
}
impl PrefixLen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PrefixLen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PrefixLen {
    #[inline(always)]
    fn from(val: u8) -> PrefixLen {
        PrefixLen::from_bits(val)
    }
}
impl From<PrefixLen> for u8 {
    #[inline(always)]
    fn from(val: PrefixLen) -> u8 {
        PrefixLen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PrefixWidth {
    #[doc = "Single width"]
    S = 0x0,
    #[doc = "Dual width"]
    D = 0x01,
    #[doc = "Quad width"]
    Q = 0x02,
    _RESERVED_3 = 0x03,
}
impl PrefixWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PrefixWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PrefixWidth {
    #[inline(always)]
    fn from(val: u8) -> PrefixWidth {
        PrefixWidth::from_bits(val)
    }
}
impl From<PrefixWidth> for u8 {
    #[inline(always)]
    fn from(val: PrefixWidth) -> u8 {
        PrefixWidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SuffixLen {
    #[doc = "No suffix"]
    NONE = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "8-bit suffix"]
    _8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl SuffixLen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SuffixLen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SuffixLen {
    #[inline(always)]
    fn from(val: u8) -> SuffixLen {
        SuffixLen::from_bits(val)
    }
}
impl From<SuffixLen> for u8 {
    #[inline(always)]
    fn from(val: SuffixLen) -> u8 {
        SuffixLen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SuffixWidth {
    #[doc = "Single width"]
    S = 0x0,
    #[doc = "Dual width"]
    D = 0x01,
    #[doc = "Quad width"]
    Q = 0x02,
    _RESERVED_3 = 0x03,
}
impl SuffixWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SuffixWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SuffixWidth {
    #[inline(always)]
    fn from(val: u8) -> SuffixWidth {
        SuffixWidth::from_bits(val)
    }
}
impl From<SuffixWidth> for u8 {
    #[inline(always)]
    fn from(val: SuffixWidth) -> u8 {
        SuffixWidth::to_bits(val)
    }
}
