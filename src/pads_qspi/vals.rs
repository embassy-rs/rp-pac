use crate::generic::*;
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
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd2Drive(pub u8);
impl GpioQspiSd2Drive {
    pub const _2MA: Self = Self(0);
    pub const _4MA: Self = Self(0x01);
    pub const _8MA: Self = Self(0x02);
    pub const _12MA: Self = Self(0x03);
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd0Drive(pub u8);
impl GpioQspiSd0Drive {
    pub const _2MA: Self = Self(0);
    pub const _4MA: Self = Self(0x01);
    pub const _8MA: Self = Self(0x02);
    pub const _12MA: Self = Self(0x03);
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSsDrive(pub u8);
impl GpioQspiSsDrive {
    pub const _2MA: Self = Self(0);
    pub const _4MA: Self = Self(0x01);
    pub const _8MA: Self = Self(0x02);
    pub const _12MA: Self = Self(0x03);
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSclkDrive(pub u8);
impl GpioQspiSclkDrive {
    pub const _2MA: Self = Self(0);
    pub const _4MA: Self = Self(0x01);
    pub const _8MA: Self = Self(0x02);
    pub const _12MA: Self = Self(0x03);
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd1Drive(pub u8);
impl GpioQspiSd1Drive {
    pub const _2MA: Self = Self(0);
    pub const _4MA: Self = Self(0x01);
    pub const _8MA: Self = Self(0x02);
    pub const _12MA: Self = Self(0x03);
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd3Drive(pub u8);
impl GpioQspiSd3Drive {
    pub const _2MA: Self = Self(0);
    pub const _4MA: Self = Self(0x01);
    pub const _8MA: Self = Self(0x02);
    pub const _12MA: Self = Self(0x03);
}
