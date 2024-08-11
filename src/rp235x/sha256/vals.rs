#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DmaSize {
    _8BIT = 0,
    _16BIT = 0x01,
    _32BIT = 0x02,
    _RESERVED_3 = 0x03,
}
impl DmaSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaSize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaSize {
    #[inline(always)]
    fn from(val: u8) -> DmaSize {
        DmaSize::from_bits(val)
    }
}
impl From<DmaSize> for u8 {
    #[inline(always)]
    fn from(val: DmaSize) -> u8 {
        DmaSize::to_bits(val)
    }
}
