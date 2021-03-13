use crate::generic::*;
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Drive(pub u8);
impl Drive {
    pub const _2MA: Self = Self(0);
    pub const _4MA: Self = Self(0x01);
    pub const _8MA: Self = Self(0x02);
    pub const _12MA: Self = Self(0x03);
}
#[doc = "Voltage select. Per bank control"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct VoltageSelect(pub u8);
impl VoltageSelect {
    #[doc = "Set voltage to 3.3V (DVDD >= 2V5)"]
    pub const _3V3: Self = Self(0);
    #[doc = "Set voltage to 1.8V (DVDD <= 1V8)"]
    pub const _1V8: Self = Self(0x01);
}
