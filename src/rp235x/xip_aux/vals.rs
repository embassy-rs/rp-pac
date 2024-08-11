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
