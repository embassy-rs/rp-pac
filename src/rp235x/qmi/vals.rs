#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Iwidth {
    #[doc = "Single width"]
    S = 0,
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
pub enum M0rfmtAddrWidth {
    #[doc = "Single width"]
    S = 0,
    #[doc = "Dual width"]
    D = 0x01,
    #[doc = "Quad width"]
    Q = 0x02,
    _RESERVED_3 = 0x03,
}
impl M0rfmtAddrWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M0rfmtAddrWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M0rfmtAddrWidth {
    #[inline(always)]
    fn from(val: u8) -> M0rfmtAddrWidth {
        M0rfmtAddrWidth::from_bits(val)
    }
}
impl From<M0rfmtAddrWidth> for u8 {
    #[inline(always)]
    fn from(val: M0rfmtAddrWidth) -> u8 {
        M0rfmtAddrWidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M0rfmtDataWidth {
    #[doc = "Single width"]
    S = 0,
    #[doc = "Dual width"]
    D = 0x01,
    #[doc = "Quad width"]
    Q = 0x02,
    _RESERVED_3 = 0x03,
}
impl M0rfmtDataWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M0rfmtDataWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M0rfmtDataWidth {
    #[inline(always)]
    fn from(val: u8) -> M0rfmtDataWidth {
        M0rfmtDataWidth::from_bits(val)
    }
}
impl From<M0rfmtDataWidth> for u8 {
    #[inline(always)]
    fn from(val: M0rfmtDataWidth) -> u8 {
        M0rfmtDataWidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M0rfmtDummyLen {
    #[doc = "No dummy phase"]
    NONE = 0,
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
impl M0rfmtDummyLen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M0rfmtDummyLen {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M0rfmtDummyLen {
    #[inline(always)]
    fn from(val: u8) -> M0rfmtDummyLen {
        M0rfmtDummyLen::from_bits(val)
    }
}
impl From<M0rfmtDummyLen> for u8 {
    #[inline(always)]
    fn from(val: M0rfmtDummyLen) -> u8 {
        M0rfmtDummyLen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M0rfmtDummyWidth {
    #[doc = "Single width"]
    S = 0,
    #[doc = "Dual width"]
    D = 0x01,
    #[doc = "Quad width"]
    Q = 0x02,
    _RESERVED_3 = 0x03,
}
impl M0rfmtDummyWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M0rfmtDummyWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M0rfmtDummyWidth {
    #[inline(always)]
    fn from(val: u8) -> M0rfmtDummyWidth {
        M0rfmtDummyWidth::from_bits(val)
    }
}
impl From<M0rfmtDummyWidth> for u8 {
    #[inline(always)]
    fn from(val: M0rfmtDummyWidth) -> u8 {
        M0rfmtDummyWidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M0rfmtPrefixLen {
    #[doc = "No prefix"]
    NONE = 0,
    #[doc = "8-bit prefix"]
    _8 = 0x01,
}
impl M0rfmtPrefixLen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M0rfmtPrefixLen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M0rfmtPrefixLen {
    #[inline(always)]
    fn from(val: u8) -> M0rfmtPrefixLen {
        M0rfmtPrefixLen::from_bits(val)
    }
}
impl From<M0rfmtPrefixLen> for u8 {
    #[inline(always)]
    fn from(val: M0rfmtPrefixLen) -> u8 {
        M0rfmtPrefixLen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M0rfmtPrefixWidth {
    #[doc = "Single width"]
    S = 0,
    #[doc = "Dual width"]
    D = 0x01,
    #[doc = "Quad width"]
    Q = 0x02,
    _RESERVED_3 = 0x03,
}
impl M0rfmtPrefixWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M0rfmtPrefixWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M0rfmtPrefixWidth {
    #[inline(always)]
    fn from(val: u8) -> M0rfmtPrefixWidth {
        M0rfmtPrefixWidth::from_bits(val)
    }
}
impl From<M0rfmtPrefixWidth> for u8 {
    #[inline(always)]
    fn from(val: M0rfmtPrefixWidth) -> u8 {
        M0rfmtPrefixWidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M0rfmtSuffixLen {
    #[doc = "No suffix"]
    NONE = 0,
    _RESERVED_1 = 0x01,
    #[doc = "8-bit suffix"]
    _8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl M0rfmtSuffixLen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M0rfmtSuffixLen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M0rfmtSuffixLen {
    #[inline(always)]
    fn from(val: u8) -> M0rfmtSuffixLen {
        M0rfmtSuffixLen::from_bits(val)
    }
}
impl From<M0rfmtSuffixLen> for u8 {
    #[inline(always)]
    fn from(val: M0rfmtSuffixLen) -> u8 {
        M0rfmtSuffixLen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M0rfmtSuffixWidth {
    #[doc = "Single width"]
    S = 0,
    #[doc = "Dual width"]
    D = 0x01,
    #[doc = "Quad width"]
    Q = 0x02,
    _RESERVED_3 = 0x03,
}
impl M0rfmtSuffixWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M0rfmtSuffixWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M0rfmtSuffixWidth {
    #[inline(always)]
    fn from(val: u8) -> M0rfmtSuffixWidth {
        M0rfmtSuffixWidth::from_bits(val)
    }
}
impl From<M0rfmtSuffixWidth> for u8 {
    #[inline(always)]
    fn from(val: M0rfmtSuffixWidth) -> u8 {
        M0rfmtSuffixWidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M0timingPagebreak {
    #[doc = "No page boundary is enforced"]
    NONE = 0,
    #[doc = "Break bursts crossing a 256-byte page boundary"]
    _256 = 0x01,
    #[doc = "Break bursts crossing a 1024-byte quad-page boundary"]
    _1024 = 0x02,
    #[doc = "Break bursts crossing a 4096-byte sector boundary"]
    _4096 = 0x03,
}
impl M0timingPagebreak {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M0timingPagebreak {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M0timingPagebreak {
    #[inline(always)]
    fn from(val: u8) -> M0timingPagebreak {
        M0timingPagebreak::from_bits(val)
    }
}
impl From<M0timingPagebreak> for u8 {
    #[inline(always)]
    fn from(val: M0timingPagebreak) -> u8 {
        M0timingPagebreak::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M0wfmtAddrWidth {
    #[doc = "Single width"]
    S = 0,
    #[doc = "Dual width"]
    D = 0x01,
    #[doc = "Quad width"]
    Q = 0x02,
    _RESERVED_3 = 0x03,
}
impl M0wfmtAddrWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M0wfmtAddrWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M0wfmtAddrWidth {
    #[inline(always)]
    fn from(val: u8) -> M0wfmtAddrWidth {
        M0wfmtAddrWidth::from_bits(val)
    }
}
impl From<M0wfmtAddrWidth> for u8 {
    #[inline(always)]
    fn from(val: M0wfmtAddrWidth) -> u8 {
        M0wfmtAddrWidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M0wfmtDataWidth {
    #[doc = "Single width"]
    S = 0,
    #[doc = "Dual width"]
    D = 0x01,
    #[doc = "Quad width"]
    Q = 0x02,
    _RESERVED_3 = 0x03,
}
impl M0wfmtDataWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M0wfmtDataWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M0wfmtDataWidth {
    #[inline(always)]
    fn from(val: u8) -> M0wfmtDataWidth {
        M0wfmtDataWidth::from_bits(val)
    }
}
impl From<M0wfmtDataWidth> for u8 {
    #[inline(always)]
    fn from(val: M0wfmtDataWidth) -> u8 {
        M0wfmtDataWidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M0wfmtDummyLen {
    #[doc = "No dummy phase"]
    NONE = 0,
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
impl M0wfmtDummyLen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M0wfmtDummyLen {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M0wfmtDummyLen {
    #[inline(always)]
    fn from(val: u8) -> M0wfmtDummyLen {
        M0wfmtDummyLen::from_bits(val)
    }
}
impl From<M0wfmtDummyLen> for u8 {
    #[inline(always)]
    fn from(val: M0wfmtDummyLen) -> u8 {
        M0wfmtDummyLen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M0wfmtDummyWidth {
    #[doc = "Single width"]
    S = 0,
    #[doc = "Dual width"]
    D = 0x01,
    #[doc = "Quad width"]
    Q = 0x02,
    _RESERVED_3 = 0x03,
}
impl M0wfmtDummyWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M0wfmtDummyWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M0wfmtDummyWidth {
    #[inline(always)]
    fn from(val: u8) -> M0wfmtDummyWidth {
        M0wfmtDummyWidth::from_bits(val)
    }
}
impl From<M0wfmtDummyWidth> for u8 {
    #[inline(always)]
    fn from(val: M0wfmtDummyWidth) -> u8 {
        M0wfmtDummyWidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M0wfmtPrefixLen {
    #[doc = "No prefix"]
    NONE = 0,
    #[doc = "8-bit prefix"]
    _8 = 0x01,
}
impl M0wfmtPrefixLen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M0wfmtPrefixLen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M0wfmtPrefixLen {
    #[inline(always)]
    fn from(val: u8) -> M0wfmtPrefixLen {
        M0wfmtPrefixLen::from_bits(val)
    }
}
impl From<M0wfmtPrefixLen> for u8 {
    #[inline(always)]
    fn from(val: M0wfmtPrefixLen) -> u8 {
        M0wfmtPrefixLen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M0wfmtPrefixWidth {
    #[doc = "Single width"]
    S = 0,
    #[doc = "Dual width"]
    D = 0x01,
    #[doc = "Quad width"]
    Q = 0x02,
    _RESERVED_3 = 0x03,
}
impl M0wfmtPrefixWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M0wfmtPrefixWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M0wfmtPrefixWidth {
    #[inline(always)]
    fn from(val: u8) -> M0wfmtPrefixWidth {
        M0wfmtPrefixWidth::from_bits(val)
    }
}
impl From<M0wfmtPrefixWidth> for u8 {
    #[inline(always)]
    fn from(val: M0wfmtPrefixWidth) -> u8 {
        M0wfmtPrefixWidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M0wfmtSuffixLen {
    #[doc = "No suffix"]
    NONE = 0,
    _RESERVED_1 = 0x01,
    #[doc = "8-bit suffix"]
    _8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl M0wfmtSuffixLen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M0wfmtSuffixLen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M0wfmtSuffixLen {
    #[inline(always)]
    fn from(val: u8) -> M0wfmtSuffixLen {
        M0wfmtSuffixLen::from_bits(val)
    }
}
impl From<M0wfmtSuffixLen> for u8 {
    #[inline(always)]
    fn from(val: M0wfmtSuffixLen) -> u8 {
        M0wfmtSuffixLen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M0wfmtSuffixWidth {
    #[doc = "Single width"]
    S = 0,
    #[doc = "Dual width"]
    D = 0x01,
    #[doc = "Quad width"]
    Q = 0x02,
    _RESERVED_3 = 0x03,
}
impl M0wfmtSuffixWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M0wfmtSuffixWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M0wfmtSuffixWidth {
    #[inline(always)]
    fn from(val: u8) -> M0wfmtSuffixWidth {
        M0wfmtSuffixWidth::from_bits(val)
    }
}
impl From<M0wfmtSuffixWidth> for u8 {
    #[inline(always)]
    fn from(val: M0wfmtSuffixWidth) -> u8 {
        M0wfmtSuffixWidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M1rfmtAddrWidth {
    #[doc = "Single width"]
    S = 0,
    #[doc = "Dual width"]
    D = 0x01,
    #[doc = "Quad width"]
    Q = 0x02,
    _RESERVED_3 = 0x03,
}
impl M1rfmtAddrWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M1rfmtAddrWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M1rfmtAddrWidth {
    #[inline(always)]
    fn from(val: u8) -> M1rfmtAddrWidth {
        M1rfmtAddrWidth::from_bits(val)
    }
}
impl From<M1rfmtAddrWidth> for u8 {
    #[inline(always)]
    fn from(val: M1rfmtAddrWidth) -> u8 {
        M1rfmtAddrWidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M1rfmtDataWidth {
    #[doc = "Single width"]
    S = 0,
    #[doc = "Dual width"]
    D = 0x01,
    #[doc = "Quad width"]
    Q = 0x02,
    _RESERVED_3 = 0x03,
}
impl M1rfmtDataWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M1rfmtDataWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M1rfmtDataWidth {
    #[inline(always)]
    fn from(val: u8) -> M1rfmtDataWidth {
        M1rfmtDataWidth::from_bits(val)
    }
}
impl From<M1rfmtDataWidth> for u8 {
    #[inline(always)]
    fn from(val: M1rfmtDataWidth) -> u8 {
        M1rfmtDataWidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M1rfmtDummyLen {
    #[doc = "No dummy phase"]
    NONE = 0,
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
impl M1rfmtDummyLen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M1rfmtDummyLen {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M1rfmtDummyLen {
    #[inline(always)]
    fn from(val: u8) -> M1rfmtDummyLen {
        M1rfmtDummyLen::from_bits(val)
    }
}
impl From<M1rfmtDummyLen> for u8 {
    #[inline(always)]
    fn from(val: M1rfmtDummyLen) -> u8 {
        M1rfmtDummyLen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M1rfmtDummyWidth {
    #[doc = "Single width"]
    S = 0,
    #[doc = "Dual width"]
    D = 0x01,
    #[doc = "Quad width"]
    Q = 0x02,
    _RESERVED_3 = 0x03,
}
impl M1rfmtDummyWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M1rfmtDummyWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M1rfmtDummyWidth {
    #[inline(always)]
    fn from(val: u8) -> M1rfmtDummyWidth {
        M1rfmtDummyWidth::from_bits(val)
    }
}
impl From<M1rfmtDummyWidth> for u8 {
    #[inline(always)]
    fn from(val: M1rfmtDummyWidth) -> u8 {
        M1rfmtDummyWidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M1rfmtPrefixLen {
    #[doc = "No prefix"]
    NONE = 0,
    #[doc = "8-bit prefix"]
    _8 = 0x01,
}
impl M1rfmtPrefixLen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M1rfmtPrefixLen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M1rfmtPrefixLen {
    #[inline(always)]
    fn from(val: u8) -> M1rfmtPrefixLen {
        M1rfmtPrefixLen::from_bits(val)
    }
}
impl From<M1rfmtPrefixLen> for u8 {
    #[inline(always)]
    fn from(val: M1rfmtPrefixLen) -> u8 {
        M1rfmtPrefixLen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M1rfmtPrefixWidth {
    #[doc = "Single width"]
    S = 0,
    #[doc = "Dual width"]
    D = 0x01,
    #[doc = "Quad width"]
    Q = 0x02,
    _RESERVED_3 = 0x03,
}
impl M1rfmtPrefixWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M1rfmtPrefixWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M1rfmtPrefixWidth {
    #[inline(always)]
    fn from(val: u8) -> M1rfmtPrefixWidth {
        M1rfmtPrefixWidth::from_bits(val)
    }
}
impl From<M1rfmtPrefixWidth> for u8 {
    #[inline(always)]
    fn from(val: M1rfmtPrefixWidth) -> u8 {
        M1rfmtPrefixWidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M1rfmtSuffixLen {
    #[doc = "No suffix"]
    NONE = 0,
    _RESERVED_1 = 0x01,
    #[doc = "8-bit suffix"]
    _8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl M1rfmtSuffixLen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M1rfmtSuffixLen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M1rfmtSuffixLen {
    #[inline(always)]
    fn from(val: u8) -> M1rfmtSuffixLen {
        M1rfmtSuffixLen::from_bits(val)
    }
}
impl From<M1rfmtSuffixLen> for u8 {
    #[inline(always)]
    fn from(val: M1rfmtSuffixLen) -> u8 {
        M1rfmtSuffixLen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M1rfmtSuffixWidth {
    #[doc = "Single width"]
    S = 0,
    #[doc = "Dual width"]
    D = 0x01,
    #[doc = "Quad width"]
    Q = 0x02,
    _RESERVED_3 = 0x03,
}
impl M1rfmtSuffixWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M1rfmtSuffixWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M1rfmtSuffixWidth {
    #[inline(always)]
    fn from(val: u8) -> M1rfmtSuffixWidth {
        M1rfmtSuffixWidth::from_bits(val)
    }
}
impl From<M1rfmtSuffixWidth> for u8 {
    #[inline(always)]
    fn from(val: M1rfmtSuffixWidth) -> u8 {
        M1rfmtSuffixWidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M1timingPagebreak {
    #[doc = "No page boundary is enforced"]
    NONE = 0,
    #[doc = "Break bursts crossing a 256-byte page boundary"]
    _256 = 0x01,
    #[doc = "Break bursts crossing a 1024-byte quad-page boundary"]
    _1024 = 0x02,
    #[doc = "Break bursts crossing a 4096-byte sector boundary"]
    _4096 = 0x03,
}
impl M1timingPagebreak {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M1timingPagebreak {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M1timingPagebreak {
    #[inline(always)]
    fn from(val: u8) -> M1timingPagebreak {
        M1timingPagebreak::from_bits(val)
    }
}
impl From<M1timingPagebreak> for u8 {
    #[inline(always)]
    fn from(val: M1timingPagebreak) -> u8 {
        M1timingPagebreak::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M1wfmtAddrWidth {
    #[doc = "Single width"]
    S = 0,
    #[doc = "Dual width"]
    D = 0x01,
    #[doc = "Quad width"]
    Q = 0x02,
    _RESERVED_3 = 0x03,
}
impl M1wfmtAddrWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M1wfmtAddrWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M1wfmtAddrWidth {
    #[inline(always)]
    fn from(val: u8) -> M1wfmtAddrWidth {
        M1wfmtAddrWidth::from_bits(val)
    }
}
impl From<M1wfmtAddrWidth> for u8 {
    #[inline(always)]
    fn from(val: M1wfmtAddrWidth) -> u8 {
        M1wfmtAddrWidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M1wfmtDataWidth {
    #[doc = "Single width"]
    S = 0,
    #[doc = "Dual width"]
    D = 0x01,
    #[doc = "Quad width"]
    Q = 0x02,
    _RESERVED_3 = 0x03,
}
impl M1wfmtDataWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M1wfmtDataWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M1wfmtDataWidth {
    #[inline(always)]
    fn from(val: u8) -> M1wfmtDataWidth {
        M1wfmtDataWidth::from_bits(val)
    }
}
impl From<M1wfmtDataWidth> for u8 {
    #[inline(always)]
    fn from(val: M1wfmtDataWidth) -> u8 {
        M1wfmtDataWidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M1wfmtDummyLen {
    #[doc = "No dummy phase"]
    NONE = 0,
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
impl M1wfmtDummyLen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M1wfmtDummyLen {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M1wfmtDummyLen {
    #[inline(always)]
    fn from(val: u8) -> M1wfmtDummyLen {
        M1wfmtDummyLen::from_bits(val)
    }
}
impl From<M1wfmtDummyLen> for u8 {
    #[inline(always)]
    fn from(val: M1wfmtDummyLen) -> u8 {
        M1wfmtDummyLen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M1wfmtDummyWidth {
    #[doc = "Single width"]
    S = 0,
    #[doc = "Dual width"]
    D = 0x01,
    #[doc = "Quad width"]
    Q = 0x02,
    _RESERVED_3 = 0x03,
}
impl M1wfmtDummyWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M1wfmtDummyWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M1wfmtDummyWidth {
    #[inline(always)]
    fn from(val: u8) -> M1wfmtDummyWidth {
        M1wfmtDummyWidth::from_bits(val)
    }
}
impl From<M1wfmtDummyWidth> for u8 {
    #[inline(always)]
    fn from(val: M1wfmtDummyWidth) -> u8 {
        M1wfmtDummyWidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M1wfmtPrefixLen {
    #[doc = "No prefix"]
    NONE = 0,
    #[doc = "8-bit prefix"]
    _8 = 0x01,
}
impl M1wfmtPrefixLen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M1wfmtPrefixLen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M1wfmtPrefixLen {
    #[inline(always)]
    fn from(val: u8) -> M1wfmtPrefixLen {
        M1wfmtPrefixLen::from_bits(val)
    }
}
impl From<M1wfmtPrefixLen> for u8 {
    #[inline(always)]
    fn from(val: M1wfmtPrefixLen) -> u8 {
        M1wfmtPrefixLen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M1wfmtPrefixWidth {
    #[doc = "Single width"]
    S = 0,
    #[doc = "Dual width"]
    D = 0x01,
    #[doc = "Quad width"]
    Q = 0x02,
    _RESERVED_3 = 0x03,
}
impl M1wfmtPrefixWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M1wfmtPrefixWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M1wfmtPrefixWidth {
    #[inline(always)]
    fn from(val: u8) -> M1wfmtPrefixWidth {
        M1wfmtPrefixWidth::from_bits(val)
    }
}
impl From<M1wfmtPrefixWidth> for u8 {
    #[inline(always)]
    fn from(val: M1wfmtPrefixWidth) -> u8 {
        M1wfmtPrefixWidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M1wfmtSuffixLen {
    #[doc = "No suffix"]
    NONE = 0,
    _RESERVED_1 = 0x01,
    #[doc = "8-bit suffix"]
    _8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl M1wfmtSuffixLen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M1wfmtSuffixLen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M1wfmtSuffixLen {
    #[inline(always)]
    fn from(val: u8) -> M1wfmtSuffixLen {
        M1wfmtSuffixLen::from_bits(val)
    }
}
impl From<M1wfmtSuffixLen> for u8 {
    #[inline(always)]
    fn from(val: M1wfmtSuffixLen) -> u8 {
        M1wfmtSuffixLen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum M1wfmtSuffixWidth {
    #[doc = "Single width"]
    S = 0,
    #[doc = "Dual width"]
    D = 0x01,
    #[doc = "Quad width"]
    Q = 0x02,
    _RESERVED_3 = 0x03,
}
impl M1wfmtSuffixWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M1wfmtSuffixWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M1wfmtSuffixWidth {
    #[inline(always)]
    fn from(val: u8) -> M1wfmtSuffixWidth {
        M1wfmtSuffixWidth::from_bits(val)
    }
}
impl From<M1wfmtSuffixWidth> for u8 {
    #[inline(always)]
    fn from(val: M1wfmtSuffixWidth) -> u8 {
        M1wfmtSuffixWidth::to_bits(val)
    }
}
