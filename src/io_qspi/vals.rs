use crate::generic::*;
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSsCtrlOutover(pub u8);
impl GpioQspiSsCtrlOutover {
    #[doc = "drive output from peripheral signal selected by funcsel"]
    pub const NORMAL: Self = Self(0);
    #[doc = "drive output from inverse of peripheral signal selected by funcsel"]
    pub const INVERT: Self = Self(0x01);
    #[doc = "drive output low"]
    pub const LOW: Self = Self(0x02);
    #[doc = "drive output high"]
    pub const HIGH: Self = Self(0x03);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd1CtrlInover(pub u8);
impl GpioQspiSd1CtrlInover {
    #[doc = "don't invert the peri input"]
    pub const NORMAL: Self = Self(0);
    #[doc = "invert the peri input"]
    pub const INVERT: Self = Self(0x01);
    #[doc = "drive peri input low"]
    pub const LOW: Self = Self(0x02);
    #[doc = "drive peri input high"]
    pub const HIGH: Self = Self(0x03);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd2CtrlInover(pub u8);
impl GpioQspiSd2CtrlInover {
    #[doc = "don't invert the peri input"]
    pub const NORMAL: Self = Self(0);
    #[doc = "invert the peri input"]
    pub const INVERT: Self = Self(0x01);
    #[doc = "drive peri input low"]
    pub const LOW: Self = Self(0x02);
    #[doc = "drive peri input high"]
    pub const HIGH: Self = Self(0x03);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd0CtrlFuncsel(pub u8);
impl GpioQspiSd0CtrlFuncsel {
    pub const XIP_SD0: Self = Self(0);
    pub const SIO_32: Self = Self(0x05);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd3CtrlOutover(pub u8);
impl GpioQspiSd3CtrlOutover {
    #[doc = "drive output from peripheral signal selected by funcsel"]
    pub const NORMAL: Self = Self(0);
    #[doc = "drive output from inverse of peripheral signal selected by funcsel"]
    pub const INVERT: Self = Self(0x01);
    #[doc = "drive output low"]
    pub const LOW: Self = Self(0x02);
    #[doc = "drive output high"]
    pub const HIGH: Self = Self(0x03);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd1CtrlIrqover(pub u8);
impl GpioQspiSd1CtrlIrqover {
    #[doc = "don't invert the interrupt"]
    pub const NORMAL: Self = Self(0);
    #[doc = "invert the interrupt"]
    pub const INVERT: Self = Self(0x01);
    #[doc = "drive interrupt low"]
    pub const LOW: Self = Self(0x02);
    #[doc = "drive interrupt high"]
    pub const HIGH: Self = Self(0x03);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd2CtrlIrqover(pub u8);
impl GpioQspiSd2CtrlIrqover {
    #[doc = "don't invert the interrupt"]
    pub const NORMAL: Self = Self(0);
    #[doc = "invert the interrupt"]
    pub const INVERT: Self = Self(0x01);
    #[doc = "drive interrupt low"]
    pub const LOW: Self = Self(0x02);
    #[doc = "drive interrupt high"]
    pub const HIGH: Self = Self(0x03);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSclkCtrlIrqover(pub u8);
impl GpioQspiSclkCtrlIrqover {
    #[doc = "don't invert the interrupt"]
    pub const NORMAL: Self = Self(0);
    #[doc = "invert the interrupt"]
    pub const INVERT: Self = Self(0x01);
    #[doc = "drive interrupt low"]
    pub const LOW: Self = Self(0x02);
    #[doc = "drive interrupt high"]
    pub const HIGH: Self = Self(0x03);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd3CtrlIrqover(pub u8);
impl GpioQspiSd3CtrlIrqover {
    #[doc = "don't invert the interrupt"]
    pub const NORMAL: Self = Self(0);
    #[doc = "invert the interrupt"]
    pub const INVERT: Self = Self(0x01);
    #[doc = "drive interrupt low"]
    pub const LOW: Self = Self(0x02);
    #[doc = "drive interrupt high"]
    pub const HIGH: Self = Self(0x03);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd0CtrlInover(pub u8);
impl GpioQspiSd0CtrlInover {
    #[doc = "don't invert the peri input"]
    pub const NORMAL: Self = Self(0);
    #[doc = "invert the peri input"]
    pub const INVERT: Self = Self(0x01);
    #[doc = "drive peri input low"]
    pub const LOW: Self = Self(0x02);
    #[doc = "drive peri input high"]
    pub const HIGH: Self = Self(0x03);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd0CtrlOeover(pub u8);
impl GpioQspiSd0CtrlOeover {
    #[doc = "drive output enable from peripheral signal selected by funcsel"]
    pub const NORMAL: Self = Self(0);
    #[doc = "drive output enable from inverse of peripheral signal selected by funcsel"]
    pub const INVERT: Self = Self(0x01);
    #[doc = "disable output"]
    pub const DISABLE: Self = Self(0x02);
    #[doc = "enable output"]
    pub const ENABLE: Self = Self(0x03);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSclkCtrlFuncsel(pub u8);
impl GpioQspiSclkCtrlFuncsel {
    pub const XIP_SCLK: Self = Self(0);
    pub const SIO_30: Self = Self(0x05);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSclkCtrlInover(pub u8);
impl GpioQspiSclkCtrlInover {
    #[doc = "don't invert the peri input"]
    pub const NORMAL: Self = Self(0);
    #[doc = "invert the peri input"]
    pub const INVERT: Self = Self(0x01);
    #[doc = "drive peri input low"]
    pub const LOW: Self = Self(0x02);
    #[doc = "drive peri input high"]
    pub const HIGH: Self = Self(0x03);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd1CtrlOeover(pub u8);
impl GpioQspiSd1CtrlOeover {
    #[doc = "drive output enable from peripheral signal selected by funcsel"]
    pub const NORMAL: Self = Self(0);
    #[doc = "drive output enable from inverse of peripheral signal selected by funcsel"]
    pub const INVERT: Self = Self(0x01);
    #[doc = "disable output"]
    pub const DISABLE: Self = Self(0x02);
    #[doc = "enable output"]
    pub const ENABLE: Self = Self(0x03);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd0CtrlOutover(pub u8);
impl GpioQspiSd0CtrlOutover {
    #[doc = "drive output from peripheral signal selected by funcsel"]
    pub const NORMAL: Self = Self(0);
    #[doc = "drive output from inverse of peripheral signal selected by funcsel"]
    pub const INVERT: Self = Self(0x01);
    #[doc = "drive output low"]
    pub const LOW: Self = Self(0x02);
    #[doc = "drive output high"]
    pub const HIGH: Self = Self(0x03);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd3CtrlInover(pub u8);
impl GpioQspiSd3CtrlInover {
    #[doc = "don't invert the peri input"]
    pub const NORMAL: Self = Self(0);
    #[doc = "invert the peri input"]
    pub const INVERT: Self = Self(0x01);
    #[doc = "drive peri input low"]
    pub const LOW: Self = Self(0x02);
    #[doc = "drive peri input high"]
    pub const HIGH: Self = Self(0x03);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd3CtrlFuncsel(pub u8);
impl GpioQspiSd3CtrlFuncsel {
    pub const XIP_SD3: Self = Self(0);
    pub const SIO_35: Self = Self(0x05);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd3CtrlOeover(pub u8);
impl GpioQspiSd3CtrlOeover {
    #[doc = "drive output enable from peripheral signal selected by funcsel"]
    pub const NORMAL: Self = Self(0);
    #[doc = "drive output enable from inverse of peripheral signal selected by funcsel"]
    pub const INVERT: Self = Self(0x01);
    #[doc = "disable output"]
    pub const DISABLE: Self = Self(0x02);
    #[doc = "enable output"]
    pub const ENABLE: Self = Self(0x03);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd2CtrlOutover(pub u8);
impl GpioQspiSd2CtrlOutover {
    #[doc = "drive output from peripheral signal selected by funcsel"]
    pub const NORMAL: Self = Self(0);
    #[doc = "drive output from inverse of peripheral signal selected by funcsel"]
    pub const INVERT: Self = Self(0x01);
    #[doc = "drive output low"]
    pub const LOW: Self = Self(0x02);
    #[doc = "drive output high"]
    pub const HIGH: Self = Self(0x03);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSsCtrlOeover(pub u8);
impl GpioQspiSsCtrlOeover {
    #[doc = "drive output enable from peripheral signal selected by funcsel"]
    pub const NORMAL: Self = Self(0);
    #[doc = "drive output enable from inverse of peripheral signal selected by funcsel"]
    pub const INVERT: Self = Self(0x01);
    #[doc = "disable output"]
    pub const DISABLE: Self = Self(0x02);
    #[doc = "enable output"]
    pub const ENABLE: Self = Self(0x03);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd2CtrlOeover(pub u8);
impl GpioQspiSd2CtrlOeover {
    #[doc = "drive output enable from peripheral signal selected by funcsel"]
    pub const NORMAL: Self = Self(0);
    #[doc = "drive output enable from inverse of peripheral signal selected by funcsel"]
    pub const INVERT: Self = Self(0x01);
    #[doc = "disable output"]
    pub const DISABLE: Self = Self(0x02);
    #[doc = "enable output"]
    pub const ENABLE: Self = Self(0x03);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSsCtrlIrqover(pub u8);
impl GpioQspiSsCtrlIrqover {
    #[doc = "don't invert the interrupt"]
    pub const NORMAL: Self = Self(0);
    #[doc = "invert the interrupt"]
    pub const INVERT: Self = Self(0x01);
    #[doc = "drive interrupt low"]
    pub const LOW: Self = Self(0x02);
    #[doc = "drive interrupt high"]
    pub const HIGH: Self = Self(0x03);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSclkCtrlOutover(pub u8);
impl GpioQspiSclkCtrlOutover {
    #[doc = "drive output from peripheral signal selected by funcsel"]
    pub const NORMAL: Self = Self(0);
    #[doc = "drive output from inverse of peripheral signal selected by funcsel"]
    pub const INVERT: Self = Self(0x01);
    #[doc = "drive output low"]
    pub const LOW: Self = Self(0x02);
    #[doc = "drive output high"]
    pub const HIGH: Self = Self(0x03);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd2CtrlFuncsel(pub u8);
impl GpioQspiSd2CtrlFuncsel {
    pub const XIP_SD2: Self = Self(0);
    pub const SIO_34: Self = Self(0x05);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSsCtrlFuncsel(pub u8);
impl GpioQspiSsCtrlFuncsel {
    pub const XIP_SS_N: Self = Self(0);
    pub const SIO_31: Self = Self(0x05);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd1CtrlFuncsel(pub u8);
impl GpioQspiSd1CtrlFuncsel {
    pub const XIP_SD1: Self = Self(0);
    pub const SIO_33: Self = Self(0x05);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd0CtrlIrqover(pub u8);
impl GpioQspiSd0CtrlIrqover {
    #[doc = "don't invert the interrupt"]
    pub const NORMAL: Self = Self(0);
    #[doc = "invert the interrupt"]
    pub const INVERT: Self = Self(0x01);
    #[doc = "drive interrupt low"]
    pub const LOW: Self = Self(0x02);
    #[doc = "drive interrupt high"]
    pub const HIGH: Self = Self(0x03);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd1CtrlOutover(pub u8);
impl GpioQspiSd1CtrlOutover {
    #[doc = "drive output from peripheral signal selected by funcsel"]
    pub const NORMAL: Self = Self(0);
    #[doc = "drive output from inverse of peripheral signal selected by funcsel"]
    pub const INVERT: Self = Self(0x01);
    #[doc = "drive output low"]
    pub const LOW: Self = Self(0x02);
    #[doc = "drive output high"]
    pub const HIGH: Self = Self(0x03);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSsCtrlInover(pub u8);
impl GpioQspiSsCtrlInover {
    #[doc = "don't invert the peri input"]
    pub const NORMAL: Self = Self(0);
    #[doc = "invert the peri input"]
    pub const INVERT: Self = Self(0x01);
    #[doc = "drive peri input low"]
    pub const LOW: Self = Self(0x02);
    #[doc = "drive peri input high"]
    pub const HIGH: Self = Self(0x03);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSclkCtrlOeover(pub u8);
impl GpioQspiSclkCtrlOeover {
    #[doc = "drive output enable from peripheral signal selected by funcsel"]
    pub const NORMAL: Self = Self(0);
    #[doc = "drive output enable from inverse of peripheral signal selected by funcsel"]
    pub const INVERT: Self = Self(0x01);
    #[doc = "disable output"]
    pub const DISABLE: Self = Self(0x02);
    #[doc = "enable output"]
    pub const ENABLE: Self = Self(0x03);
}
