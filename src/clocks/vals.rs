#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ClkGpout0ctrlAuxsrc(pub u8);
impl ClkGpout0ctrlAuxsrc {
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ClkRefCtrlAuxsrc(pub u8);
impl ClkRefCtrlAuxsrc {
    pub const CLKSRC_PLL_USB: Self = Self(0);
    pub const CLKSRC_GPIN0: Self = Self(0x01);
    pub const CLKSRC_GPIN1: Self = Self(0x02);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ClkRtcCtrlAuxsrc(pub u8);
impl ClkRtcCtrlAuxsrc {
    pub const CLKSRC_PLL_USB: Self = Self(0);
    pub const CLKSRC_PLL_SYS: Self = Self(0x01);
    pub const ROSC_CLKSRC_PH: Self = Self(0x02);
    pub const XOSC_CLKSRC: Self = Self(0x03);
    pub const CLKSRC_GPIN0: Self = Self(0x04);
    pub const CLKSRC_GPIN1: Self = Self(0x05);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ClkRefCtrlSrc(pub u8);
impl ClkRefCtrlSrc {
    pub const ROSC_CLKSRC_PH: Self = Self(0);
    pub const CLKSRC_CLK_REF_AUX: Self = Self(0x01);
    pub const XOSC_CLKSRC: Self = Self(0x02);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ClkAdcCtrlAuxsrc(pub u8);
impl ClkAdcCtrlAuxsrc {
    pub const CLKSRC_PLL_USB: Self = Self(0);
    pub const CLKSRC_PLL_SYS: Self = Self(0x01);
    pub const ROSC_CLKSRC_PH: Self = Self(0x02);
    pub const XOSC_CLKSRC: Self = Self(0x03);
    pub const CLKSRC_GPIN0: Self = Self(0x04);
    pub const CLKSRC_GPIN1: Self = Self(0x05);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ClkGpout1ctrlAuxsrc(pub u8);
impl ClkGpout1ctrlAuxsrc {
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ClkUsbCtrlAuxsrc(pub u8);
impl ClkUsbCtrlAuxsrc {
    pub const CLKSRC_PLL_USB: Self = Self(0);
    pub const CLKSRC_PLL_SYS: Self = Self(0x01);
    pub const ROSC_CLKSRC_PH: Self = Self(0x02);
    pub const XOSC_CLKSRC: Self = Self(0x03);
    pub const CLKSRC_GPIN0: Self = Self(0x04);
    pub const CLKSRC_GPIN1: Self = Self(0x05);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ClkGpout3ctrlAuxsrc(pub u8);
impl ClkGpout3ctrlAuxsrc {
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ClkGpout2ctrlAuxsrc(pub u8);
impl ClkGpout2ctrlAuxsrc {
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ClkSysCtrlAuxsrc(pub u8);
impl ClkSysCtrlAuxsrc {
    pub const CLKSRC_PLL_SYS: Self = Self(0);
    pub const CLKSRC_PLL_USB: Self = Self(0x01);
    pub const ROSC_CLKSRC: Self = Self(0x02);
    pub const XOSC_CLKSRC: Self = Self(0x03);
    pub const CLKSRC_GPIN0: Self = Self(0x04);
    pub const CLKSRC_GPIN1: Self = Self(0x05);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ClkSysCtrlSrc(pub u8);
impl ClkSysCtrlSrc {
    pub const CLK_REF: Self = Self(0);
    pub const CLKSRC_CLK_SYS_AUX: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Fc0src(pub u8);
impl Fc0src {
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
