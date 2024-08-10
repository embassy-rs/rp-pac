#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Calc {
    #[doc = "Calculate a CRC-32 (IEEE802.3 polynomial)"]
    CRC32 = 0,
    #[doc = "Calculate a CRC-32 (IEEE802.3 polynomial) with bit reversed data"]
    CRC32R = 0x01,
    #[doc = "Calculate a CRC-16-CCITT"]
    CRC16 = 0x02,
    #[doc = "Calculate a CRC-16-CCITT with bit reversed data"]
    CRC16R = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "XOR reduction over all data. == 1 if the total 1 population count is odd."]
    EVEN = 0x0e,
    #[doc = "Calculate a simple 32-bit checksum (addition with a 32 bit accumulator)"]
    SUM = 0x0f,
}
impl Calc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Calc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Calc {
    #[inline(always)]
    fn from(val: u8) -> Calc {
        Calc::from_bits(val)
    }
}
impl From<Calc> for u8 {
    #[inline(always)]
    fn from(val: Calc) -> u8 {
        Calc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ch0transCountMode {
    NORMAL = 0,
    TRIGGER_SELF = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    ENDLESS = 0x0f,
}
impl Ch0transCountMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch0transCountMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch0transCountMode {
    #[inline(always)]
    fn from(val: u8) -> Ch0transCountMode {
        Ch0transCountMode::from_bits(val)
    }
}
impl From<Ch0transCountMode> for u8 {
    #[inline(always)]
    fn from(val: Ch0transCountMode) -> u8 {
        Ch0transCountMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ch10transCountMode {
    NORMAL = 0,
    TRIGGER_SELF = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    ENDLESS = 0x0f,
}
impl Ch10transCountMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch10transCountMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch10transCountMode {
    #[inline(always)]
    fn from(val: u8) -> Ch10transCountMode {
        Ch10transCountMode::from_bits(val)
    }
}
impl From<Ch10transCountMode> for u8 {
    #[inline(always)]
    fn from(val: Ch10transCountMode) -> u8 {
        Ch10transCountMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ch11transCountMode {
    NORMAL = 0,
    TRIGGER_SELF = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    ENDLESS = 0x0f,
}
impl Ch11transCountMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch11transCountMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch11transCountMode {
    #[inline(always)]
    fn from(val: u8) -> Ch11transCountMode {
        Ch11transCountMode::from_bits(val)
    }
}
impl From<Ch11transCountMode> for u8 {
    #[inline(always)]
    fn from(val: Ch11transCountMode) -> u8 {
        Ch11transCountMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ch12transCountMode {
    NORMAL = 0,
    TRIGGER_SELF = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    ENDLESS = 0x0f,
}
impl Ch12transCountMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch12transCountMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch12transCountMode {
    #[inline(always)]
    fn from(val: u8) -> Ch12transCountMode {
        Ch12transCountMode::from_bits(val)
    }
}
impl From<Ch12transCountMode> for u8 {
    #[inline(always)]
    fn from(val: Ch12transCountMode) -> u8 {
        Ch12transCountMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ch13transCountMode {
    NORMAL = 0,
    TRIGGER_SELF = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    ENDLESS = 0x0f,
}
impl Ch13transCountMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch13transCountMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch13transCountMode {
    #[inline(always)]
    fn from(val: u8) -> Ch13transCountMode {
        Ch13transCountMode::from_bits(val)
    }
}
impl From<Ch13transCountMode> for u8 {
    #[inline(always)]
    fn from(val: Ch13transCountMode) -> u8 {
        Ch13transCountMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ch14transCountMode {
    NORMAL = 0,
    TRIGGER_SELF = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    ENDLESS = 0x0f,
}
impl Ch14transCountMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch14transCountMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch14transCountMode {
    #[inline(always)]
    fn from(val: u8) -> Ch14transCountMode {
        Ch14transCountMode::from_bits(val)
    }
}
impl From<Ch14transCountMode> for u8 {
    #[inline(always)]
    fn from(val: Ch14transCountMode) -> u8 {
        Ch14transCountMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ch15transCountMode {
    NORMAL = 0,
    TRIGGER_SELF = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    ENDLESS = 0x0f,
}
impl Ch15transCountMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch15transCountMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch15transCountMode {
    #[inline(always)]
    fn from(val: u8) -> Ch15transCountMode {
        Ch15transCountMode::from_bits(val)
    }
}
impl From<Ch15transCountMode> for u8 {
    #[inline(always)]
    fn from(val: Ch15transCountMode) -> u8 {
        Ch15transCountMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ch1transCountMode {
    NORMAL = 0,
    TRIGGER_SELF = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    ENDLESS = 0x0f,
}
impl Ch1transCountMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch1transCountMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch1transCountMode {
    #[inline(always)]
    fn from(val: u8) -> Ch1transCountMode {
        Ch1transCountMode::from_bits(val)
    }
}
impl From<Ch1transCountMode> for u8 {
    #[inline(always)]
    fn from(val: Ch1transCountMode) -> u8 {
        Ch1transCountMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ch2transCountMode {
    NORMAL = 0,
    TRIGGER_SELF = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    ENDLESS = 0x0f,
}
impl Ch2transCountMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch2transCountMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch2transCountMode {
    #[inline(always)]
    fn from(val: u8) -> Ch2transCountMode {
        Ch2transCountMode::from_bits(val)
    }
}
impl From<Ch2transCountMode> for u8 {
    #[inline(always)]
    fn from(val: Ch2transCountMode) -> u8 {
        Ch2transCountMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ch3transCountMode {
    NORMAL = 0,
    TRIGGER_SELF = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    ENDLESS = 0x0f,
}
impl Ch3transCountMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch3transCountMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch3transCountMode {
    #[inline(always)]
    fn from(val: u8) -> Ch3transCountMode {
        Ch3transCountMode::from_bits(val)
    }
}
impl From<Ch3transCountMode> for u8 {
    #[inline(always)]
    fn from(val: Ch3transCountMode) -> u8 {
        Ch3transCountMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ch4transCountMode {
    NORMAL = 0,
    TRIGGER_SELF = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    ENDLESS = 0x0f,
}
impl Ch4transCountMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch4transCountMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch4transCountMode {
    #[inline(always)]
    fn from(val: u8) -> Ch4transCountMode {
        Ch4transCountMode::from_bits(val)
    }
}
impl From<Ch4transCountMode> for u8 {
    #[inline(always)]
    fn from(val: Ch4transCountMode) -> u8 {
        Ch4transCountMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ch5transCountMode {
    NORMAL = 0,
    TRIGGER_SELF = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    ENDLESS = 0x0f,
}
impl Ch5transCountMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch5transCountMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch5transCountMode {
    #[inline(always)]
    fn from(val: u8) -> Ch5transCountMode {
        Ch5transCountMode::from_bits(val)
    }
}
impl From<Ch5transCountMode> for u8 {
    #[inline(always)]
    fn from(val: Ch5transCountMode) -> u8 {
        Ch5transCountMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ch6transCountMode {
    NORMAL = 0,
    TRIGGER_SELF = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    ENDLESS = 0x0f,
}
impl Ch6transCountMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch6transCountMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch6transCountMode {
    #[inline(always)]
    fn from(val: u8) -> Ch6transCountMode {
        Ch6transCountMode::from_bits(val)
    }
}
impl From<Ch6transCountMode> for u8 {
    #[inline(always)]
    fn from(val: Ch6transCountMode) -> u8 {
        Ch6transCountMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ch7transCountMode {
    NORMAL = 0,
    TRIGGER_SELF = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    ENDLESS = 0x0f,
}
impl Ch7transCountMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch7transCountMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch7transCountMode {
    #[inline(always)]
    fn from(val: u8) -> Ch7transCountMode {
        Ch7transCountMode::from_bits(val)
    }
}
impl From<Ch7transCountMode> for u8 {
    #[inline(always)]
    fn from(val: Ch7transCountMode) -> u8 {
        Ch7transCountMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ch8transCountMode {
    NORMAL = 0,
    TRIGGER_SELF = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    ENDLESS = 0x0f,
}
impl Ch8transCountMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch8transCountMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch8transCountMode {
    #[inline(always)]
    fn from(val: u8) -> Ch8transCountMode {
        Ch8transCountMode::from_bits(val)
    }
}
impl From<Ch8transCountMode> for u8 {
    #[inline(always)]
    fn from(val: Ch8transCountMode) -> u8 {
        Ch8transCountMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ch9transCountMode {
    NORMAL = 0,
    TRIGGER_SELF = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    ENDLESS = 0x0f,
}
impl Ch9transCountMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch9transCountMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch9transCountMode {
    #[inline(always)]
    fn from(val: u8) -> Ch9transCountMode {
        Ch9transCountMode::from_bits(val)
    }
}
impl From<Ch9transCountMode> for u8 {
    #[inline(always)]
    fn from(val: Ch9transCountMode) -> u8 {
        Ch9transCountMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DataSize {
    SIZE_BYTE = 0,
    SIZE_HALFWORD = 0x01,
    SIZE_WORD = 0x02,
    _RESERVED_3 = 0x03,
}
impl DataSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DataSize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DataSize {
    #[inline(always)]
    fn from(val: u8) -> DataSize {
        DataSize::from_bits(val)
    }
}
impl From<DataSize> for u8 {
    #[inline(always)]
    fn from(val: DataSize) -> u8 {
        DataSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TreqSel {
    #[doc = "Select PIO0's TX FIFO 0 as TREQ"]
    PIO0_TX0 = 0,
    #[doc = "Select PIO0's TX FIFO 1 as TREQ"]
    PIO0_TX1 = 0x01,
    #[doc = "Select PIO0's TX FIFO 2 as TREQ"]
    PIO0_TX2 = 0x02,
    #[doc = "Select PIO0's TX FIFO 3 as TREQ"]
    PIO0_TX3 = 0x03,
    #[doc = "Select PIO0's RX FIFO 0 as TREQ"]
    PIO0_RX0 = 0x04,
    #[doc = "Select PIO0's RX FIFO 1 as TREQ"]
    PIO0_RX1 = 0x05,
    #[doc = "Select PIO0's RX FIFO 2 as TREQ"]
    PIO0_RX2 = 0x06,
    #[doc = "Select PIO0's RX FIFO 3 as TREQ"]
    PIO0_RX3 = 0x07,
    #[doc = "Select PIO1's TX FIFO 0 as TREQ"]
    PIO1_TX0 = 0x08,
    #[doc = "Select PIO1's TX FIFO 1 as TREQ"]
    PIO1_TX1 = 0x09,
    #[doc = "Select PIO1's TX FIFO 2 as TREQ"]
    PIO1_TX2 = 0x0a,
    #[doc = "Select PIO1's TX FIFO 3 as TREQ"]
    PIO1_TX3 = 0x0b,
    #[doc = "Select PIO1's RX FIFO 0 as TREQ"]
    PIO1_RX0 = 0x0c,
    #[doc = "Select PIO1's RX FIFO 1 as TREQ"]
    PIO1_RX1 = 0x0d,
    #[doc = "Select PIO1's RX FIFO 2 as TREQ"]
    PIO1_RX2 = 0x0e,
    #[doc = "Select PIO1's RX FIFO 3 as TREQ"]
    PIO1_RX3 = 0x0f,
    #[doc = "Select PIO2's TX FIFO 0 as TREQ"]
    PIO2_TX0 = 0x10,
    #[doc = "Select PIO2's TX FIFO 1 as TREQ"]
    PIO2_TX1 = 0x11,
    #[doc = "Select PIO2's TX FIFO 2 as TREQ"]
    PIO2_TX2 = 0x12,
    #[doc = "Select PIO2's TX FIFO 3 as TREQ"]
    PIO2_TX3 = 0x13,
    #[doc = "Select PIO2's RX FIFO 0 as TREQ"]
    PIO2_RX0 = 0x14,
    #[doc = "Select PIO2's RX FIFO 1 as TREQ"]
    PIO2_RX1 = 0x15,
    #[doc = "Select PIO2's RX FIFO 2 as TREQ"]
    PIO2_RX2 = 0x16,
    #[doc = "Select PIO2's RX FIFO 3 as TREQ"]
    PIO2_RX3 = 0x17,
    #[doc = "Select SPI0's TX FIFO as TREQ"]
    SPI0_TX = 0x18,
    #[doc = "Select SPI0's RX FIFO as TREQ"]
    SPI0_RX = 0x19,
    #[doc = "Select SPI1's TX FIFO as TREQ"]
    SPI1_TX = 0x1a,
    #[doc = "Select SPI1's RX FIFO as TREQ"]
    SPI1_RX = 0x1b,
    #[doc = "Select UART0's TX FIFO as TREQ"]
    UART0_TX = 0x1c,
    #[doc = "Select UART0's RX FIFO as TREQ"]
    UART0_RX = 0x1d,
    #[doc = "Select UART1's TX FIFO as TREQ"]
    UART1_TX = 0x1e,
    #[doc = "Select UART1's RX FIFO as TREQ"]
    UART1_RX = 0x1f,
    #[doc = "Select PWM Counter 0's Wrap Value as TREQ"]
    PWM_WRAP0 = 0x20,
    #[doc = "Select PWM Counter 1's Wrap Value as TREQ"]
    PWM_WRAP1 = 0x21,
    #[doc = "Select PWM Counter 2's Wrap Value as TREQ"]
    PWM_WRAP2 = 0x22,
    #[doc = "Select PWM Counter 3's Wrap Value as TREQ"]
    PWM_WRAP3 = 0x23,
    #[doc = "Select PWM Counter 4's Wrap Value as TREQ"]
    PWM_WRAP4 = 0x24,
    #[doc = "Select PWM Counter 5's Wrap Value as TREQ"]
    PWM_WRAP5 = 0x25,
    #[doc = "Select PWM Counter 6's Wrap Value as TREQ"]
    PWM_WRAP6 = 0x26,
    #[doc = "Select PWM Counter 7's Wrap Value as TREQ"]
    PWM_WRAP7 = 0x27,
    #[doc = "Select PWM Counter 8's Wrap Value as TREQ"]
    PWM_WRAP8 = 0x28,
    #[doc = "Select PWM Counter 9's Wrap Value as TREQ"]
    PWM_WRAP9 = 0x29,
    #[doc = "Select PWM Counter 0's Wrap Value as TREQ"]
    PWM_WRAP10 = 0x2a,
    #[doc = "Select PWM Counter 1's Wrap Value as TREQ"]
    PWM_WRAP11 = 0x2b,
    #[doc = "Select I2C0's TX FIFO as TREQ"]
    I2C0_TX = 0x2c,
    #[doc = "Select I2C0's RX FIFO as TREQ"]
    I2C0_RX = 0x2d,
    #[doc = "Select I2C1's TX FIFO as TREQ"]
    I2C1_TX = 0x2e,
    #[doc = "Select I2C1's RX FIFO as TREQ"]
    I2C1_RX = 0x2f,
    #[doc = "Select the ADC as TREQ"]
    ADC = 0x30,
    #[doc = "Select the XIP Streaming FIFO as TREQ"]
    XIP_STREAM = 0x31,
    #[doc = "Select XIP_QMITX as TREQ"]
    XIP_QMITX = 0x32,
    #[doc = "Select XIP_QMIRX as TREQ"]
    XIP_QMIRX = 0x33,
    #[doc = "Select HSTX as TREQ"]
    HSTX = 0x34,
    #[doc = "Select CORESIGHT as TREQ"]
    CORESIGHT = 0x35,
    #[doc = "Select SHA256 as TREQ"]
    SHA256 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    #[doc = "Select Timer 0 as TREQ"]
    TIMER0 = 0x3b,
    #[doc = "Select Timer 1 as TREQ"]
    TIMER1 = 0x3c,
    #[doc = "Select Timer 2 as TREQ (Optional)"]
    TIMER2 = 0x3d,
    #[doc = "Select Timer 3 as TREQ (Optional)"]
    TIMER3 = 0x3e,
    #[doc = "Permanent request, for unpaced transfers."]
    PERMANENT = 0x3f,
}
impl TreqSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TreqSel {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TreqSel {
    #[inline(always)]
    fn from(val: u8) -> TreqSel {
        TreqSel::from_bits(val)
    }
}
impl From<TreqSel> for u8 {
    #[inline(always)]
    fn from(val: TreqSel) -> u8 {
        TreqSel::to_bits(val)
    }
}
