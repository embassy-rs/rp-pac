#[doc = "Divider minus 1 for the 1 second counter. Safe to change the value when RTC is not enabled."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkdivM1(pub u32);
impl ClkdivM1 {
    #[inline(always)]
    pub const fn clkdiv_m1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_clkdiv_m1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for ClkdivM1 {
    #[inline(always)]
    fn default() -> ClkdivM1 {
        ClkdivM1(0)
    }
}
#[doc = "RTC Control and status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Enable RTC"]
    #[inline(always)]
    pub const fn rtc_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable RTC"]
    #[inline(always)]
    pub fn set_rtc_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RTC enabled (running)"]
    #[inline(always)]
    pub const fn rtc_active(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RTC enabled (running)"]
    #[inline(always)]
    pub fn set_rtc_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Load RTC"]
    #[inline(always)]
    pub const fn load(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Load RTC"]
    #[inline(always)]
    pub fn set_load(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If set, leapyear is forced off. Useful for years divisible by 100 but not by 400"]
    #[inline(always)]
    pub const fn force_notleapyear(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "If set, leapyear is forced off. Useful for years divisible by 100 but not by 400"]
    #[inline(always)]
    pub fn set_force_notleapyear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
#[doc = "Interrupt Force"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Int(pub u32);
impl Int {
    #[inline(always)]
    pub const fn rtc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_rtc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Int {
    #[inline(always)]
    fn default() -> Int {
        Int(0)
    }
}
#[doc = "Interrupt setup register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IrqSetup0(pub u32);
impl IrqSetup0 {
    #[doc = "Day of the month (1..31)"]
    #[inline(always)]
    pub const fn day(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Day of the month (1..31)"]
    #[inline(always)]
    pub fn set_day(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Month (1..12)"]
    #[inline(always)]
    pub const fn month(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Month (1..12)"]
    #[inline(always)]
    pub fn set_month(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Year"]
    #[inline(always)]
    pub const fn year(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[doc = "Year"]
    #[inline(always)]
    pub fn set_year(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
    #[doc = "Enable day matching"]
    #[inline(always)]
    pub const fn day_ena(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enable day matching"]
    #[inline(always)]
    pub fn set_day_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Enable month matching"]
    #[inline(always)]
    pub const fn month_ena(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Enable month matching"]
    #[inline(always)]
    pub fn set_month_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Enable year matching"]
    #[inline(always)]
    pub const fn year_ena(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enable year matching"]
    #[inline(always)]
    pub fn set_year_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Global match enable. Don't change any other value while this one is enabled"]
    #[inline(always)]
    pub const fn match_ena(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Global match enable. Don't change any other value while this one is enabled"]
    #[inline(always)]
    pub fn set_match_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[inline(always)]
    pub const fn match_active(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_match_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for IrqSetup0 {
    #[inline(always)]
    fn default() -> IrqSetup0 {
        IrqSetup0(0)
    }
}
#[doc = "Interrupt setup register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IrqSetup1(pub u32);
impl IrqSetup1 {
    #[doc = "Seconds"]
    #[inline(always)]
    pub const fn sec(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Seconds"]
    #[inline(always)]
    pub fn set_sec(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Minutes"]
    #[inline(always)]
    pub const fn min(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Minutes"]
    #[inline(always)]
    pub fn set_min(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Hours"]
    #[inline(always)]
    pub const fn hour(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Hours"]
    #[inline(always)]
    pub fn set_hour(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Day of the week"]
    #[inline(always)]
    pub const fn dotw(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Day of the week"]
    #[inline(always)]
    pub fn set_dotw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "Enable second matching"]
    #[inline(always)]
    pub const fn sec_ena(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Enable second matching"]
    #[inline(always)]
    pub fn set_sec_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Enable minute matching"]
    #[inline(always)]
    pub const fn min_ena(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Enable minute matching"]
    #[inline(always)]
    pub fn set_min_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Enable hour matching"]
    #[inline(always)]
    pub const fn hour_ena(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enable hour matching"]
    #[inline(always)]
    pub fn set_hour_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Enable day of the week matching"]
    #[inline(always)]
    pub const fn dotw_ena(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable day of the week matching"]
    #[inline(always)]
    pub fn set_dotw_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IrqSetup1 {
    #[inline(always)]
    fn default() -> IrqSetup1 {
        IrqSetup1(0)
    }
}
#[doc = "RTC register 0 Read this before RTC 1!"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtc0(pub u32);
impl Rtc0 {
    #[doc = "Seconds"]
    #[inline(always)]
    pub const fn sec(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Seconds"]
    #[inline(always)]
    pub fn set_sec(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Minutes"]
    #[inline(always)]
    pub const fn min(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Minutes"]
    #[inline(always)]
    pub fn set_min(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Hours"]
    #[inline(always)]
    pub const fn hour(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Hours"]
    #[inline(always)]
    pub fn set_hour(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Day of the week"]
    #[inline(always)]
    pub const fn dotw(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Day of the week"]
    #[inline(always)]
    pub fn set_dotw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
}
impl Default for Rtc0 {
    #[inline(always)]
    fn default() -> Rtc0 {
        Rtc0(0)
    }
}
#[doc = "RTC register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtc1(pub u32);
impl Rtc1 {
    #[doc = "Day of the month (1..31)"]
    #[inline(always)]
    pub const fn day(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Day of the month (1..31)"]
    #[inline(always)]
    pub fn set_day(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Month (1..12)"]
    #[inline(always)]
    pub const fn month(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Month (1..12)"]
    #[inline(always)]
    pub fn set_month(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Year"]
    #[inline(always)]
    pub const fn year(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[doc = "Year"]
    #[inline(always)]
    pub fn set_year(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for Rtc1 {
    #[inline(always)]
    fn default() -> Rtc1 {
        Rtc1(0)
    }
}
#[doc = "RTC setup register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Setup0(pub u32);
impl Setup0 {
    #[doc = "Day of the month (1..31)"]
    #[inline(always)]
    pub const fn day(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Day of the month (1..31)"]
    #[inline(always)]
    pub fn set_day(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Month (1..12)"]
    #[inline(always)]
    pub const fn month(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Month (1..12)"]
    #[inline(always)]
    pub fn set_month(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Year"]
    #[inline(always)]
    pub const fn year(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[doc = "Year"]
    #[inline(always)]
    pub fn set_year(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for Setup0 {
    #[inline(always)]
    fn default() -> Setup0 {
        Setup0(0)
    }
}
#[doc = "RTC setup register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Setup1(pub u32);
impl Setup1 {
    #[doc = "Seconds"]
    #[inline(always)]
    pub const fn sec(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Seconds"]
    #[inline(always)]
    pub fn set_sec(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Minutes"]
    #[inline(always)]
    pub const fn min(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Minutes"]
    #[inline(always)]
    pub fn set_min(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Hours"]
    #[inline(always)]
    pub const fn hour(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Hours"]
    #[inline(always)]
    pub fn set_hour(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Day of the week: 1-Monday...0-Sunday ISO 8601 mod 7"]
    #[inline(always)]
    pub const fn dotw(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Day of the week: 1-Monday...0-Sunday ISO 8601 mod 7"]
    #[inline(always)]
    pub fn set_dotw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
}
impl Default for Setup1 {
    #[inline(always)]
    fn default() -> Setup1 {
        Setup1(0)
    }
}
