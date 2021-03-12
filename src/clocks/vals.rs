use crate::generic::*;
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkSysCtrlSrc(u8);
impl ClkSysCtrlSrc {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> ClkSysCtrlSrc {
        ClkSysCtrlSrc(val)
    }
    pub const CLK_REF: Self = Self(0);
    pub const CLKSRC_CLK_SYS_AUX: Self = Self(0x01);
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkUsbCtrlAuxsrc(u8);
impl ClkUsbCtrlAuxsrc {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> ClkUsbCtrlAuxsrc {
        ClkUsbCtrlAuxsrc(val)
    }
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
pub struct ClkGpout2CtrlAuxsrc(u8);
impl ClkGpout2CtrlAuxsrc {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> ClkGpout2CtrlAuxsrc {
        ClkGpout2CtrlAuxsrc(val)
    }
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
pub struct ClkRefCtrlAuxsrc(u8);
impl ClkRefCtrlAuxsrc {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> ClkRefCtrlAuxsrc {
        ClkRefCtrlAuxsrc(val)
    }
    pub const CLKSRC_PLL_USB: Self = Self(0);
    pub const CLKSRC_GPIN0: Self = Self(0x01);
    pub const CLKSRC_GPIN1: Self = Self(0x02);
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkSysCtrlAuxsrc(u8);
impl ClkSysCtrlAuxsrc {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> ClkSysCtrlAuxsrc {
        ClkSysCtrlAuxsrc(val)
    }
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
pub struct ClkGpout1CtrlAuxsrc(u8);
impl ClkGpout1CtrlAuxsrc {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> ClkGpout1CtrlAuxsrc {
        ClkGpout1CtrlAuxsrc(val)
    }
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
pub struct ClkPeriCtrlAuxsrc(u8);
impl ClkPeriCtrlAuxsrc {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> ClkPeriCtrlAuxsrc {
        ClkPeriCtrlAuxsrc(val)
    }
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
pub struct ClkRefCtrlSrc(u8);
impl ClkRefCtrlSrc {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> ClkRefCtrlSrc {
        ClkRefCtrlSrc(val)
    }
    pub const ROSC_CLKSRC_PH: Self = Self(0);
    pub const CLKSRC_CLK_REF_AUX: Self = Self(0x01);
    pub const XOSC_CLKSRC: Self = Self(0x02);
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkAdcCtrlAuxsrc(u8);
impl ClkAdcCtrlAuxsrc {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> ClkAdcCtrlAuxsrc {
        ClkAdcCtrlAuxsrc(val)
    }
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
pub struct ClkGpout0CtrlAuxsrc(u8);
impl ClkGpout0CtrlAuxsrc {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> ClkGpout0CtrlAuxsrc {
        ClkGpout0CtrlAuxsrc(val)
    }
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
pub struct ClkRtcCtrlAuxsrc(u8);
impl ClkRtcCtrlAuxsrc {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> ClkRtcCtrlAuxsrc {
        ClkRtcCtrlAuxsrc(val)
    }
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
pub struct ClkGpout3CtrlAuxsrc(u8);
impl ClkGpout3CtrlAuxsrc {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> ClkGpout3CtrlAuxsrc {
        ClkGpout3CtrlAuxsrc(val)
    }
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
#[doc = "Clock sent to frequency counter, set to 0 when not required Writing to this register initiates the frequency count"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Fc0SrcFc0Src(u8);
impl Fc0SrcFc0Src {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Fc0SrcFc0Src {
        Fc0SrcFc0Src(val)
    }
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
