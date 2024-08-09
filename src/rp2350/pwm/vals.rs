#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ch10csrDivmode {
    #[doc = "Free-running counting at rate dictated by fractional divider"]
    DIV = 0,
    #[doc = "Fractional divider operation is gated by the PWM B pin."]
    LEVEL = 0x01,
    #[doc = "Counter advances with each rising edge of the PWM B pin."]
    RISE = 0x02,
    #[doc = "Counter advances with each falling edge of the PWM B pin."]
    FALL = 0x03,
}
impl Ch10csrDivmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch10csrDivmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch10csrDivmode {
    #[inline(always)]
    fn from(val: u8) -> Ch10csrDivmode {
        Ch10csrDivmode::from_bits(val)
    }
}
impl From<Ch10csrDivmode> for u8 {
    #[inline(always)]
    fn from(val: Ch10csrDivmode) -> u8 {
        Ch10csrDivmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ch11csrDivmode {
    #[doc = "Free-running counting at rate dictated by fractional divider"]
    DIV = 0,
    #[doc = "Fractional divider operation is gated by the PWM B pin."]
    LEVEL = 0x01,
    #[doc = "Counter advances with each rising edge of the PWM B pin."]
    RISE = 0x02,
    #[doc = "Counter advances with each falling edge of the PWM B pin."]
    FALL = 0x03,
}
impl Ch11csrDivmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch11csrDivmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch11csrDivmode {
    #[inline(always)]
    fn from(val: u8) -> Ch11csrDivmode {
        Ch11csrDivmode::from_bits(val)
    }
}
impl From<Ch11csrDivmode> for u8 {
    #[inline(always)]
    fn from(val: Ch11csrDivmode) -> u8 {
        Ch11csrDivmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Divmode {
    #[doc = "Free-running counting at rate dictated by fractional divider"]
    DIV = 0,
    #[doc = "Fractional divider operation is gated by the PWM B pin."]
    LEVEL = 0x01,
    #[doc = "Counter advances with each rising edge of the PWM B pin."]
    RISE = 0x02,
    #[doc = "Counter advances with each falling edge of the PWM B pin."]
    FALL = 0x03,
}
impl Divmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Divmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Divmode {
    #[inline(always)]
    fn from(val: u8) -> Divmode {
        Divmode::from_bits(val)
    }
}
impl From<Divmode> for u8 {
    #[inline(always)]
    fn from(val: Divmode) -> u8 {
        Divmode::to_bits(val)
    }
}
