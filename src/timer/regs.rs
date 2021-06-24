#[doc = "Interrupt Force"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intf(pub u32);
impl Intf {
    pub const fn alarm_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    pub fn set_alarm_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    pub const fn alarm_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    pub fn set_alarm_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    pub const fn alarm_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    pub fn set_alarm_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    pub const fn alarm_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    pub fn set_alarm_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Intf {
    fn default() -> Intf {
        Intf(0)
    }
}
#[doc = "Indicates the armed/disarmed status of each alarm. A write to the corresponding ALARMx register arms the alarm. Alarms automatically disarm upon firing, but writing ones here will disarm immediately without waiting to fire."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Armed(pub u32);
impl Armed {
    pub const fn armed(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    pub fn set_armed(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Armed {
    fn default() -> Armed {
        Armed(0)
    }
}
#[doc = "Set high to pause the timer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pause(pub u32);
impl Pause {
    pub const fn pause(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    pub fn set_pause(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Pause {
    fn default() -> Pause {
        Pause(0)
    }
}
#[doc = "Interrupt status after masking & forcing"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ints(pub u32);
impl Ints {
    pub const fn alarm_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    pub fn set_alarm_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    pub const fn alarm_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    pub fn set_alarm_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    pub const fn alarm_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    pub fn set_alarm_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    pub const fn alarm_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    pub fn set_alarm_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Ints {
    fn default() -> Ints {
        Ints(0)
    }
}
#[doc = "Set bits high to enable pause when the corresponding debug ports are active"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgpause(pub u32);
impl Dbgpause {
    #[doc = "Pause when processor 0 is in debug mode"]
    pub const fn dbg0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Pause when processor 0 is in debug mode"]
    pub fn set_dbg0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Pause when processor 1 is in debug mode"]
    pub const fn dbg1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Pause when processor 1 is in debug mode"]
    pub fn set_dbg1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Dbgpause {
    fn default() -> Dbgpause {
        Dbgpause(0)
    }
}
#[doc = "Raw Interrupts"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    pub const fn alarm_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    pub fn set_alarm_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    pub const fn alarm_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    pub fn set_alarm_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    pub const fn alarm_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    pub fn set_alarm_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    pub const fn alarm_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    pub fn set_alarm_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Intr {
    fn default() -> Intr {
        Intr(0)
    }
}
#[doc = "Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inte(pub u32);
impl Inte {
    pub const fn alarm_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    pub fn set_alarm_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    pub const fn alarm_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    pub fn set_alarm_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    pub const fn alarm_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    pub fn set_alarm_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    pub const fn alarm_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    pub fn set_alarm_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Inte {
    fn default() -> Inte {
        Inte(0)
    }
}
