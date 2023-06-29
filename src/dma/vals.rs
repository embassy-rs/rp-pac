#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Calc {
    #[doc = "Calculate a CRC-32 (IEEE802.3 polynomial)"]
    CRC32 = 0,
    #[doc = "Calculate a CRC-32 (IEEE802.3 polynomial) with bit reversed data"]
    CRC32R = 0x01,
    #[doc = "Calculate a CRC-16-CCITT"]
    CRC16 = 0x02,
    #[doc = "Calculate a CRC-16-CCITT with bit reversed data"]
    CRC16R = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "XOR reduction over all data. == 1 if the total 1 population count is odd."]
    EVEN = 0x0e,
    #[doc = "Calculate a simple 32-bit checksum (addition with a 32 bit accumulator)"]
    SUM = 0x0f,
}
impl Calc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Calc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Calc {
    #[inline(always)]
    fn from(val: u8) -> Calc {
        Calc::from_bits(val)
    }
}
impl From<Calc> for u8 {
    #[inline(always)]
    fn from(val: Calc) -> u8 {
        Calc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DataSize {
    SIZE_BYTE = 0,
    SIZE_HALFWORD = 0x01,
    SIZE_WORD = 0x02,
    _RESERVED_3 = 0x03,
}
impl DataSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DataSize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DataSize {
    #[inline(always)]
    fn from(val: u8) -> DataSize {
        DataSize::from_bits(val)
    }
}
impl From<DataSize> for u8 {
    #[inline(always)]
    fn from(val: DataSize) -> u8 {
        DataSize::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TreqSel(pub u8);
impl TreqSel {
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
impl TreqSel {
    pub const fn from_bits(val: u8) -> TreqSel {
        Self(val & 0x3f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for TreqSel {
    #[inline(always)]
    fn from(val: u8) -> TreqSel {
        TreqSel::from_bits(val)
    }
}
impl From<TreqSel> for u8 {
    #[inline(always)]
    fn from(val: TreqSel) -> u8 {
        TreqSel::to_bits(val)
    }
}
