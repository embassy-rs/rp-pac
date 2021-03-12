use crate::generic::*;
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Oeover(u8);
impl Oeover {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Oeover {
        Oeover(val)
    }
    #[doc = "drive output enable from peripheral signal selected by funcsel"]
    pub const NORMAL: Self = Self(0);
    #[doc = "drive output enable from inverse of peripheral signal selected by funcsel"]
    pub const INVERT: Self = Self(0x01);
    #[doc = "disable output"]
    pub const DISABLE: Self = Self(0x02);
    #[doc = "enable output"]
    pub const ENABLE: Self = Self(0x03);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Inover(u8);
impl Inover {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Inover {
        Inover(val)
    }
    #[doc = "don't invert the peri input"]
    pub const NORMAL: Self = Self(0);
    #[doc = "invert the peri input"]
    pub const INVERT: Self = Self(0x01);
    #[doc = "drive peri input low"]
    pub const LOW: Self = Self(0x02);
    #[doc = "drive peri input high"]
    pub const HIGH: Self = Self(0x03);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Irqover(u8);
impl Irqover {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Irqover {
        Irqover(val)
    }
    #[doc = "don't invert the interrupt"]
    pub const NORMAL: Self = Self(0);
    #[doc = "invert the interrupt"]
    pub const INVERT: Self = Self(0x01);
    #[doc = "drive interrupt low"]
    pub const LOW: Self = Self(0x02);
    #[doc = "drive interrupt high"]
    pub const HIGH: Self = Self(0x03);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Outover(u8);
impl Outover {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Outover {
        Outover(val)
    }
    #[doc = "drive output from peripheral signal selected by funcsel"]
    pub const NORMAL: Self = Self(0);
    #[doc = "drive output from inverse of peripheral signal selected by funcsel"]
    pub const INVERT: Self = Self(0x01);
    #[doc = "drive output low"]
    pub const LOW: Self = Self(0x02);
    #[doc = "drive output high"]
    pub const HIGH: Self = Self(0x03);
}
