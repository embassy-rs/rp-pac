#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pwrup0direction {
    LOW_FALLING = 0,
    HIGH_RISING = 0x01,
}
impl Pwrup0direction {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwrup0direction {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwrup0direction {
    #[inline(always)]
    fn from(val: u8) -> Pwrup0direction {
        Pwrup0direction::from_bits(val)
    }
}
impl From<Pwrup0direction> for u8 {
    #[inline(always)]
    fn from(val: Pwrup0direction) -> u8 {
        Pwrup0direction::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pwrup0mode {
    LEVEL = 0,
    EDGE = 0x01,
}
impl Pwrup0mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwrup0mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwrup0mode {
    #[inline(always)]
    fn from(val: u8) -> Pwrup0mode {
        Pwrup0mode::from_bits(val)
    }
}
impl From<Pwrup0mode> for u8 {
    #[inline(always)]
    fn from(val: Pwrup0mode) -> u8 {
        Pwrup0mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pwrup1direction {
    LOW_FALLING = 0,
    HIGH_RISING = 0x01,
}
impl Pwrup1direction {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwrup1direction {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwrup1direction {
    #[inline(always)]
    fn from(val: u8) -> Pwrup1direction {
        Pwrup1direction::from_bits(val)
    }
}
impl From<Pwrup1direction> for u8 {
    #[inline(always)]
    fn from(val: Pwrup1direction) -> u8 {
        Pwrup1direction::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pwrup1mode {
    LEVEL = 0,
    EDGE = 0x01,
}
impl Pwrup1mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwrup1mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwrup1mode {
    #[inline(always)]
    fn from(val: u8) -> Pwrup1mode {
        Pwrup1mode::from_bits(val)
    }
}
impl From<Pwrup1mode> for u8 {
    #[inline(always)]
    fn from(val: Pwrup1mode) -> u8 {
        Pwrup1mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pwrup2direction {
    LOW_FALLING = 0,
    HIGH_RISING = 0x01,
}
impl Pwrup2direction {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwrup2direction {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwrup2direction {
    #[inline(always)]
    fn from(val: u8) -> Pwrup2direction {
        Pwrup2direction::from_bits(val)
    }
}
impl From<Pwrup2direction> for u8 {
    #[inline(always)]
    fn from(val: Pwrup2direction) -> u8 {
        Pwrup2direction::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pwrup2mode {
    LEVEL = 0,
    EDGE = 0x01,
}
impl Pwrup2mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwrup2mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwrup2mode {
    #[inline(always)]
    fn from(val: u8) -> Pwrup2mode {
        Pwrup2mode::from_bits(val)
    }
}
impl From<Pwrup2mode> for u8 {
    #[inline(always)]
    fn from(val: Pwrup2mode) -> u8 {
        Pwrup2mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pwrup3direction {
    LOW_FALLING = 0,
    HIGH_RISING = 0x01,
}
impl Pwrup3direction {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwrup3direction {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwrup3direction {
    #[inline(always)]
    fn from(val: u8) -> Pwrup3direction {
        Pwrup3direction::from_bits(val)
    }
}
impl From<Pwrup3direction> for u8 {
    #[inline(always)]
    fn from(val: Pwrup3direction) -> u8 {
        Pwrup3direction::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pwrup3mode {
    LEVEL = 0,
    EDGE = 0x01,
}
impl Pwrup3mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwrup3mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwrup3mode {
    #[inline(always)]
    fn from(val: u8) -> Pwrup3mode {
        Pwrup3mode::from_bits(val)
    }
}
impl From<Pwrup3mode> for u8 {
    #[inline(always)]
    fn from(val: Pwrup3mode) -> u8 {
        Pwrup3mode::to_bits(val)
    }
}
