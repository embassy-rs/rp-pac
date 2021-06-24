#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Speed(pub u8);
impl Speed {
    #[doc = "Standard Speed mode of operation"]
    pub const STANDARD: Self = Self(0x01);
    #[doc = "Fast or Fast Plus mode of operation"]
    pub const FAST: Self = Self(0x02);
    #[doc = "High Speed mode of operation"]
    pub const HIGH: Self = Self(0x03);
}
