#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Speed {
    _RESERVED_0 = 0,
    #[doc = "Standard Speed mode of operation"]
    STANDARD = 0x01,
    #[doc = "Fast or Fast Plus mode of operation"]
    FAST = 0x02,
    #[doc = "High Speed mode of operation"]
    HIGH = 0x03,
}
impl Speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Speed {
    #[inline(always)]
    fn from(val: u8) -> Speed {
        Speed::from_bits(val)
    }
}
impl From<Speed> for u8 {
    #[inline(always)]
    fn from(val: Speed) -> u8 {
        Speed::to_bits(val)
    }
}
