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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cs1size(pub u16);
impl Cs1size {
    pub const NONE: Self = Self(0);
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
pub enum Page0lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page0lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page0lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page0lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page0lock0noKeyState {
        Page0lock0noKeyState::from_bits(val)
    }
}
impl From<Page0lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page0lock0noKeyState) -> u8 {
        Page0lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page0lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page0lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page0lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page0lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page0lock1lockBl {
        Page0lock1lockBl::from_bits(val)
    }
}
impl From<Page0lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page0lock1lockBl) -> u8 {
        Page0lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page0lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page0lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page0lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page0lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page0lock1lockNs {
        Page0lock1lockNs::from_bits(val)
    }
}
impl From<Page0lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page0lock1lockNs) -> u8 {
        Page0lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page0lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page0lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page0lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page0lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page0lock1lockS {
        Page0lock1lockS::from_bits(val)
    }
}
impl From<Page0lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page0lock1lockS) -> u8 {
        Page0lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page10lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page10lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page10lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page10lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page10lock0noKeyState {
        Page10lock0noKeyState::from_bits(val)
    }
}
impl From<Page10lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page10lock0noKeyState) -> u8 {
        Page10lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page10lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page10lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page10lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page10lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page10lock1lockBl {
        Page10lock1lockBl::from_bits(val)
    }
}
impl From<Page10lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page10lock1lockBl) -> u8 {
        Page10lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page10lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page10lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page10lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page10lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page10lock1lockNs {
        Page10lock1lockNs::from_bits(val)
    }
}
impl From<Page10lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page10lock1lockNs) -> u8 {
        Page10lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page10lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page10lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page10lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page10lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page10lock1lockS {
        Page10lock1lockS::from_bits(val)
    }
}
impl From<Page10lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page10lock1lockS) -> u8 {
        Page10lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page11lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page11lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page11lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page11lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page11lock0noKeyState {
        Page11lock0noKeyState::from_bits(val)
    }
}
impl From<Page11lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page11lock0noKeyState) -> u8 {
        Page11lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page11lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page11lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page11lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page11lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page11lock1lockBl {
        Page11lock1lockBl::from_bits(val)
    }
}
impl From<Page11lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page11lock1lockBl) -> u8 {
        Page11lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page11lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page11lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page11lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page11lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page11lock1lockNs {
        Page11lock1lockNs::from_bits(val)
    }
}
impl From<Page11lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page11lock1lockNs) -> u8 {
        Page11lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page11lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page11lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page11lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page11lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page11lock1lockS {
        Page11lock1lockS::from_bits(val)
    }
}
impl From<Page11lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page11lock1lockS) -> u8 {
        Page11lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page12lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page12lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page12lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page12lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page12lock0noKeyState {
        Page12lock0noKeyState::from_bits(val)
    }
}
impl From<Page12lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page12lock0noKeyState) -> u8 {
        Page12lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page12lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page12lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page12lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page12lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page12lock1lockBl {
        Page12lock1lockBl::from_bits(val)
    }
}
impl From<Page12lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page12lock1lockBl) -> u8 {
        Page12lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page12lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page12lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page12lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page12lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page12lock1lockNs {
        Page12lock1lockNs::from_bits(val)
    }
}
impl From<Page12lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page12lock1lockNs) -> u8 {
        Page12lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page12lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page12lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page12lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page12lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page12lock1lockS {
        Page12lock1lockS::from_bits(val)
    }
}
impl From<Page12lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page12lock1lockS) -> u8 {
        Page12lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page13lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page13lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page13lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page13lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page13lock0noKeyState {
        Page13lock0noKeyState::from_bits(val)
    }
}
impl From<Page13lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page13lock0noKeyState) -> u8 {
        Page13lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page13lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page13lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page13lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page13lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page13lock1lockBl {
        Page13lock1lockBl::from_bits(val)
    }
}
impl From<Page13lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page13lock1lockBl) -> u8 {
        Page13lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page13lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page13lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page13lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page13lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page13lock1lockNs {
        Page13lock1lockNs::from_bits(val)
    }
}
impl From<Page13lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page13lock1lockNs) -> u8 {
        Page13lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page13lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page13lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page13lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page13lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page13lock1lockS {
        Page13lock1lockS::from_bits(val)
    }
}
impl From<Page13lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page13lock1lockS) -> u8 {
        Page13lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page14lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page14lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page14lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page14lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page14lock0noKeyState {
        Page14lock0noKeyState::from_bits(val)
    }
}
impl From<Page14lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page14lock0noKeyState) -> u8 {
        Page14lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page14lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page14lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page14lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page14lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page14lock1lockBl {
        Page14lock1lockBl::from_bits(val)
    }
}
impl From<Page14lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page14lock1lockBl) -> u8 {
        Page14lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page14lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page14lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page14lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page14lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page14lock1lockNs {
        Page14lock1lockNs::from_bits(val)
    }
}
impl From<Page14lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page14lock1lockNs) -> u8 {
        Page14lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page14lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page14lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page14lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page14lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page14lock1lockS {
        Page14lock1lockS::from_bits(val)
    }
}
impl From<Page14lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page14lock1lockS) -> u8 {
        Page14lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page15lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page15lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page15lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page15lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page15lock0noKeyState {
        Page15lock0noKeyState::from_bits(val)
    }
}
impl From<Page15lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page15lock0noKeyState) -> u8 {
        Page15lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page15lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page15lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page15lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page15lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page15lock1lockBl {
        Page15lock1lockBl::from_bits(val)
    }
}
impl From<Page15lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page15lock1lockBl) -> u8 {
        Page15lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page15lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page15lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page15lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page15lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page15lock1lockNs {
        Page15lock1lockNs::from_bits(val)
    }
}
impl From<Page15lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page15lock1lockNs) -> u8 {
        Page15lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page15lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page15lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page15lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page15lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page15lock1lockS {
        Page15lock1lockS::from_bits(val)
    }
}
impl From<Page15lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page15lock1lockS) -> u8 {
        Page15lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page16lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page16lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page16lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page16lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page16lock0noKeyState {
        Page16lock0noKeyState::from_bits(val)
    }
}
impl From<Page16lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page16lock0noKeyState) -> u8 {
        Page16lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page16lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page16lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page16lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page16lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page16lock1lockBl {
        Page16lock1lockBl::from_bits(val)
    }
}
impl From<Page16lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page16lock1lockBl) -> u8 {
        Page16lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page16lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page16lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page16lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page16lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page16lock1lockNs {
        Page16lock1lockNs::from_bits(val)
    }
}
impl From<Page16lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page16lock1lockNs) -> u8 {
        Page16lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page16lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page16lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page16lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page16lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page16lock1lockS {
        Page16lock1lockS::from_bits(val)
    }
}
impl From<Page16lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page16lock1lockS) -> u8 {
        Page16lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page17lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page17lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page17lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page17lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page17lock0noKeyState {
        Page17lock0noKeyState::from_bits(val)
    }
}
impl From<Page17lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page17lock0noKeyState) -> u8 {
        Page17lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page17lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page17lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page17lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page17lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page17lock1lockBl {
        Page17lock1lockBl::from_bits(val)
    }
}
impl From<Page17lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page17lock1lockBl) -> u8 {
        Page17lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page17lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page17lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page17lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page17lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page17lock1lockNs {
        Page17lock1lockNs::from_bits(val)
    }
}
impl From<Page17lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page17lock1lockNs) -> u8 {
        Page17lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page17lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page17lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page17lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page17lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page17lock1lockS {
        Page17lock1lockS::from_bits(val)
    }
}
impl From<Page17lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page17lock1lockS) -> u8 {
        Page17lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page18lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page18lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page18lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page18lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page18lock0noKeyState {
        Page18lock0noKeyState::from_bits(val)
    }
}
impl From<Page18lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page18lock0noKeyState) -> u8 {
        Page18lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page18lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page18lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page18lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page18lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page18lock1lockBl {
        Page18lock1lockBl::from_bits(val)
    }
}
impl From<Page18lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page18lock1lockBl) -> u8 {
        Page18lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page18lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page18lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page18lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page18lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page18lock1lockNs {
        Page18lock1lockNs::from_bits(val)
    }
}
impl From<Page18lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page18lock1lockNs) -> u8 {
        Page18lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page18lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page18lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page18lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page18lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page18lock1lockS {
        Page18lock1lockS::from_bits(val)
    }
}
impl From<Page18lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page18lock1lockS) -> u8 {
        Page18lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page19lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page19lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page19lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page19lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page19lock0noKeyState {
        Page19lock0noKeyState::from_bits(val)
    }
}
impl From<Page19lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page19lock0noKeyState) -> u8 {
        Page19lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page19lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page19lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page19lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page19lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page19lock1lockBl {
        Page19lock1lockBl::from_bits(val)
    }
}
impl From<Page19lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page19lock1lockBl) -> u8 {
        Page19lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page19lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page19lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page19lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page19lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page19lock1lockNs {
        Page19lock1lockNs::from_bits(val)
    }
}
impl From<Page19lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page19lock1lockNs) -> u8 {
        Page19lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page19lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page19lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page19lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page19lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page19lock1lockS {
        Page19lock1lockS::from_bits(val)
    }
}
impl From<Page19lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page19lock1lockS) -> u8 {
        Page19lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page1lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page1lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page1lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page1lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page1lock0noKeyState {
        Page1lock0noKeyState::from_bits(val)
    }
}
impl From<Page1lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page1lock0noKeyState) -> u8 {
        Page1lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page1lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page1lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page1lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page1lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page1lock1lockBl {
        Page1lock1lockBl::from_bits(val)
    }
}
impl From<Page1lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page1lock1lockBl) -> u8 {
        Page1lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page1lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page1lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page1lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page1lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page1lock1lockNs {
        Page1lock1lockNs::from_bits(val)
    }
}
impl From<Page1lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page1lock1lockNs) -> u8 {
        Page1lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page1lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page1lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page1lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page1lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page1lock1lockS {
        Page1lock1lockS::from_bits(val)
    }
}
impl From<Page1lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page1lock1lockS) -> u8 {
        Page1lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page20lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page20lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page20lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page20lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page20lock0noKeyState {
        Page20lock0noKeyState::from_bits(val)
    }
}
impl From<Page20lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page20lock0noKeyState) -> u8 {
        Page20lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page20lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page20lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page20lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page20lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page20lock1lockBl {
        Page20lock1lockBl::from_bits(val)
    }
}
impl From<Page20lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page20lock1lockBl) -> u8 {
        Page20lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page20lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page20lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page20lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page20lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page20lock1lockNs {
        Page20lock1lockNs::from_bits(val)
    }
}
impl From<Page20lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page20lock1lockNs) -> u8 {
        Page20lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page20lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page20lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page20lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page20lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page20lock1lockS {
        Page20lock1lockS::from_bits(val)
    }
}
impl From<Page20lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page20lock1lockS) -> u8 {
        Page20lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page21lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page21lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page21lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page21lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page21lock0noKeyState {
        Page21lock0noKeyState::from_bits(val)
    }
}
impl From<Page21lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page21lock0noKeyState) -> u8 {
        Page21lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page21lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page21lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page21lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page21lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page21lock1lockBl {
        Page21lock1lockBl::from_bits(val)
    }
}
impl From<Page21lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page21lock1lockBl) -> u8 {
        Page21lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page21lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page21lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page21lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page21lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page21lock1lockNs {
        Page21lock1lockNs::from_bits(val)
    }
}
impl From<Page21lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page21lock1lockNs) -> u8 {
        Page21lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page21lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page21lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page21lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page21lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page21lock1lockS {
        Page21lock1lockS::from_bits(val)
    }
}
impl From<Page21lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page21lock1lockS) -> u8 {
        Page21lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page22lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page22lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page22lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page22lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page22lock0noKeyState {
        Page22lock0noKeyState::from_bits(val)
    }
}
impl From<Page22lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page22lock0noKeyState) -> u8 {
        Page22lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page22lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page22lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page22lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page22lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page22lock1lockBl {
        Page22lock1lockBl::from_bits(val)
    }
}
impl From<Page22lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page22lock1lockBl) -> u8 {
        Page22lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page22lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page22lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page22lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page22lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page22lock1lockNs {
        Page22lock1lockNs::from_bits(val)
    }
}
impl From<Page22lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page22lock1lockNs) -> u8 {
        Page22lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page22lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page22lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page22lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page22lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page22lock1lockS {
        Page22lock1lockS::from_bits(val)
    }
}
impl From<Page22lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page22lock1lockS) -> u8 {
        Page22lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page23lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page23lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page23lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page23lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page23lock0noKeyState {
        Page23lock0noKeyState::from_bits(val)
    }
}
impl From<Page23lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page23lock0noKeyState) -> u8 {
        Page23lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page23lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page23lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page23lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page23lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page23lock1lockBl {
        Page23lock1lockBl::from_bits(val)
    }
}
impl From<Page23lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page23lock1lockBl) -> u8 {
        Page23lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page23lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page23lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page23lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page23lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page23lock1lockNs {
        Page23lock1lockNs::from_bits(val)
    }
}
impl From<Page23lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page23lock1lockNs) -> u8 {
        Page23lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page23lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page23lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page23lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page23lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page23lock1lockS {
        Page23lock1lockS::from_bits(val)
    }
}
impl From<Page23lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page23lock1lockS) -> u8 {
        Page23lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page24lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page24lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page24lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page24lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page24lock0noKeyState {
        Page24lock0noKeyState::from_bits(val)
    }
}
impl From<Page24lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page24lock0noKeyState) -> u8 {
        Page24lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page24lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page24lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page24lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page24lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page24lock1lockBl {
        Page24lock1lockBl::from_bits(val)
    }
}
impl From<Page24lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page24lock1lockBl) -> u8 {
        Page24lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page24lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page24lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page24lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page24lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page24lock1lockNs {
        Page24lock1lockNs::from_bits(val)
    }
}
impl From<Page24lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page24lock1lockNs) -> u8 {
        Page24lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page24lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page24lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page24lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page24lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page24lock1lockS {
        Page24lock1lockS::from_bits(val)
    }
}
impl From<Page24lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page24lock1lockS) -> u8 {
        Page24lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page25lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page25lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page25lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page25lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page25lock0noKeyState {
        Page25lock0noKeyState::from_bits(val)
    }
}
impl From<Page25lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page25lock0noKeyState) -> u8 {
        Page25lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page25lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page25lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page25lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page25lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page25lock1lockBl {
        Page25lock1lockBl::from_bits(val)
    }
}
impl From<Page25lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page25lock1lockBl) -> u8 {
        Page25lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page25lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page25lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page25lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page25lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page25lock1lockNs {
        Page25lock1lockNs::from_bits(val)
    }
}
impl From<Page25lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page25lock1lockNs) -> u8 {
        Page25lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page25lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page25lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page25lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page25lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page25lock1lockS {
        Page25lock1lockS::from_bits(val)
    }
}
impl From<Page25lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page25lock1lockS) -> u8 {
        Page25lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page26lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page26lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page26lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page26lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page26lock0noKeyState {
        Page26lock0noKeyState::from_bits(val)
    }
}
impl From<Page26lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page26lock0noKeyState) -> u8 {
        Page26lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page26lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page26lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page26lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page26lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page26lock1lockBl {
        Page26lock1lockBl::from_bits(val)
    }
}
impl From<Page26lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page26lock1lockBl) -> u8 {
        Page26lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page26lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page26lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page26lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page26lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page26lock1lockNs {
        Page26lock1lockNs::from_bits(val)
    }
}
impl From<Page26lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page26lock1lockNs) -> u8 {
        Page26lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page26lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page26lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page26lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page26lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page26lock1lockS {
        Page26lock1lockS::from_bits(val)
    }
}
impl From<Page26lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page26lock1lockS) -> u8 {
        Page26lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page27lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page27lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page27lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page27lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page27lock0noKeyState {
        Page27lock0noKeyState::from_bits(val)
    }
}
impl From<Page27lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page27lock0noKeyState) -> u8 {
        Page27lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page27lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page27lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page27lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page27lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page27lock1lockBl {
        Page27lock1lockBl::from_bits(val)
    }
}
impl From<Page27lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page27lock1lockBl) -> u8 {
        Page27lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page27lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page27lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page27lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page27lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page27lock1lockNs {
        Page27lock1lockNs::from_bits(val)
    }
}
impl From<Page27lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page27lock1lockNs) -> u8 {
        Page27lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page27lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page27lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page27lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page27lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page27lock1lockS {
        Page27lock1lockS::from_bits(val)
    }
}
impl From<Page27lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page27lock1lockS) -> u8 {
        Page27lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page28lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page28lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page28lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page28lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page28lock0noKeyState {
        Page28lock0noKeyState::from_bits(val)
    }
}
impl From<Page28lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page28lock0noKeyState) -> u8 {
        Page28lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page28lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page28lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page28lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page28lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page28lock1lockBl {
        Page28lock1lockBl::from_bits(val)
    }
}
impl From<Page28lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page28lock1lockBl) -> u8 {
        Page28lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page28lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page28lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page28lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page28lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page28lock1lockNs {
        Page28lock1lockNs::from_bits(val)
    }
}
impl From<Page28lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page28lock1lockNs) -> u8 {
        Page28lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page28lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page28lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page28lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page28lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page28lock1lockS {
        Page28lock1lockS::from_bits(val)
    }
}
impl From<Page28lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page28lock1lockS) -> u8 {
        Page28lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page29lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page29lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page29lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page29lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page29lock0noKeyState {
        Page29lock0noKeyState::from_bits(val)
    }
}
impl From<Page29lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page29lock0noKeyState) -> u8 {
        Page29lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page29lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page29lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page29lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page29lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page29lock1lockBl {
        Page29lock1lockBl::from_bits(val)
    }
}
impl From<Page29lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page29lock1lockBl) -> u8 {
        Page29lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page29lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page29lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page29lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page29lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page29lock1lockNs {
        Page29lock1lockNs::from_bits(val)
    }
}
impl From<Page29lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page29lock1lockNs) -> u8 {
        Page29lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page29lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page29lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page29lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page29lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page29lock1lockS {
        Page29lock1lockS::from_bits(val)
    }
}
impl From<Page29lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page29lock1lockS) -> u8 {
        Page29lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page2lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page2lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page2lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page2lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page2lock0noKeyState {
        Page2lock0noKeyState::from_bits(val)
    }
}
impl From<Page2lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page2lock0noKeyState) -> u8 {
        Page2lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page2lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page2lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page2lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page2lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page2lock1lockBl {
        Page2lock1lockBl::from_bits(val)
    }
}
impl From<Page2lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page2lock1lockBl) -> u8 {
        Page2lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page2lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page2lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page2lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page2lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page2lock1lockNs {
        Page2lock1lockNs::from_bits(val)
    }
}
impl From<Page2lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page2lock1lockNs) -> u8 {
        Page2lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page2lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page2lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page2lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page2lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page2lock1lockS {
        Page2lock1lockS::from_bits(val)
    }
}
impl From<Page2lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page2lock1lockS) -> u8 {
        Page2lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page30lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page30lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page30lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page30lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page30lock0noKeyState {
        Page30lock0noKeyState::from_bits(val)
    }
}
impl From<Page30lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page30lock0noKeyState) -> u8 {
        Page30lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page30lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page30lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page30lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page30lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page30lock1lockBl {
        Page30lock1lockBl::from_bits(val)
    }
}
impl From<Page30lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page30lock1lockBl) -> u8 {
        Page30lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page30lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page30lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page30lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page30lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page30lock1lockNs {
        Page30lock1lockNs::from_bits(val)
    }
}
impl From<Page30lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page30lock1lockNs) -> u8 {
        Page30lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page30lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page30lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page30lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page30lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page30lock1lockS {
        Page30lock1lockS::from_bits(val)
    }
}
impl From<Page30lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page30lock1lockS) -> u8 {
        Page30lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page31lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page31lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page31lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page31lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page31lock0noKeyState {
        Page31lock0noKeyState::from_bits(val)
    }
}
impl From<Page31lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page31lock0noKeyState) -> u8 {
        Page31lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page31lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page31lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page31lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page31lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page31lock1lockBl {
        Page31lock1lockBl::from_bits(val)
    }
}
impl From<Page31lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page31lock1lockBl) -> u8 {
        Page31lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page31lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page31lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page31lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page31lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page31lock1lockNs {
        Page31lock1lockNs::from_bits(val)
    }
}
impl From<Page31lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page31lock1lockNs) -> u8 {
        Page31lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page31lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page31lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page31lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page31lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page31lock1lockS {
        Page31lock1lockS::from_bits(val)
    }
}
impl From<Page31lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page31lock1lockS) -> u8 {
        Page31lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page32lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page32lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page32lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page32lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page32lock0noKeyState {
        Page32lock0noKeyState::from_bits(val)
    }
}
impl From<Page32lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page32lock0noKeyState) -> u8 {
        Page32lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page32lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page32lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page32lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page32lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page32lock1lockBl {
        Page32lock1lockBl::from_bits(val)
    }
}
impl From<Page32lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page32lock1lockBl) -> u8 {
        Page32lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page32lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page32lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page32lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page32lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page32lock1lockNs {
        Page32lock1lockNs::from_bits(val)
    }
}
impl From<Page32lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page32lock1lockNs) -> u8 {
        Page32lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page32lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page32lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page32lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page32lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page32lock1lockS {
        Page32lock1lockS::from_bits(val)
    }
}
impl From<Page32lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page32lock1lockS) -> u8 {
        Page32lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page33lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page33lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page33lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page33lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page33lock0noKeyState {
        Page33lock0noKeyState::from_bits(val)
    }
}
impl From<Page33lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page33lock0noKeyState) -> u8 {
        Page33lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page33lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page33lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page33lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page33lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page33lock1lockBl {
        Page33lock1lockBl::from_bits(val)
    }
}
impl From<Page33lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page33lock1lockBl) -> u8 {
        Page33lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page33lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page33lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page33lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page33lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page33lock1lockNs {
        Page33lock1lockNs::from_bits(val)
    }
}
impl From<Page33lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page33lock1lockNs) -> u8 {
        Page33lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page33lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page33lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page33lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page33lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page33lock1lockS {
        Page33lock1lockS::from_bits(val)
    }
}
impl From<Page33lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page33lock1lockS) -> u8 {
        Page33lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page34lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page34lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page34lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page34lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page34lock0noKeyState {
        Page34lock0noKeyState::from_bits(val)
    }
}
impl From<Page34lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page34lock0noKeyState) -> u8 {
        Page34lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page34lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page34lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page34lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page34lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page34lock1lockBl {
        Page34lock1lockBl::from_bits(val)
    }
}
impl From<Page34lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page34lock1lockBl) -> u8 {
        Page34lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page34lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page34lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page34lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page34lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page34lock1lockNs {
        Page34lock1lockNs::from_bits(val)
    }
}
impl From<Page34lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page34lock1lockNs) -> u8 {
        Page34lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page34lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page34lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page34lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page34lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page34lock1lockS {
        Page34lock1lockS::from_bits(val)
    }
}
impl From<Page34lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page34lock1lockS) -> u8 {
        Page34lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page35lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page35lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page35lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page35lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page35lock0noKeyState {
        Page35lock0noKeyState::from_bits(val)
    }
}
impl From<Page35lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page35lock0noKeyState) -> u8 {
        Page35lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page35lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page35lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page35lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page35lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page35lock1lockBl {
        Page35lock1lockBl::from_bits(val)
    }
}
impl From<Page35lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page35lock1lockBl) -> u8 {
        Page35lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page35lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page35lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page35lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page35lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page35lock1lockNs {
        Page35lock1lockNs::from_bits(val)
    }
}
impl From<Page35lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page35lock1lockNs) -> u8 {
        Page35lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page35lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page35lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page35lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page35lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page35lock1lockS {
        Page35lock1lockS::from_bits(val)
    }
}
impl From<Page35lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page35lock1lockS) -> u8 {
        Page35lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page36lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page36lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page36lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page36lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page36lock0noKeyState {
        Page36lock0noKeyState::from_bits(val)
    }
}
impl From<Page36lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page36lock0noKeyState) -> u8 {
        Page36lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page36lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page36lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page36lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page36lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page36lock1lockBl {
        Page36lock1lockBl::from_bits(val)
    }
}
impl From<Page36lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page36lock1lockBl) -> u8 {
        Page36lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page36lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page36lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page36lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page36lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page36lock1lockNs {
        Page36lock1lockNs::from_bits(val)
    }
}
impl From<Page36lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page36lock1lockNs) -> u8 {
        Page36lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page36lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page36lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page36lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page36lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page36lock1lockS {
        Page36lock1lockS::from_bits(val)
    }
}
impl From<Page36lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page36lock1lockS) -> u8 {
        Page36lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page37lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page37lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page37lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page37lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page37lock0noKeyState {
        Page37lock0noKeyState::from_bits(val)
    }
}
impl From<Page37lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page37lock0noKeyState) -> u8 {
        Page37lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page37lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page37lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page37lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page37lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page37lock1lockBl {
        Page37lock1lockBl::from_bits(val)
    }
}
impl From<Page37lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page37lock1lockBl) -> u8 {
        Page37lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page37lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page37lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page37lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page37lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page37lock1lockNs {
        Page37lock1lockNs::from_bits(val)
    }
}
impl From<Page37lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page37lock1lockNs) -> u8 {
        Page37lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page37lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page37lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page37lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page37lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page37lock1lockS {
        Page37lock1lockS::from_bits(val)
    }
}
impl From<Page37lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page37lock1lockS) -> u8 {
        Page37lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page38lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page38lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page38lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page38lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page38lock0noKeyState {
        Page38lock0noKeyState::from_bits(val)
    }
}
impl From<Page38lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page38lock0noKeyState) -> u8 {
        Page38lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page38lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page38lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page38lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page38lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page38lock1lockBl {
        Page38lock1lockBl::from_bits(val)
    }
}
impl From<Page38lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page38lock1lockBl) -> u8 {
        Page38lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page38lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page38lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page38lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page38lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page38lock1lockNs {
        Page38lock1lockNs::from_bits(val)
    }
}
impl From<Page38lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page38lock1lockNs) -> u8 {
        Page38lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page38lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page38lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page38lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page38lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page38lock1lockS {
        Page38lock1lockS::from_bits(val)
    }
}
impl From<Page38lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page38lock1lockS) -> u8 {
        Page38lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page39lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page39lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page39lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page39lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page39lock0noKeyState {
        Page39lock0noKeyState::from_bits(val)
    }
}
impl From<Page39lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page39lock0noKeyState) -> u8 {
        Page39lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page39lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page39lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page39lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page39lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page39lock1lockBl {
        Page39lock1lockBl::from_bits(val)
    }
}
impl From<Page39lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page39lock1lockBl) -> u8 {
        Page39lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page39lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page39lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page39lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page39lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page39lock1lockNs {
        Page39lock1lockNs::from_bits(val)
    }
}
impl From<Page39lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page39lock1lockNs) -> u8 {
        Page39lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page39lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page39lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page39lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page39lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page39lock1lockS {
        Page39lock1lockS::from_bits(val)
    }
}
impl From<Page39lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page39lock1lockS) -> u8 {
        Page39lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page3lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page3lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page3lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page3lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page3lock0noKeyState {
        Page3lock0noKeyState::from_bits(val)
    }
}
impl From<Page3lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page3lock0noKeyState) -> u8 {
        Page3lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page3lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page3lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page3lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page3lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page3lock1lockBl {
        Page3lock1lockBl::from_bits(val)
    }
}
impl From<Page3lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page3lock1lockBl) -> u8 {
        Page3lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page3lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page3lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page3lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page3lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page3lock1lockNs {
        Page3lock1lockNs::from_bits(val)
    }
}
impl From<Page3lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page3lock1lockNs) -> u8 {
        Page3lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page3lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page3lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page3lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page3lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page3lock1lockS {
        Page3lock1lockS::from_bits(val)
    }
}
impl From<Page3lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page3lock1lockS) -> u8 {
        Page3lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page40lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page40lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page40lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page40lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page40lock0noKeyState {
        Page40lock0noKeyState::from_bits(val)
    }
}
impl From<Page40lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page40lock0noKeyState) -> u8 {
        Page40lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page40lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page40lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page40lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page40lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page40lock1lockBl {
        Page40lock1lockBl::from_bits(val)
    }
}
impl From<Page40lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page40lock1lockBl) -> u8 {
        Page40lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page40lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page40lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page40lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page40lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page40lock1lockNs {
        Page40lock1lockNs::from_bits(val)
    }
}
impl From<Page40lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page40lock1lockNs) -> u8 {
        Page40lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page40lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page40lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page40lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page40lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page40lock1lockS {
        Page40lock1lockS::from_bits(val)
    }
}
impl From<Page40lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page40lock1lockS) -> u8 {
        Page40lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page41lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page41lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page41lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page41lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page41lock0noKeyState {
        Page41lock0noKeyState::from_bits(val)
    }
}
impl From<Page41lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page41lock0noKeyState) -> u8 {
        Page41lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page41lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page41lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page41lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page41lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page41lock1lockBl {
        Page41lock1lockBl::from_bits(val)
    }
}
impl From<Page41lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page41lock1lockBl) -> u8 {
        Page41lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page41lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page41lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page41lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page41lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page41lock1lockNs {
        Page41lock1lockNs::from_bits(val)
    }
}
impl From<Page41lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page41lock1lockNs) -> u8 {
        Page41lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page41lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page41lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page41lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page41lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page41lock1lockS {
        Page41lock1lockS::from_bits(val)
    }
}
impl From<Page41lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page41lock1lockS) -> u8 {
        Page41lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page42lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page42lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page42lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page42lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page42lock0noKeyState {
        Page42lock0noKeyState::from_bits(val)
    }
}
impl From<Page42lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page42lock0noKeyState) -> u8 {
        Page42lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page42lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page42lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page42lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page42lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page42lock1lockBl {
        Page42lock1lockBl::from_bits(val)
    }
}
impl From<Page42lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page42lock1lockBl) -> u8 {
        Page42lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page42lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page42lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page42lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page42lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page42lock1lockNs {
        Page42lock1lockNs::from_bits(val)
    }
}
impl From<Page42lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page42lock1lockNs) -> u8 {
        Page42lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page42lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page42lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page42lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page42lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page42lock1lockS {
        Page42lock1lockS::from_bits(val)
    }
}
impl From<Page42lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page42lock1lockS) -> u8 {
        Page42lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page43lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page43lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page43lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page43lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page43lock0noKeyState {
        Page43lock0noKeyState::from_bits(val)
    }
}
impl From<Page43lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page43lock0noKeyState) -> u8 {
        Page43lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page43lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page43lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page43lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page43lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page43lock1lockBl {
        Page43lock1lockBl::from_bits(val)
    }
}
impl From<Page43lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page43lock1lockBl) -> u8 {
        Page43lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page43lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page43lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page43lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page43lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page43lock1lockNs {
        Page43lock1lockNs::from_bits(val)
    }
}
impl From<Page43lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page43lock1lockNs) -> u8 {
        Page43lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page43lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page43lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page43lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page43lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page43lock1lockS {
        Page43lock1lockS::from_bits(val)
    }
}
impl From<Page43lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page43lock1lockS) -> u8 {
        Page43lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page44lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page44lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page44lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page44lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page44lock0noKeyState {
        Page44lock0noKeyState::from_bits(val)
    }
}
impl From<Page44lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page44lock0noKeyState) -> u8 {
        Page44lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page44lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page44lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page44lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page44lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page44lock1lockBl {
        Page44lock1lockBl::from_bits(val)
    }
}
impl From<Page44lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page44lock1lockBl) -> u8 {
        Page44lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page44lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page44lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page44lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page44lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page44lock1lockNs {
        Page44lock1lockNs::from_bits(val)
    }
}
impl From<Page44lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page44lock1lockNs) -> u8 {
        Page44lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page44lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page44lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page44lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page44lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page44lock1lockS {
        Page44lock1lockS::from_bits(val)
    }
}
impl From<Page44lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page44lock1lockS) -> u8 {
        Page44lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page45lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page45lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page45lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page45lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page45lock0noKeyState {
        Page45lock0noKeyState::from_bits(val)
    }
}
impl From<Page45lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page45lock0noKeyState) -> u8 {
        Page45lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page45lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page45lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page45lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page45lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page45lock1lockBl {
        Page45lock1lockBl::from_bits(val)
    }
}
impl From<Page45lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page45lock1lockBl) -> u8 {
        Page45lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page45lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page45lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page45lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page45lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page45lock1lockNs {
        Page45lock1lockNs::from_bits(val)
    }
}
impl From<Page45lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page45lock1lockNs) -> u8 {
        Page45lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page45lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page45lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page45lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page45lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page45lock1lockS {
        Page45lock1lockS::from_bits(val)
    }
}
impl From<Page45lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page45lock1lockS) -> u8 {
        Page45lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page46lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page46lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page46lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page46lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page46lock0noKeyState {
        Page46lock0noKeyState::from_bits(val)
    }
}
impl From<Page46lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page46lock0noKeyState) -> u8 {
        Page46lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page46lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page46lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page46lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page46lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page46lock1lockBl {
        Page46lock1lockBl::from_bits(val)
    }
}
impl From<Page46lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page46lock1lockBl) -> u8 {
        Page46lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page46lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page46lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page46lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page46lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page46lock1lockNs {
        Page46lock1lockNs::from_bits(val)
    }
}
impl From<Page46lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page46lock1lockNs) -> u8 {
        Page46lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page46lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page46lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page46lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page46lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page46lock1lockS {
        Page46lock1lockS::from_bits(val)
    }
}
impl From<Page46lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page46lock1lockS) -> u8 {
        Page46lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page47lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page47lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page47lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page47lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page47lock0noKeyState {
        Page47lock0noKeyState::from_bits(val)
    }
}
impl From<Page47lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page47lock0noKeyState) -> u8 {
        Page47lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page47lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page47lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page47lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page47lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page47lock1lockBl {
        Page47lock1lockBl::from_bits(val)
    }
}
impl From<Page47lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page47lock1lockBl) -> u8 {
        Page47lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page47lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page47lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page47lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page47lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page47lock1lockNs {
        Page47lock1lockNs::from_bits(val)
    }
}
impl From<Page47lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page47lock1lockNs) -> u8 {
        Page47lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page47lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page47lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page47lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page47lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page47lock1lockS {
        Page47lock1lockS::from_bits(val)
    }
}
impl From<Page47lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page47lock1lockS) -> u8 {
        Page47lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page48lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page48lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page48lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page48lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page48lock0noKeyState {
        Page48lock0noKeyState::from_bits(val)
    }
}
impl From<Page48lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page48lock0noKeyState) -> u8 {
        Page48lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page48lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page48lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page48lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page48lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page48lock1lockBl {
        Page48lock1lockBl::from_bits(val)
    }
}
impl From<Page48lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page48lock1lockBl) -> u8 {
        Page48lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page48lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page48lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page48lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page48lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page48lock1lockNs {
        Page48lock1lockNs::from_bits(val)
    }
}
impl From<Page48lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page48lock1lockNs) -> u8 {
        Page48lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page48lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page48lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page48lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page48lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page48lock1lockS {
        Page48lock1lockS::from_bits(val)
    }
}
impl From<Page48lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page48lock1lockS) -> u8 {
        Page48lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page49lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page49lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page49lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page49lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page49lock0noKeyState {
        Page49lock0noKeyState::from_bits(val)
    }
}
impl From<Page49lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page49lock0noKeyState) -> u8 {
        Page49lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page49lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page49lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page49lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page49lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page49lock1lockBl {
        Page49lock1lockBl::from_bits(val)
    }
}
impl From<Page49lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page49lock1lockBl) -> u8 {
        Page49lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page49lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page49lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page49lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page49lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page49lock1lockNs {
        Page49lock1lockNs::from_bits(val)
    }
}
impl From<Page49lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page49lock1lockNs) -> u8 {
        Page49lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page49lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page49lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page49lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page49lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page49lock1lockS {
        Page49lock1lockS::from_bits(val)
    }
}
impl From<Page49lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page49lock1lockS) -> u8 {
        Page49lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page4lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page4lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page4lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page4lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page4lock0noKeyState {
        Page4lock0noKeyState::from_bits(val)
    }
}
impl From<Page4lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page4lock0noKeyState) -> u8 {
        Page4lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page4lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page4lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page4lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page4lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page4lock1lockBl {
        Page4lock1lockBl::from_bits(val)
    }
}
impl From<Page4lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page4lock1lockBl) -> u8 {
        Page4lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page4lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page4lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page4lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page4lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page4lock1lockNs {
        Page4lock1lockNs::from_bits(val)
    }
}
impl From<Page4lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page4lock1lockNs) -> u8 {
        Page4lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page4lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page4lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page4lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page4lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page4lock1lockS {
        Page4lock1lockS::from_bits(val)
    }
}
impl From<Page4lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page4lock1lockS) -> u8 {
        Page4lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page50lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page50lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page50lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page50lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page50lock0noKeyState {
        Page50lock0noKeyState::from_bits(val)
    }
}
impl From<Page50lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page50lock0noKeyState) -> u8 {
        Page50lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page50lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page50lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page50lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page50lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page50lock1lockBl {
        Page50lock1lockBl::from_bits(val)
    }
}
impl From<Page50lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page50lock1lockBl) -> u8 {
        Page50lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page50lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page50lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page50lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page50lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page50lock1lockNs {
        Page50lock1lockNs::from_bits(val)
    }
}
impl From<Page50lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page50lock1lockNs) -> u8 {
        Page50lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page50lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page50lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page50lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page50lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page50lock1lockS {
        Page50lock1lockS::from_bits(val)
    }
}
impl From<Page50lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page50lock1lockS) -> u8 {
        Page50lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page51lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page51lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page51lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page51lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page51lock0noKeyState {
        Page51lock0noKeyState::from_bits(val)
    }
}
impl From<Page51lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page51lock0noKeyState) -> u8 {
        Page51lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page51lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page51lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page51lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page51lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page51lock1lockBl {
        Page51lock1lockBl::from_bits(val)
    }
}
impl From<Page51lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page51lock1lockBl) -> u8 {
        Page51lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page51lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page51lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page51lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page51lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page51lock1lockNs {
        Page51lock1lockNs::from_bits(val)
    }
}
impl From<Page51lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page51lock1lockNs) -> u8 {
        Page51lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page51lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page51lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page51lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page51lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page51lock1lockS {
        Page51lock1lockS::from_bits(val)
    }
}
impl From<Page51lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page51lock1lockS) -> u8 {
        Page51lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page52lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page52lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page52lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page52lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page52lock0noKeyState {
        Page52lock0noKeyState::from_bits(val)
    }
}
impl From<Page52lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page52lock0noKeyState) -> u8 {
        Page52lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page52lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page52lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page52lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page52lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page52lock1lockBl {
        Page52lock1lockBl::from_bits(val)
    }
}
impl From<Page52lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page52lock1lockBl) -> u8 {
        Page52lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page52lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page52lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page52lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page52lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page52lock1lockNs {
        Page52lock1lockNs::from_bits(val)
    }
}
impl From<Page52lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page52lock1lockNs) -> u8 {
        Page52lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page52lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page52lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page52lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page52lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page52lock1lockS {
        Page52lock1lockS::from_bits(val)
    }
}
impl From<Page52lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page52lock1lockS) -> u8 {
        Page52lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page53lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page53lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page53lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page53lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page53lock0noKeyState {
        Page53lock0noKeyState::from_bits(val)
    }
}
impl From<Page53lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page53lock0noKeyState) -> u8 {
        Page53lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page53lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page53lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page53lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page53lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page53lock1lockBl {
        Page53lock1lockBl::from_bits(val)
    }
}
impl From<Page53lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page53lock1lockBl) -> u8 {
        Page53lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page53lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page53lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page53lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page53lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page53lock1lockNs {
        Page53lock1lockNs::from_bits(val)
    }
}
impl From<Page53lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page53lock1lockNs) -> u8 {
        Page53lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page53lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page53lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page53lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page53lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page53lock1lockS {
        Page53lock1lockS::from_bits(val)
    }
}
impl From<Page53lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page53lock1lockS) -> u8 {
        Page53lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page54lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page54lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page54lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page54lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page54lock0noKeyState {
        Page54lock0noKeyState::from_bits(val)
    }
}
impl From<Page54lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page54lock0noKeyState) -> u8 {
        Page54lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page54lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page54lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page54lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page54lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page54lock1lockBl {
        Page54lock1lockBl::from_bits(val)
    }
}
impl From<Page54lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page54lock1lockBl) -> u8 {
        Page54lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page54lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page54lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page54lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page54lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page54lock1lockNs {
        Page54lock1lockNs::from_bits(val)
    }
}
impl From<Page54lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page54lock1lockNs) -> u8 {
        Page54lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page54lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page54lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page54lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page54lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page54lock1lockS {
        Page54lock1lockS::from_bits(val)
    }
}
impl From<Page54lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page54lock1lockS) -> u8 {
        Page54lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page55lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page55lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page55lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page55lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page55lock0noKeyState {
        Page55lock0noKeyState::from_bits(val)
    }
}
impl From<Page55lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page55lock0noKeyState) -> u8 {
        Page55lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page55lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page55lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page55lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page55lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page55lock1lockBl {
        Page55lock1lockBl::from_bits(val)
    }
}
impl From<Page55lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page55lock1lockBl) -> u8 {
        Page55lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page55lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page55lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page55lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page55lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page55lock1lockNs {
        Page55lock1lockNs::from_bits(val)
    }
}
impl From<Page55lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page55lock1lockNs) -> u8 {
        Page55lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page55lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page55lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page55lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page55lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page55lock1lockS {
        Page55lock1lockS::from_bits(val)
    }
}
impl From<Page55lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page55lock1lockS) -> u8 {
        Page55lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page56lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page56lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page56lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page56lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page56lock0noKeyState {
        Page56lock0noKeyState::from_bits(val)
    }
}
impl From<Page56lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page56lock0noKeyState) -> u8 {
        Page56lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page56lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page56lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page56lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page56lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page56lock1lockBl {
        Page56lock1lockBl::from_bits(val)
    }
}
impl From<Page56lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page56lock1lockBl) -> u8 {
        Page56lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page56lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page56lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page56lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page56lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page56lock1lockNs {
        Page56lock1lockNs::from_bits(val)
    }
}
impl From<Page56lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page56lock1lockNs) -> u8 {
        Page56lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page56lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page56lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page56lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page56lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page56lock1lockS {
        Page56lock1lockS::from_bits(val)
    }
}
impl From<Page56lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page56lock1lockS) -> u8 {
        Page56lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page57lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page57lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page57lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page57lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page57lock0noKeyState {
        Page57lock0noKeyState::from_bits(val)
    }
}
impl From<Page57lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page57lock0noKeyState) -> u8 {
        Page57lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page57lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page57lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page57lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page57lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page57lock1lockBl {
        Page57lock1lockBl::from_bits(val)
    }
}
impl From<Page57lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page57lock1lockBl) -> u8 {
        Page57lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page57lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page57lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page57lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page57lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page57lock1lockNs {
        Page57lock1lockNs::from_bits(val)
    }
}
impl From<Page57lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page57lock1lockNs) -> u8 {
        Page57lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page57lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page57lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page57lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page57lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page57lock1lockS {
        Page57lock1lockS::from_bits(val)
    }
}
impl From<Page57lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page57lock1lockS) -> u8 {
        Page57lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page58lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page58lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page58lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page58lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page58lock0noKeyState {
        Page58lock0noKeyState::from_bits(val)
    }
}
impl From<Page58lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page58lock0noKeyState) -> u8 {
        Page58lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page58lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page58lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page58lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page58lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page58lock1lockBl {
        Page58lock1lockBl::from_bits(val)
    }
}
impl From<Page58lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page58lock1lockBl) -> u8 {
        Page58lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page58lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page58lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page58lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page58lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page58lock1lockNs {
        Page58lock1lockNs::from_bits(val)
    }
}
impl From<Page58lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page58lock1lockNs) -> u8 {
        Page58lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page58lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page58lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page58lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page58lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page58lock1lockS {
        Page58lock1lockS::from_bits(val)
    }
}
impl From<Page58lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page58lock1lockS) -> u8 {
        Page58lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page59lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page59lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page59lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page59lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page59lock0noKeyState {
        Page59lock0noKeyState::from_bits(val)
    }
}
impl From<Page59lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page59lock0noKeyState) -> u8 {
        Page59lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page59lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page59lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page59lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page59lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page59lock1lockBl {
        Page59lock1lockBl::from_bits(val)
    }
}
impl From<Page59lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page59lock1lockBl) -> u8 {
        Page59lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page59lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page59lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page59lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page59lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page59lock1lockNs {
        Page59lock1lockNs::from_bits(val)
    }
}
impl From<Page59lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page59lock1lockNs) -> u8 {
        Page59lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page59lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page59lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page59lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page59lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page59lock1lockS {
        Page59lock1lockS::from_bits(val)
    }
}
impl From<Page59lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page59lock1lockS) -> u8 {
        Page59lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page5lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page5lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page5lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page5lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page5lock0noKeyState {
        Page5lock0noKeyState::from_bits(val)
    }
}
impl From<Page5lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page5lock0noKeyState) -> u8 {
        Page5lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page5lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page5lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page5lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page5lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page5lock1lockBl {
        Page5lock1lockBl::from_bits(val)
    }
}
impl From<Page5lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page5lock1lockBl) -> u8 {
        Page5lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page5lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page5lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page5lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page5lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page5lock1lockNs {
        Page5lock1lockNs::from_bits(val)
    }
}
impl From<Page5lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page5lock1lockNs) -> u8 {
        Page5lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page5lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page5lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page5lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page5lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page5lock1lockS {
        Page5lock1lockS::from_bits(val)
    }
}
impl From<Page5lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page5lock1lockS) -> u8 {
        Page5lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page60lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page60lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page60lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page60lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page60lock0noKeyState {
        Page60lock0noKeyState::from_bits(val)
    }
}
impl From<Page60lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page60lock0noKeyState) -> u8 {
        Page60lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page60lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page60lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page60lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page60lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page60lock1lockBl {
        Page60lock1lockBl::from_bits(val)
    }
}
impl From<Page60lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page60lock1lockBl) -> u8 {
        Page60lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page60lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page60lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page60lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page60lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page60lock1lockNs {
        Page60lock1lockNs::from_bits(val)
    }
}
impl From<Page60lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page60lock1lockNs) -> u8 {
        Page60lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page60lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page60lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page60lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page60lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page60lock1lockS {
        Page60lock1lockS::from_bits(val)
    }
}
impl From<Page60lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page60lock1lockS) -> u8 {
        Page60lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page61lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page61lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page61lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page61lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page61lock0noKeyState {
        Page61lock0noKeyState::from_bits(val)
    }
}
impl From<Page61lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page61lock0noKeyState) -> u8 {
        Page61lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page61lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page61lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page61lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page61lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page61lock1lockBl {
        Page61lock1lockBl::from_bits(val)
    }
}
impl From<Page61lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page61lock1lockBl) -> u8 {
        Page61lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page61lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page61lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page61lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page61lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page61lock1lockNs {
        Page61lock1lockNs::from_bits(val)
    }
}
impl From<Page61lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page61lock1lockNs) -> u8 {
        Page61lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page61lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page61lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page61lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page61lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page61lock1lockS {
        Page61lock1lockS::from_bits(val)
    }
}
impl From<Page61lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page61lock1lockS) -> u8 {
        Page61lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page62lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page62lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page62lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page62lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page62lock0noKeyState {
        Page62lock0noKeyState::from_bits(val)
    }
}
impl From<Page62lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page62lock0noKeyState) -> u8 {
        Page62lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page62lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page62lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page62lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page62lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page62lock1lockBl {
        Page62lock1lockBl::from_bits(val)
    }
}
impl From<Page62lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page62lock1lockBl) -> u8 {
        Page62lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page62lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page62lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page62lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page62lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page62lock1lockNs {
        Page62lock1lockNs::from_bits(val)
    }
}
impl From<Page62lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page62lock1lockNs) -> u8 {
        Page62lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page62lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page62lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page62lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page62lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page62lock1lockS {
        Page62lock1lockS::from_bits(val)
    }
}
impl From<Page62lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page62lock1lockS) -> u8 {
        Page62lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page63lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page63lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page63lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page63lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page63lock0noKeyState {
        Page63lock0noKeyState::from_bits(val)
    }
}
impl From<Page63lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page63lock0noKeyState) -> u8 {
        Page63lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page63lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page63lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page63lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page63lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page63lock1lockBl {
        Page63lock1lockBl::from_bits(val)
    }
}
impl From<Page63lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page63lock1lockBl) -> u8 {
        Page63lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page63lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page63lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page63lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page63lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page63lock1lockNs {
        Page63lock1lockNs::from_bits(val)
    }
}
impl From<Page63lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page63lock1lockNs) -> u8 {
        Page63lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page63lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page63lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page63lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page63lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page63lock1lockS {
        Page63lock1lockS::from_bits(val)
    }
}
impl From<Page63lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page63lock1lockS) -> u8 {
        Page63lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page6lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page6lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page6lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page6lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page6lock0noKeyState {
        Page6lock0noKeyState::from_bits(val)
    }
}
impl From<Page6lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page6lock0noKeyState) -> u8 {
        Page6lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page6lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page6lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page6lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page6lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page6lock1lockBl {
        Page6lock1lockBl::from_bits(val)
    }
}
impl From<Page6lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page6lock1lockBl) -> u8 {
        Page6lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page6lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page6lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page6lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page6lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page6lock1lockNs {
        Page6lock1lockNs::from_bits(val)
    }
}
impl From<Page6lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page6lock1lockNs) -> u8 {
        Page6lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page6lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page6lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page6lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page6lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page6lock1lockS {
        Page6lock1lockS::from_bits(val)
    }
}
impl From<Page6lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page6lock1lockS) -> u8 {
        Page6lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page7lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page7lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page7lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page7lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page7lock0noKeyState {
        Page7lock0noKeyState::from_bits(val)
    }
}
impl From<Page7lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page7lock0noKeyState) -> u8 {
        Page7lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page7lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page7lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page7lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page7lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page7lock1lockBl {
        Page7lock1lockBl::from_bits(val)
    }
}
impl From<Page7lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page7lock1lockBl) -> u8 {
        Page7lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page7lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page7lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page7lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page7lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page7lock1lockNs {
        Page7lock1lockNs::from_bits(val)
    }
}
impl From<Page7lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page7lock1lockNs) -> u8 {
        Page7lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page7lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page7lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page7lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page7lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page7lock1lockS {
        Page7lock1lockS::from_bits(val)
    }
}
impl From<Page7lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page7lock1lockS) -> u8 {
        Page7lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page8lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page8lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page8lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page8lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page8lock0noKeyState {
        Page8lock0noKeyState::from_bits(val)
    }
}
impl From<Page8lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page8lock0noKeyState) -> u8 {
        Page8lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page8lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page8lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page8lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page8lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page8lock1lockBl {
        Page8lock1lockBl::from_bits(val)
    }
}
impl From<Page8lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page8lock1lockBl) -> u8 {
        Page8lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page8lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page8lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page8lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page8lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page8lock1lockNs {
        Page8lock1lockNs::from_bits(val)
    }
}
impl From<Page8lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page8lock1lockNs) -> u8 {
        Page8lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page8lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page8lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page8lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page8lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page8lock1lockS {
        Page8lock1lockS::from_bits(val)
    }
}
impl From<Page8lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page8lock1lockS) -> u8 {
        Page8lock1lockS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page9lock0noKeyState {
    READ_ONLY = 0,
    INACCESSIBLE = 0x01,
}
impl Page9lock0noKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page9lock0noKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page9lock0noKeyState {
    #[inline(always)]
    fn from(val: u8) -> Page9lock0noKeyState {
        Page9lock0noKeyState::from_bits(val)
    }
}
impl From<Page9lock0noKeyState> for u8 {
    #[inline(always)]
    fn from(val: Page9lock0noKeyState) -> u8 {
        Page9lock0noKeyState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page9lock1lockBl {
    #[doc = "Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "Bootloader permits user reads of this page"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE"]
    RESERVED = 0x02,
    #[doc = "Bootloader does not permit user access to this page"]
    INACCESSIBLE = 0x03,
}
impl Page9lock1lockBl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page9lock1lockBl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page9lock1lockBl {
    #[inline(always)]
    fn from(val: u8) -> Page9lock1lockBl {
        Page9lock1lockBl::from_bits(val)
    }
}
impl From<Page9lock1lockBl> for u8 {
    #[inline(always)]
    fn from(val: Page9lock1lockBl) -> u8 {
        Page9lock1lockBl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page9lock1lockNs {
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Non-secure software"]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 0x03,
}
impl Page9lock1lockNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page9lock1lockNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page9lock1lockNs {
    #[inline(always)]
    fn from(val: u8) -> Page9lock1lockNs {
        Page9lock1lockNs::from_bits(val)
    }
}
impl From<Page9lock1lockNs> for u8 {
    #[inline(always)]
    fn from(val: Page9lock1lockNs) -> u8 {
        Page9lock1lockNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Page9lock1lockS {
    #[doc = "Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "Page can be read by Secure software, but can not be written."]
    READ_ONLY = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    RESERVED = 0x02,
    #[doc = "Page can not be accessed by Secure software."]
    INACCESSIBLE = 0x03,
}
impl Page9lock1lockS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Page9lock1lockS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Page9lock1lockS {
    #[inline(always)]
    fn from(val: u8) -> Page9lock1lockS {
        Page9lock1lockS::from_bits(val)
    }
}
impl From<Page9lock1lockS> for u8 {
    #[inline(always)]
    fn from(val: Page9lock1lockS) -> u8 {
        Page9lock1lockS::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Range(pub u16);
impl Range {
    pub const _1_15MHZ: Self = Self(0);
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
