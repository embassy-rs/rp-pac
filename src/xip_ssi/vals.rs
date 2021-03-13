use crate::generic::*;
#[doc = "SPI control"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct SpiCtrlr0InstL(pub u8);
impl SpiCtrlr0InstL {
    #[doc = "No instruction"]
    pub const NONE: Self = Self(0);
    #[doc = "4-bit instruction"]
    pub const _4B: Self = Self(0x01);
    #[doc = "8-bit instruction"]
    pub const _8B: Self = Self(0x02);
    #[doc = "16-bit instruction"]
    pub const _16B: Self = Self(0x03);
}
#[doc = "Control register 0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ctrlr0Tmod(pub u8);
impl Ctrlr0Tmod {
    #[doc = "Both transmit and receive"]
    pub const TX_AND_RX: Self = Self(0);
    #[doc = "Transmit only (not for FRF == 0, standard SPI mode)"]
    pub const TX_ONLY: Self = Self(0x01);
    #[doc = "Receive only (not for FRF == 0, standard SPI mode)"]
    pub const RX_ONLY: Self = Self(0x02);
    #[doc = "EEPROM read mode (TX then RX; RX starts after control data TX'd)"]
    pub const EEPROM_READ: Self = Self(0x03);
}
#[doc = "SPI control"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct SpiCtrlr0TransType(pub u8);
impl SpiCtrlr0TransType {
    #[doc = "Command and address both in standard SPI frame format"]
    pub const _1C1A: Self = Self(0);
    #[doc = "Command in standard SPI format, address in format specified by FRF"]
    pub const _1C2A: Self = Self(0x01);
    #[doc = "Command and address both in format specified by FRF (e.g. Dual-SPI)"]
    pub const _2C2A: Self = Self(0x02);
}
#[doc = "Control register 0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ctrlr0SpiFrf(pub u8);
impl Ctrlr0SpiFrf {
    #[doc = "Standard 1-bit SPI frame format; 1 bit per SCK, full-duplex"]
    pub const STD: Self = Self(0);
    #[doc = "Dual-SPI frame format; two bits per SCK, half-duplex"]
    pub const DUAL: Self = Self(0x01);
    #[doc = "Quad-SPI frame format; four bits per SCK, half-duplex"]
    pub const QUAD: Self = Self(0x02);
}
