use crate::generic::*;
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkSysCtrlSrc(pub u8);
impl ClkSysCtrlSrc {
    pub const CLK_REF: Self = Self(0);
    pub const CLKSRC_CLK_SYS_AUX: Self = Self(0x01);
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkGpout2CtrlAuxsrc(pub u8);
impl ClkGpout2CtrlAuxsrc {
    pub const CLKSRC_PLL_SYS: Self = Self(0);
    pub const CLKSRC_GPIN0: Self = Self(0x01);
    pub const CLKSRC_GPIN1: Self = Self(0x02);
    pub const CLKSRC_PLL_USB: Self = Self(0x03);
    pub const ROSC_CLKSRC_PH: Self = Self(0x04);
    pub const XOSC_CLKSRC: Self = Self(0x05);
    pub const CLK_SYS: Self = Self(0x06);
    pub const CLK_USB: Self = Self(0x07);
    pub const CLK_ADC: Self = Self(0x08);
    pub const CLK_RTC: Self = Self(0x09);
    pub const CLK_REF: Self = Self(0x0a);
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkUsbCtrlAuxsrc(pub u8);
impl ClkUsbCtrlAuxsrc {
    pub const CLKSRC_PLL_USB: Self = Self(0);
    pub const CLKSRC_PLL_SYS: Self = Self(0x01);
    pub const ROSC_CLKSRC_PH: Self = Self(0x02);
    pub const XOSC_CLKSRC: Self = Self(0x03);
    pub const CLKSRC_GPIN0: Self = Self(0x04);
    pub const CLKSRC_GPIN1: Self = Self(0x05);
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkGpout0CtrlAuxsrc(pub u8);
impl ClkGpout0CtrlAuxsrc {
    pub const CLKSRC_PLL_SYS: Self = Self(0);
    pub const CLKSRC_GPIN0: Self = Self(0x01);
    pub const CLKSRC_GPIN1: Self = Self(0x02);
    pub const CLKSRC_PLL_USB: Self = Self(0x03);
    pub const ROSC_CLKSRC: Self = Self(0x04);
    pub const XOSC_CLKSRC: Self = Self(0x05);
    pub const CLK_SYS: Self = Self(0x06);
    pub const CLK_USB: Self = Self(0x07);
    pub const CLK_ADC: Self = Self(0x08);
    pub const CLK_RTC: Self = Self(0x09);
    pub const CLK_REF: Self = Self(0x0a);
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkGpout3CtrlAuxsrc(pub u8);
impl ClkGpout3CtrlAuxsrc {
    pub const CLKSRC_PLL_SYS: Self = Self(0);
    pub const CLKSRC_GPIN0: Self = Self(0x01);
    pub const CLKSRC_GPIN1: Self = Self(0x02);
    pub const CLKSRC_PLL_USB: Self = Self(0x03);
    pub const ROSC_CLKSRC_PH: Self = Self(0x04);
    pub const XOSC_CLKSRC: Self = Self(0x05);
    pub const CLK_SYS: Self = Self(0x06);
    pub const CLK_USB: Self = Self(0x07);
    pub const CLK_ADC: Self = Self(0x08);
    pub const CLK_RTC: Self = Self(0x09);
    pub const CLK_REF: Self = Self(0x0a);
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkAdcCtrlAuxsrc(pub u8);
impl ClkAdcCtrlAuxsrc {
    pub const CLKSRC_PLL_USB: Self = Self(0);
    pub const CLKSRC_PLL_SYS: Self = Self(0x01);
    pub const ROSC_CLKSRC_PH: Self = Self(0x02);
    pub const XOSC_CLKSRC: Self = Self(0x03);
    pub const CLKSRC_GPIN0: Self = Self(0x04);
    pub const CLKSRC_GPIN1: Self = Self(0x05);
}
#[doc = "Clock sent to frequency counter, set to 0 when not required Writing to this register initiates the frequency count"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Fc0Src(pub u8);
impl Fc0Src {
    pub const NULL: Self = Self(0);
    pub const PLL_SYS_CLKSRC_PRIMARY: Self = Self(0x01);
    pub const PLL_USB_CLKSRC_PRIMARY: Self = Self(0x02);
    pub const ROSC_CLKSRC: Self = Self(0x03);
    pub const ROSC_CLKSRC_PH: Self = Self(0x04);
    pub const XOSC_CLKSRC: Self = Self(0x05);
    pub const CLKSRC_GPIN0: Self = Self(0x06);
    pub const CLKSRC_GPIN1: Self = Self(0x07);
    pub const CLK_REF: Self = Self(0x08);
    pub const CLK_SYS: Self = Self(0x09);
    pub const CLK_PERI: Self = Self(0x0a);
    pub const CLK_USB: Self = Self(0x0b);
    pub const CLK_ADC: Self = Self(0x0c);
    pub const CLK_RTC: Self = Self(0x0d);
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkRefCtrlAuxsrc(pub u8);
impl ClkRefCtrlAuxsrc {
    pub const CLKSRC_PLL_USB: Self = Self(0);
    pub const CLKSRC_GPIN0: Self = Self(0x01);
    pub const CLKSRC_GPIN1: Self = Self(0x02);
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkSysCtrlAuxsrc(pub u8);
impl ClkSysCtrlAuxsrc {
    pub const CLKSRC_PLL_SYS: Self = Self(0);
    pub const CLKSRC_PLL_USB: Self = Self(0x01);
    pub const ROSC_CLKSRC: Self = Self(0x02);
    pub const XOSC_CLKSRC: Self = Self(0x03);
    pub const CLKSRC_GPIN0: Self = Self(0x04);
    pub const CLKSRC_GPIN1: Self = Self(0x05);
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkGpout1CtrlAuxsrc(pub u8);
impl ClkGpout1CtrlAuxsrc {
    pub const CLKSRC_PLL_SYS: Self = Self(0);
    pub const CLKSRC_GPIN0: Self = Self(0x01);
    pub const CLKSRC_GPIN1: Self = Self(0x02);
    pub const CLKSRC_PLL_USB: Self = Self(0x03);
    pub const ROSC_CLKSRC: Self = Self(0x04);
    pub const XOSC_CLKSRC: Self = Self(0x05);
    pub const CLK_SYS: Self = Self(0x06);
    pub const CLK_USB: Self = Self(0x07);
    pub const CLK_ADC: Self = Self(0x08);
    pub const CLK_RTC: Self = Self(0x09);
    pub const CLK_REF: Self = Self(0x0a);
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkPeriCtrlAuxsrc(pub u8);
impl ClkPeriCtrlAuxsrc {
    pub const CLK_SYS: Self = Self(0);
    pub const CLKSRC_PLL_SYS: Self = Self(0x01);
    pub const CLKSRC_PLL_USB: Self = Self(0x02);
    pub const ROSC_CLKSRC_PH: Self = Self(0x03);
    pub const XOSC_CLKSRC: Self = Self(0x04);
    pub const CLKSRC_GPIN0: Self = Self(0x05);
    pub const CLKSRC_GPIN1: Self = Self(0x06);
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkRtcCtrlAuxsrc(pub u8);
impl ClkRtcCtrlAuxsrc {
    pub const CLKSRC_PLL_USB: Self = Self(0);
    pub const CLKSRC_PLL_SYS: Self = Self(0x01);
    pub const ROSC_CLKSRC_PH: Self = Self(0x02);
    pub const XOSC_CLKSRC: Self = Self(0x03);
    pub const CLKSRC_GPIN0: Self = Self(0x04);
    pub const CLKSRC_GPIN1: Self = Self(0x05);
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkRefCtrlSrc(pub u8);
impl ClkRefCtrlSrc {
    pub const ROSC_CLKSRC_PH: Self = Self(0);
    pub const CLKSRC_CLK_REF_AUX: Self = Self(0x01);
    pub const XOSC_CLKSRC: Self = Self(0x02);
}
