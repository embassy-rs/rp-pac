#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Drive {
    _2M_A = 0x0,
    _4M_A = 0x01,
    _8M_A = 0x02,
    _12M_A = 0x03,
}
impl Drive {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Drive {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Drive {
    #[inline(always)]
    fn from(val: u8) -> Drive {
        Drive::from_bits(val)
    }
}
impl From<Drive> for u8 {
    #[inline(always)]
    fn from(val: Drive) -> u8 {
        Drive::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VoltageSelect {
    #[doc = "Set voltage to 3.3V (DVDD >= 2V5)"]
    _3V3 = 0x0,
    #[doc = "Set voltage to 1.8V (DVDD <= 1V8)"]
    _1V8 = 0x01,
}
impl VoltageSelect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VoltageSelect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VoltageSelect {
    #[inline(always)]
    fn from(val: u8) -> VoltageSelect {
        VoltageSelect::from_bits(val)
    }
}
impl From<VoltageSelect> for u8 {
    #[inline(always)]
    fn from(val: VoltageSelect) -> u8 {
        VoltageSelect::to_bits(val)
    }
}
