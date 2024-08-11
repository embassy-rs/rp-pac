#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cs1size(pub u16);
impl Cs1size {
    pub const NONE: Self = Self(0x0);
    pub const _8K: Self = Self(0x01);
    pub const _16K: Self = Self(0x02);
    pub const _32K: Self = Self(0x03);
    pub const _64K: Self = Self(0x04);
    pub const _128K: Self = Self(0x05);
    pub const _256K: Self = Self(0x06);
    pub const _512K: Self = Self(0x07);
    pub const _1M: Self = Self(0x08);
    pub const _2M: Self = Self(0x09);
    pub const _4M: Self = Self(0x0a);
    pub const _8M: Self = Self(0x0b);
    pub const _16M: Self = Self(0x0c);
}
impl Cs1size {
    pub const fn from_bits(val: u16) -> Cs1size {
        Self(val & 0x0fff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl From<u16> for Cs1size {
    #[inline(always)]
    fn from(val: u16) -> Cs1size {
        Cs1size::from_bits(val)
    }
}
impl From<Cs1size> for u16 {
    #[inline(always)]
    fn from(val: Cs1size) -> u16 {
        Cs1size::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PageLock {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0x0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl PageLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PageLock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PageLock {
    #[inline(always)]
    fn from(val: u8) -> PageLock {
        PageLock::from_bits(val)
    }
}
impl From<PageLock> for u8 {
    #[inline(always)]
    fn from(val: PageLock) -> u8 {
        PageLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PageLockNoKeyState {
    READ_ONLY = 0x0,
    INACCESSIBLE = 0x01,
}
impl PageLockNoKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PageLockNoKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PageLockNoKeyState {
    #[inline(always)]
    fn from(val: u8) -> PageLockNoKeyState {
        PageLockNoKeyState::from_bits(val)
    }
}
impl From<PageLockNoKeyState> for u8 {
    #[inline(always)]
    fn from(val: PageLockNoKeyState) -> u8 {
        PageLockNoKeyState::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Range(pub u16);
impl Range {
    pub const _1_15MHZ: Self = Self(0x0);
    pub const _10_30MHZ: Self = Self(0x01);
    pub const _25_60MHZ: Self = Self(0x02);
    pub const _40_100MHZ: Self = Self(0x03);
}
impl Range {
    pub const fn from_bits(val: u16) -> Range {
        Self(val & 0x03ff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl From<u16> for Range {
    #[inline(always)]
    fn from(val: u16) -> Range {
        Range::from_bits(val)
    }
}
impl From<Range> for u16 {
    #[inline(always)]
    fn from(val: Range) -> u16 {
        Range::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UsbWhiteLabelAddr(pub u32);
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
    pub const fn from_bits(val: u32) -> UsbWhiteLabelAddr {
        Self(val & 0x00ff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for UsbWhiteLabelAddr {
    #[inline(always)]
    fn from(val: u32) -> UsbWhiteLabelAddr {
        UsbWhiteLabelAddr::from_bits(val)
    }
}
impl From<UsbWhiteLabelAddr> for u32 {
    #[inline(always)]
    fn from(val: UsbWhiteLabelAddr) -> u32 {
        UsbWhiteLabelAddr::to_bits(val)
    }
}
