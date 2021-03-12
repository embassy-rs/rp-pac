use crate::generic::*;
#[doc = "Voltage select. Per bank control"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct VoltageSelect(u8);
impl VoltageSelect {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> VoltageSelect {
        VoltageSelect(val)
    }
    #[doc = "Set voltage to 3.3V (DVDD >= 2V5)"]
    pub const _3V3: Self = Self(0);
    #[doc = "Set voltage to 1.8V (DVDD <= 1V8)"]
    pub const _1V8: Self = Self(0x01);
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Drive(u8);
impl Drive {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Drive {
        Drive(val)
    }
    pub const _2MA: Self = Self(0);
    pub const _4MA: Self = Self(0x01);
    pub const _8MA: Self = Self(0x02);
    pub const _12MA: Self = Self(0x03);
}
