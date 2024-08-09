#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PixShift {
    #[doc = "Do not shift the colour data register."]
    _0 = 0,
    #[doc = "Shift the colour data register by 1 bit"]
    _1 = 0x01,
    #[doc = "Shift the colour data register by 2 bits"]
    _2 = 0x02,
    #[doc = "Shift the colour data register by 4 bits"]
    _4 = 0x03,
    #[doc = "Shift the colour data register by 8 bits"]
    _8 = 0x04,
    #[doc = "Shift the colour data register by 16 bits"]
    _16 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl PixShift {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PixShift {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PixShift {
    #[inline(always)]
    fn from(val: u8) -> PixShift {
        PixShift::from_bits(val)
    }
}
impl From<PixShift> for u8 {
    #[inline(always)]
    fn from(val: PixShift) -> u8 {
        PixShift::to_bits(val)
    }
}
