#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InstL {
    #[doc = "No instruction"]
    NONE = 0,
    #[doc = "4-bit instruction"]
    _4B = 0x01,
    #[doc = "8-bit instruction"]
    _8B = 0x02,
    #[doc = "16-bit instruction"]
    _16B = 0x03,
}
impl InstL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InstL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InstL {
    #[inline(always)]
    fn from(val: u8) -> InstL {
        InstL::from_bits(val)
    }
}
impl From<InstL> for u8 {
    #[inline(always)]
    fn from(val: InstL) -> u8 {
        InstL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SpiFrf {
    #[doc = "Standard 1-bit SPI frame format; 1 bit per SCK, full-duplex"]
    STD = 0,
    #[doc = "Dual-SPI frame format; two bits per SCK, half-duplex"]
    DUAL = 0x01,
    #[doc = "Quad-SPI frame format; four bits per SCK, half-duplex"]
    QUAD = 0x02,
    _RESERVED_3 = 0x03,
}
impl SpiFrf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpiFrf {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpiFrf {
    #[inline(always)]
    fn from(val: u8) -> SpiFrf {
        SpiFrf::from_bits(val)
    }
}
impl From<SpiFrf> for u8 {
    #[inline(always)]
    fn from(val: SpiFrf) -> u8 {
        SpiFrf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tmod {
    #[doc = "Both transmit and receive"]
    TX_AND_RX = 0,
    #[doc = "Transmit only (not for FRF == 0, standard SPI mode)"]
    TX_ONLY = 0x01,
    #[doc = "Receive only (not for FRF == 0, standard SPI mode)"]
    RX_ONLY = 0x02,
    #[doc = "EEPROM read mode (TX then RX; RX starts after control data TX'd)"]
    EEPROM_READ = 0x03,
}
impl Tmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmod {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmod {
    #[inline(always)]
    fn from(val: u8) -> Tmod {
        Tmod::from_bits(val)
    }
}
impl From<Tmod> for u8 {
    #[inline(always)]
    fn from(val: Tmod) -> u8 {
        Tmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TransType {
    #[doc = "Command and address both in standard SPI frame format"]
    _1C1A = 0,
    #[doc = "Command in standard SPI format, address in format specified by FRF"]
    _1C2A = 0x01,
    #[doc = "Command and address both in format specified by FRF (e.g. Dual-SPI)"]
    _2C2A = 0x02,
    _RESERVED_3 = 0x03,
}
impl TransType {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TransType {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TransType {
    #[inline(always)]
    fn from(val: u8) -> TransType {
        TransType::from_bits(val)
    }
}
impl From<TransType> for u8 {
    #[inline(always)]
    fn from(val: TransType) -> u8 {
        TransType::to_bits(val)
    }
}
