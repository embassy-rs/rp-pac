#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ClkSys {
    TICK = 0,
    CLK_SYS = 0x01,
}
impl ClkSys {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkSys {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkSys {
    #[inline(always)]
    fn from(val: u8) -> ClkSys {
        ClkSys::from_bits(val)
    }
}
impl From<ClkSys> for u8 {
    #[inline(always)]
    fn from(val: ClkSys) -> u8 {
        ClkSys::to_bits(val)
    }
}
