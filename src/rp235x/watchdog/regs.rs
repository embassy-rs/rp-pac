#[doc = "Watchdog control The rst_wdsel register determines which subsystems are reset when the watchdog is triggered. The watchdog can be triggered in software."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Indicates the time in usec before a watchdog reset will be triggered"]
    #[inline(always)]
    pub const fn time(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Indicates the time in usec before a watchdog reset will be triggered"]
    #[inline(always)]
    pub fn set_time(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Pause the watchdog timer when JTAG is accessing the bus fabric"]
    #[inline(always)]
    pub const fn pause_jtag(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Pause the watchdog timer when JTAG is accessing the bus fabric"]
    #[inline(always)]
    pub fn set_pause_jtag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Pause the watchdog timer when processor 0 is in debug mode"]
    #[inline(always)]
    pub const fn pause_dbg0(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Pause the watchdog timer when processor 0 is in debug mode"]
    #[inline(always)]
    pub fn set_pause_dbg0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Pause the watchdog timer when processor 1 is in debug mode"]
    #[inline(always)]
    pub const fn pause_dbg1(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Pause the watchdog timer when processor 1 is in debug mode"]
    #[inline(always)]
    pub fn set_pause_dbg1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "When not enabled the watchdog timer is paused"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "When not enabled the watchdog timer is paused"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Trigger a watchdog reset"]
    #[inline(always)]
    pub const fn trigger(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger a watchdog reset"]
    #[inline(always)]
    pub fn set_trigger(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("time", &self.time())
            .field("pause_jtag", &self.pause_jtag())
            .field("pause_dbg0", &self.pause_dbg0())
            .field("pause_dbg1", &self.pause_dbg1())
            .field("enable", &self.enable())
            .field("trigger", &self.trigger())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl {
            time: u32,
            pause_jtag: bool,
            pause_dbg0: bool,
            pause_dbg1: bool,
            enable: bool,
            trigger: bool,
        }
        let proxy = Ctrl {
            time: self.time(),
            pause_jtag: self.pause_jtag(),
            pause_dbg0: self.pause_dbg0(),
            pause_dbg1: self.pause_dbg1(),
            enable: self.enable(),
            trigger: self.trigger(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Load the watchdog timer. The maximum setting is 0xffffff which corresponds to approximately 16 seconds."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Load(pub u32);
impl Load {
    #[inline(always)]
    pub const fn load(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_load(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Load {
    #[inline(always)]
    fn default() -> Load {
        Load(0)
    }
}
impl core::fmt::Debug for Load {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Load").field("load", &self.load()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Load {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Load {
            load: u32,
        }
        let proxy = Load { load: self.load() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Logs the reason for the last reset. Both bits are zero for the case of a hardware reset. Additionally, as of RP2350, a debugger warm reset of either core (SYSRESETREQ or hartreset) will also clear the watchdog reason register, so that software loaded under the debugger following a watchdog timeout will not continue to see the timeout condition."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reason(pub u32);
impl Reason {
    #[inline(always)]
    pub const fn timer(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_timer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn force(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_force(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Reason {
    #[inline(always)]
    fn default() -> Reason {
        Reason(0)
    }
}
impl core::fmt::Debug for Reason {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reason")
            .field("timer", &self.timer())
            .field("force", &self.force())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reason {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Reason {
            timer: bool,
            force: bool,
        }
        let proxy = Reason {
            timer: self.timer(),
            force: self.force(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
