#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Perfsel(pub u8);
impl Perfsel {
    pub const APB_CONTESTED: Self = Self(0);
    pub const APB: Self = Self(0x01);
    pub const FASTPERI_CONTESTED: Self = Self(0x02);
    pub const FASTPERI: Self = Self(0x03);
    pub const SRAM5_CONTESTED: Self = Self(0x04);
    pub const SRAM5: Self = Self(0x05);
    pub const SRAM4_CONTESTED: Self = Self(0x06);
    pub const SRAM4: Self = Self(0x07);
    pub const SRAM3_CONTESTED: Self = Self(0x08);
    pub const SRAM3: Self = Self(0x09);
    pub const SRAM2_CONTESTED: Self = Self(0x0a);
    pub const SRAM2: Self = Self(0x0b);
    pub const SRAM1_CONTESTED: Self = Self(0x0c);
    pub const SRAM1: Self = Self(0x0d);
    pub const SRAM0_CONTESTED: Self = Self(0x0e);
    pub const SRAM0: Self = Self(0x0f);
    pub const XIP_MAIN_CONTESTED: Self = Self(0x10);
    pub const XIP_MAIN: Self = Self(0x11);
    pub const ROM_CONTESTED: Self = Self(0x12);
    pub const ROM: Self = Self(0x13);
}
