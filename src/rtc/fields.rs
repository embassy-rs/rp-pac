use crate::generic::*;
#[doc = "Interrupt Force"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Intf(u32);
impl Intf {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Intf {
        Intf(val)
    }
    pub const fn rtc(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_rtc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Inte(u32);
impl Inte {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Inte {
        Inte(val)
    }
    pub const fn rtc(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_rtc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "RTC setup register 1"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Setup1(u32);
impl Setup1 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Setup1 {
        Setup1(val)
    }
    #[doc = "Day of the week: 1-Monday...0-Sunday ISO 8601 mod 7"]
    pub const fn dotw(&self) -> u8 {
        let val = (self.0 >> 24u32) & 0x07;
        val as u8
    }
    #[doc = "Day of the week: 1-Monday...0-Sunday ISO 8601 mod 7"]
    pub fn set_dotw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24u32)) | (((val as u32) & 0x07) << 24u32);
    }
    #[doc = "Hours"]
    pub const fn hour(&self) -> u8 {
        let val = (self.0 >> 16u32) & 0x1f;
        val as u8
    }
    #[doc = "Hours"]
    pub fn set_hour(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16u32)) | (((val as u32) & 0x1f) << 16u32);
    }
    #[doc = "Minutes"]
    pub const fn min(&self) -> u8 {
        let val = (self.0 >> 8u32) & 0x3f;
        val as u8
    }
    #[doc = "Minutes"]
    pub fn set_min(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8u32)) | (((val as u32) & 0x3f) << 8u32);
    }
    #[doc = "Seconds"]
    pub const fn sec(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x3f;
        val as u8
    }
    #[doc = "Seconds"]
    pub fn set_sec(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0u32)) | (((val as u32) & 0x3f) << 0u32);
    }
}
#[doc = "RTC setup register 0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Setup0(u32);
impl Setup0 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Setup0 {
        Setup0(val)
    }
    #[doc = "Year"]
    pub const fn year(&self) -> u16 {
        let val = (self.0 >> 12u32) & 0x0fff;
        val as u16
    }
    #[doc = "Year"]
    pub fn set_year(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12u32)) | (((val as u32) & 0x0fff) << 12u32);
    }
    #[doc = "Month (1..12)"]
    pub const fn month(&self) -> u8 {
        let val = (self.0 >> 8u32) & 0x0f;
        val as u8
    }
    #[doc = "Month (1..12)"]
    pub fn set_month(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8u32)) | (((val as u32) & 0x0f) << 8u32);
    }
    #[doc = "Day of the month (1..31)"]
    pub const fn day(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x1f;
        val as u8
    }
    #[doc = "Day of the month (1..31)"]
    pub fn set_day(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val as u32) & 0x1f) << 0u32);
    }
}
#[doc = "Divider minus 1 for the 1 second counter. Safe to change the value when RTC is not enabled."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkdivM1(u32);
impl ClkdivM1 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> ClkdivM1 {
        ClkdivM1(val)
    }
    pub const fn clkdiv_m1(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    pub fn set_clkdiv_m1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
#[doc = "RTC register 0 Read this before RTC 1!"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Rtc0(u32);
impl Rtc0 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Rtc0 {
        Rtc0(val)
    }
    #[doc = "Day of the week"]
    pub const fn dotw(&self) -> u8 {
        let val = (self.0 >> 24u32) & 0x07;
        val as u8
    }
    #[doc = "Day of the week"]
    pub fn set_dotw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24u32)) | (((val as u32) & 0x07) << 24u32);
    }
    #[doc = "Hours"]
    pub const fn hour(&self) -> u8 {
        let val = (self.0 >> 16u32) & 0x1f;
        val as u8
    }
    #[doc = "Hours"]
    pub fn set_hour(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16u32)) | (((val as u32) & 0x1f) << 16u32);
    }
    #[doc = "Minutes"]
    pub const fn min(&self) -> u8 {
        let val = (self.0 >> 8u32) & 0x3f;
        val as u8
    }
    #[doc = "Minutes"]
    pub fn set_min(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8u32)) | (((val as u32) & 0x3f) << 8u32);
    }
    #[doc = "Seconds"]
    pub const fn sec(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x3f;
        val as u8
    }
    #[doc = "Seconds"]
    pub fn set_sec(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0u32)) | (((val as u32) & 0x3f) << 0u32);
    }
}
#[doc = "Interrupt setup register 0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IrqSetup0(u32);
impl IrqSetup0 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> IrqSetup0 {
        IrqSetup0(val)
    }
    pub const fn match_active(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_match_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    #[doc = "Global match enable. Don't change any other value while this one is enabled"]
    pub const fn match_ena(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    #[doc = "Global match enable. Don't change any other value while this one is enabled"]
    pub fn set_match_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    #[doc = "Enable year matching"]
    pub const fn year_ena(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "Enable year matching"]
    pub fn set_year_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "Enable month matching"]
    pub const fn month_ena(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    #[doc = "Enable month matching"]
    pub fn set_month_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    #[doc = "Enable day matching"]
    pub const fn day_ena(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "Enable day matching"]
    pub fn set_day_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "Year"]
    pub const fn year(&self) -> u16 {
        let val = (self.0 >> 12u32) & 0x0fff;
        val as u16
    }
    #[doc = "Year"]
    pub fn set_year(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12u32)) | (((val as u32) & 0x0fff) << 12u32);
    }
    #[doc = "Month (1..12)"]
    pub const fn month(&self) -> u8 {
        let val = (self.0 >> 8u32) & 0x0f;
        val as u8
    }
    #[doc = "Month (1..12)"]
    pub fn set_month(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8u32)) | (((val as u32) & 0x0f) << 8u32);
    }
    #[doc = "Day of the month (1..31)"]
    pub const fn day(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x1f;
        val as u8
    }
    #[doc = "Day of the month (1..31)"]
    pub fn set_day(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val as u32) & 0x1f) << 0u32);
    }
}
#[doc = "Interrupt setup register 1"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IrqSetup1(u32);
impl IrqSetup1 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> IrqSetup1 {
        IrqSetup1(val)
    }
    #[doc = "Enable day of the week matching"]
    pub const fn dotw_ena(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    #[doc = "Enable day of the week matching"]
    pub fn set_dotw_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    #[doc = "Enable hour matching"]
    pub const fn hour_ena(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    #[doc = "Enable hour matching"]
    pub fn set_hour_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    #[doc = "Enable minute matching"]
    pub const fn min_ena(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    #[doc = "Enable minute matching"]
    pub fn set_min_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    #[doc = "Enable second matching"]
    pub const fn sec_ena(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    #[doc = "Enable second matching"]
    pub fn set_sec_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    #[doc = "Day of the week"]
    pub const fn dotw(&self) -> u8 {
        let val = (self.0 >> 24u32) & 0x07;
        val as u8
    }
    #[doc = "Day of the week"]
    pub fn set_dotw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24u32)) | (((val as u32) & 0x07) << 24u32);
    }
    #[doc = "Hours"]
    pub const fn hour(&self) -> u8 {
        let val = (self.0 >> 16u32) & 0x1f;
        val as u8
    }
    #[doc = "Hours"]
    pub fn set_hour(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16u32)) | (((val as u32) & 0x1f) << 16u32);
    }
    #[doc = "Minutes"]
    pub const fn min(&self) -> u8 {
        let val = (self.0 >> 8u32) & 0x3f;
        val as u8
    }
    #[doc = "Minutes"]
    pub fn set_min(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8u32)) | (((val as u32) & 0x3f) << 8u32);
    }
    #[doc = "Seconds"]
    pub const fn sec(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x3f;
        val as u8
    }
    #[doc = "Seconds"]
    pub fn set_sec(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0u32)) | (((val as u32) & 0x3f) << 0u32);
    }
}
#[doc = "Interrupt status after masking & forcing"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ints(u32);
impl Ints {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Ints {
        Ints(val)
    }
    pub const fn rtc(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_rtc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "RTC register 1."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Rtc1(u32);
impl Rtc1 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Rtc1 {
        Rtc1(val)
    }
    #[doc = "Year"]
    pub const fn year(&self) -> u16 {
        let val = (self.0 >> 12u32) & 0x0fff;
        val as u16
    }
    #[doc = "Year"]
    pub fn set_year(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12u32)) | (((val as u32) & 0x0fff) << 12u32);
    }
    #[doc = "Month (1..12)"]
    pub const fn month(&self) -> u8 {
        let val = (self.0 >> 8u32) & 0x0f;
        val as u8
    }
    #[doc = "Month (1..12)"]
    pub fn set_month(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8u32)) | (((val as u32) & 0x0f) << 8u32);
    }
    #[doc = "Day of the month (1..31)"]
    pub const fn day(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x1f;
        val as u8
    }
    #[doc = "Day of the month (1..31)"]
    pub fn set_day(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val as u32) & 0x1f) << 0u32);
    }
}
#[doc = "Raw Interrupts"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Intr(u32);
impl Intr {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Intr {
        Intr(val)
    }
    pub const fn rtc(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_rtc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "RTC Control and status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ctrl(u32);
impl Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Ctrl {
        Ctrl(val)
    }
    #[doc = "If set, leapyear is forced off. Useful for years divisible by 100 but not by 400"]
    pub const fn force_notleapyear(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "If set, leapyear is forced off. Useful for years divisible by 100 but not by 400"]
    pub fn set_force_notleapyear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    #[doc = "Load RTC"]
    pub const fn load(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    #[doc = "Load RTC"]
    pub fn set_load(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    #[doc = "RTC enabled (running)"]
    pub const fn rtc_active(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "RTC enabled (running)"]
    pub fn set_rtc_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Enable RTC"]
    pub const fn rtc_enable(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Enable RTC"]
    pub fn set_rtc_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
