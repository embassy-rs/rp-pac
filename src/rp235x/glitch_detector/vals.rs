#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Arm(pub u16);
impl Arm {
    #[doc = "Force the glitch detectors to be armed. (Any value other than ARM_NO counts as YES)"]
    pub const YES: Self = Self(0);
    #[doc = "Do not force the glitch detectors to be armed"]
    pub const NO: Self = Self(0x5bad);
}
impl Arm {
    pub const fn from_bits(val: u16) -> Arm {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl From<u16> for Arm {
    #[inline(always)]
    fn from(val: u16) -> Arm {
        Arm::from_bits(val)
    }
}
impl From<Arm> for u16 {
    #[inline(always)]
    fn from(val: Arm) -> u16 {
        Arm::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Default(pub u8);
impl Default {
    #[doc = "Use the default sensitivity configured in OTP for all detectors. (Any value other than DEFAULT_NO counts as YES)"]
    pub const YES: Self = Self(0);
    #[doc = "Do not use the default sensitivity configured in OTP. Instead use the value from this register."]
    pub const NO: Self = Self(0xde);
}
impl Default {
    pub const fn from_bits(val: u8) -> Default {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Default {
    #[inline(always)]
    fn from(val: u8) -> Default {
        Default::from_bits(val)
    }
}
impl From<Default> for u8 {
    #[inline(always)]
    fn from(val: Default) -> u8 {
        Default::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Disarm(pub u16);
impl Disarm {
    #[doc = "Do not disarm the glitch detectors. (Any value other than DISARM_YES counts as NO)"]
    pub const NO: Self = Self(0);
    #[doc = "Disarm the glitch detectors"]
    pub const YES: Self = Self(0xdcaf);
}
impl Disarm {
    pub const fn from_bits(val: u16) -> Disarm {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl From<u16> for Disarm {
    #[inline(always)]
    fn from(val: u16) -> Disarm {
        Disarm::from_bits(val)
    }
}
impl From<Disarm> for u16 {
    #[inline(always)]
    fn from(val: Disarm) -> u16 {
        Disarm::to_bits(val)
    }
}
