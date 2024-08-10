#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cs0size {
    NONE = 0,
    _8K = 0x01,
    _16K = 0x02,
    _32K = 0x03,
    _64K = 0x04,
    _128K = 0x05,
    _256K = 0x06,
    _512K = 0x07,
    _1M = 0x08,
    _2M = 0x09,
    _4M = 0x0a,
    _8M = 0x0b,
    _16M = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Cs0size {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cs0size {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cs0size {
    #[inline(always)]
    fn from(val: u8) -> Cs0size {
        Cs0size::from_bits(val)
    }
}
impl From<Cs0size> for u8 {
    #[inline(always)]
    fn from(val: Cs0size) -> u8 {
        Cs0size::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cs1size {
    NONE = 0,
    _8K = 0x01,
    _16K = 0x02,
    _32K = 0x03,
    _64K = 0x04,
    _128K = 0x05,
    _256K = 0x06,
    _512K = 0x07,
    _1M = 0x08,
    _2M = 0x09,
    _4M = 0x0a,
    _8M = 0x0b,
    _16M = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Cs1size {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cs1size {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cs1size {
    #[inline(always)]
    fn from(val: u8) -> Cs1size {
        Cs1size::from_bits(val)
    }
}
impl From<Cs1size> for u8 {
    #[inline(always)]
    fn from(val: Cs1size) -> u8 {
        Cs1size::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Range {
    _1_15MHZ = 0,
    _10_30MHZ = 0x01,
    _25_60MHZ = 0x02,
    _40_100MHZ = 0x03,
}
impl Range {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Range {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Range {
    #[inline(always)]
    fn from(val: u8) -> Range {
        Range::from_bits(val)
    }
}
impl From<Range> for u8 {
    #[inline(always)]
    fn from(val: Range) -> u8 {
        Range::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UsbWhiteLabelAddr(pub u16);
impl UsbWhiteLabelAddr {
    pub const INDEX_USB_DEVICE_VID_VALUE: Self = Self(0);
    pub const INDEX_USB_DEVICE_PID_VALUE: Self = Self(0x01);
    pub const INDEX_USB_DEVICE_BCD_DEVICE_VALUE: Self = Self(0x02);
    pub const INDEX_USB_DEVICE_LANG_ID_VALUE: Self = Self(0x03);
    pub const INDEX_USB_DEVICE_MANUFACTURER_STRDEF: Self = Self(0x04);
    pub const INDEX_USB_DEVICE_PRODUCT_STRDEF: Self = Self(0x05);
    pub const INDEX_USB_DEVICE_SERIAL_NUMBER_STRDEF: Self = Self(0x06);
    pub const INDEX_USB_CONFIG_ATTRIBUTES_MAX_POWER_VALUES: Self = Self(0x07);
    pub const INDEX_VOLUME_LABEL_STRDEF: Self = Self(0x08);
    pub const INDEX_SCSI_INQUIRY_VENDOR_STRDEF: Self = Self(0x09);
    pub const INDEX_SCSI_INQUIRY_PRODUCT_STRDEF: Self = Self(0x0a);
    pub const INDEX_SCSI_INQUIRY_VERSION_STRDEF: Self = Self(0x0b);
    pub const INDEX_INDEX_HTM_REDIRECT_URL_STRDEF: Self = Self(0x0c);
    pub const INDEX_INDEX_HTM_REDIRECT_NAME_STRDEF: Self = Self(0x0d);
    pub const INDEX_INFO_UF2_TXT_MODEL_STRDEF: Self = Self(0x0e);
    pub const INDEX_INFO_UF2_TXT_BOARD_ID_STRDEF: Self = Self(0x0f);
}
impl UsbWhiteLabelAddr {
    pub const fn from_bits(val: u16) -> UsbWhiteLabelAddr {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl From<u16> for UsbWhiteLabelAddr {
    #[inline(always)]
    fn from(val: u16) -> UsbWhiteLabelAddr {
        UsbWhiteLabelAddr::from_bits(val)
    }
}
impl From<UsbWhiteLabelAddr> for u16 {
    #[inline(always)]
    fn from(val: UsbWhiteLabelAddr) -> u16 {
        UsbWhiteLabelAddr::to_bits(val)
    }
}
