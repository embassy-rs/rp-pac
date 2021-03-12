use crate::generic::*;
#[doc = "DMA Channel 1 Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct TreqSel(u8);
impl TreqSel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> TreqSel {
        TreqSel(val)
    }
    #[doc = "Select Timer 0 as TREQ"]
    pub const TIMER0: Self = Self(0x3b);
    #[doc = "Select Timer 1 as TREQ"]
    pub const TIMER1: Self = Self(0x3c);
    #[doc = "Select Timer 2 as TREQ (Optional)"]
    pub const TIMER2: Self = Self(0x3d);
    #[doc = "Select Timer 3 as TREQ (Optional)"]
    pub const TIMER3: Self = Self(0x3e);
    #[doc = "Permanent request, for unpaced transfers."]
    pub const PERMANENT: Self = Self(0x3f);
}
#[doc = "DMA Channel 1 Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DataSize(u8);
impl DataSize {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> DataSize {
        DataSize(val)
    }
    pub const SIZE_BYTE: Self = Self(0);
    pub const SIZE_HALFWORD: Self = Self(0x01);
    pub const SIZE_WORD: Self = Self(0x02);
}
#[doc = "Sniffer Control"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct SniffCtrlCalc(u8);
impl SniffCtrlCalc {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> SniffCtrlCalc {
        SniffCtrlCalc(val)
    }
    #[doc = "Calculate a CRC-32 (IEEE802.3 polynomial)"]
    pub const CRC32: Self = Self(0);
    #[doc = "Calculate a CRC-32 (IEEE802.3 polynomial) with bit reversed data"]
    pub const CRC32R: Self = Self(0x01);
    #[doc = "Calculate a CRC-16-CCITT"]
    pub const CRC16: Self = Self(0x02);
    #[doc = "Calculate a CRC-16-CCITT with bit reversed data"]
    pub const CRC16R: Self = Self(0x03);
    #[doc = "XOR reduction over all data. == 1 if the total 1 population count is odd."]
    pub const EVEN: Self = Self(0x0e);
    #[doc = "Calculate a simple 32-bit checksum (addition with a 32 bit accumulator)"]
    pub const SUM: Self = Self(0x0f);
}
