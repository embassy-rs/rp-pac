#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cs0size {
    NONE = 0x0,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cs1size {
    NONE = 0x0,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Range {
    _1_15MHZ = 0x0,
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
    pub const INDEX_USB_DEVICE_VID_VALUE: Self = Self(0x0);
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
impl core::fmt::Debug for UsbWhiteLabelAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("INDEX_USB_DEVICE_VID_VALUE"),
            0x01 => f.write_str("INDEX_USB_DEVICE_PID_VALUE"),
            0x02 => f.write_str("INDEX_USB_DEVICE_BCD_DEVICE_VALUE"),
            0x03 => f.write_str("INDEX_USB_DEVICE_LANG_ID_VALUE"),
            0x04 => f.write_str("INDEX_USB_DEVICE_MANUFACTURER_STRDEF"),
            0x05 => f.write_str("INDEX_USB_DEVICE_PRODUCT_STRDEF"),
            0x06 => f.write_str("INDEX_USB_DEVICE_SERIAL_NUMBER_STRDEF"),
            0x07 => f.write_str("INDEX_USB_CONFIG_ATTRIBUTES_MAX_POWER_VALUES"),
            0x08 => f.write_str("INDEX_VOLUME_LABEL_STRDEF"),
            0x09 => f.write_str("INDEX_SCSI_INQUIRY_VENDOR_STRDEF"),
            0x0a => f.write_str("INDEX_SCSI_INQUIRY_PRODUCT_STRDEF"),
            0x0b => f.write_str("INDEX_SCSI_INQUIRY_VERSION_STRDEF"),
            0x0c => f.write_str("INDEX_INDEX_HTM_REDIRECT_URL_STRDEF"),
            0x0d => f.write_str("INDEX_INDEX_HTM_REDIRECT_NAME_STRDEF"),
            0x0e => f.write_str("INDEX_INFO_UF2_TXT_MODEL_STRDEF"),
            0x0f => f.write_str("INDEX_INFO_UF2_TXT_BOARD_ID_STRDEF"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UsbWhiteLabelAddr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "INDEX_USB_DEVICE_VID_VALUE"),
            0x01 => defmt::write!(f, "INDEX_USB_DEVICE_PID_VALUE"),
            0x02 => defmt::write!(f, "INDEX_USB_DEVICE_BCD_DEVICE_VALUE"),
            0x03 => defmt::write!(f, "INDEX_USB_DEVICE_LANG_ID_VALUE"),
            0x04 => defmt::write!(f, "INDEX_USB_DEVICE_MANUFACTURER_STRDEF"),
            0x05 => defmt::write!(f, "INDEX_USB_DEVICE_PRODUCT_STRDEF"),
            0x06 => defmt::write!(f, "INDEX_USB_DEVICE_SERIAL_NUMBER_STRDEF"),
            0x07 => defmt::write!(f, "INDEX_USB_CONFIG_ATTRIBUTES_MAX_POWER_VALUES"),
            0x08 => defmt::write!(f, "INDEX_VOLUME_LABEL_STRDEF"),
            0x09 => defmt::write!(f, "INDEX_SCSI_INQUIRY_VENDOR_STRDEF"),
            0x0a => defmt::write!(f, "INDEX_SCSI_INQUIRY_PRODUCT_STRDEF"),
            0x0b => defmt::write!(f, "INDEX_SCSI_INQUIRY_VERSION_STRDEF"),
            0x0c => defmt::write!(f, "INDEX_INDEX_HTM_REDIRECT_URL_STRDEF"),
            0x0d => defmt::write!(f, "INDEX_INDEX_HTM_REDIRECT_NAME_STRDEF"),
            0x0e => defmt::write!(f, "INDEX_INFO_UF2_TXT_MODEL_STRDEF"),
            0x0f => defmt::write!(f, "INDEX_INFO_UF2_TXT_BOARD_ID_STRDEF"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
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
