use crate::generic::*;
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSsCtrlFuncsel(u8);
impl GpioQspiSsCtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> GpioQspiSsCtrlFuncsel {
        GpioQspiSsCtrlFuncsel(val)
    }
    pub const XIP_SS_N: Self = Self(0);
    pub const SIO_31: Self = Self(0x05);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd2CtrlFuncsel(u8);
impl GpioQspiSd2CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> GpioQspiSd2CtrlFuncsel {
        GpioQspiSd2CtrlFuncsel(val)
    }
    pub const XIP_SD2: Self = Self(0);
    pub const SIO_34: Self = Self(0x05);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd1CtrlFuncsel(u8);
impl GpioQspiSd1CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> GpioQspiSd1CtrlFuncsel {
        GpioQspiSd1CtrlFuncsel(val)
    }
    pub const XIP_SD1: Self = Self(0);
    pub const SIO_33: Self = Self(0x05);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSclkCtrlFuncsel(u8);
impl GpioQspiSclkCtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> GpioQspiSclkCtrlFuncsel {
        GpioQspiSclkCtrlFuncsel(val)
    }
    pub const XIP_SCLK: Self = Self(0);
    pub const SIO_30: Self = Self(0x05);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd0CtrlFuncsel(u8);
impl GpioQspiSd0CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> GpioQspiSd0CtrlFuncsel {
        GpioQspiSd0CtrlFuncsel(val)
    }
    pub const XIP_SD0: Self = Self(0);
    pub const SIO_32: Self = Self(0x05);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd3CtrlFuncsel(u8);
impl GpioQspiSd3CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> GpioQspiSd3CtrlFuncsel {
        GpioQspiSd3CtrlFuncsel(val)
    }
    pub const XIP_SD3: Self = Self(0);
    pub const SIO_35: Self = Self(0x05);
    pub const NULL: Self = Self(0x1f);
}
