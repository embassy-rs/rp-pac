#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Perfsel {
    APB_CONTESTED = 0,
    APB = 0x01,
    FASTPERI_CONTESTED = 0x02,
    FASTPERI = 0x03,
    SRAM5_CONTESTED = 0x04,
    SRAM5 = 0x05,
    SRAM4_CONTESTED = 0x06,
    SRAM4 = 0x07,
    SRAM3_CONTESTED = 0x08,
    SRAM3 = 0x09,
    SRAM2_CONTESTED = 0x0a,
    SRAM2 = 0x0b,
    SRAM1_CONTESTED = 0x0c,
    SRAM1 = 0x0d,
    SRAM0_CONTESTED = 0x0e,
    SRAM0 = 0x0f,
    XIP_MAIN_CONTESTED = 0x10,
    XIP_MAIN = 0x11,
    ROM_CONTESTED = 0x12,
    ROM = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Perfsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Perfsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Perfsel {
    #[inline(always)]
    fn from(val: u8) -> Perfsel {
        Perfsel::from_bits(val)
    }
}
impl From<Perfsel> for u8 {
    #[inline(always)]
    fn from(val: Perfsel) -> u8 {
        Perfsel::to_bits(val)
    }
}
